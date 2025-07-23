//! Process monitoring for non-blocking command execution
//! 
//! This module tracks spawned processes to detect hangs and orphaned processes

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use std::process::Child;
use std::thread;

/// Global process tracker
static PROCESS_TRACKER: std::sync::OnceLock<Arc<Mutex<ProcessTracker>>> = std::sync::OnceLock::new();

#[derive(Debug)]
pub struct ProcessInfo {
    pub command: String,
    pub start_time: Instant,
    pub pid: Option<u32>,
    pub completed: bool,
}

pub struct ProcessTracker {
    processes: HashMap<u32, ProcessInfo>,
    next_id: u32,
}

impl ProcessTracker {
    fn new() -> Self {
        Self {
            processes: HashMap::new(),
            next_id: 1,
        }
    }

    /// Register a new spawned process
    pub fn register_process(&mut self, mut child: Child, command: String) -> u32 {
        let id = self.next_id;
        self.next_id += 1;
        
        let pid = child.id();
        let info = ProcessInfo {
            command: command.clone(),
            start_time: Instant::now(),
            pid: Some(pid),
            completed: false,
        };
        
        self.processes.insert(id, info);
        
        // Spawn a monitor thread for this process
        let tracker = get_process_tracker();
        thread::spawn(move || {
            let start_time = Instant::now();
            crate::utils::debug_log("PROCESS_MONITOR", &format!("Monitoring process {} (PID: {}) - command: '{}'", id, pid, command));
            
            match child.wait() {
                Ok(status) => {
                    let duration = start_time.elapsed();
                    let status_msg = if status.success() {
                        "SUCCESS"
                    } else {
                        "FAILED"
                    };
                    crate::utils::debug_log("PROCESS_MONITOR", &format!("Process {} completed: {} in {:?}", id, status_msg, duration));
                    
                    // Mark as completed
                    if let Ok(mut tracker) = tracker.lock() {
                        if let Some(info) = tracker.processes.get_mut(&id) {
                            info.completed = true;
                        }
                    }
                }
                Err(e) => {
                    let duration = start_time.elapsed();
                    crate::utils::debug_log("PROCESS_MONITOR", &format!("Process {} wait failed after {:?}: {}", id, duration, e));
                }
            }
        });
        
        crate::utils::debug_log("PROCESS_MONITOR", &format!("Registered process {} (PID: {}) for monitoring", id, pid));
        id
    }

    /// Get status of all processes
    pub fn get_status(&self) -> Vec<(u32, &ProcessInfo)> {
        self.processes.iter().map(|(id, info)| (*id, info)).collect()
    }

    /// Clean up completed processes older than threshold
    pub fn cleanup_old_processes(&mut self, max_age: Duration) {
        let now = Instant::now();
        self.processes.retain(|id, info| {
            if info.completed && now.duration_since(info.start_time) > max_age {
                crate::utils::debug_log("PROCESS_MONITOR", &format!("Cleaned up completed process {}", id));
                false
            } else {
                true
            }
        });
    }

    /// Check for potentially hung processes
    pub fn check_for_hung_processes(&self, hang_threshold: Duration) -> Vec<(u32, &ProcessInfo)> {
        let now = Instant::now();
        self.processes
            .iter()
            .filter(|(_, info)| {
                !info.completed && now.duration_since(info.start_time) > hang_threshold
            })
            .map(|(id, info)| (*id, info))
            .collect()
    }
}

/// Get the global process tracker
pub fn get_process_tracker() -> Arc<Mutex<ProcessTracker>> {
    PROCESS_TRACKER.get_or_init(|| Arc::new(Mutex::new(ProcessTracker::new()))).clone()
}

/// Register a spawned process for monitoring
pub fn register_process(child: Child, command: String) -> u32 {
    let tracker = get_process_tracker();
    let result = match tracker.lock() {
        Ok(mut tracker) => tracker.register_process(child, command),
        Err(_) => {
            crate::utils::debug_log("PROCESS_MONITOR", "Failed to acquire tracker lock for registration");
            0
        }
    };
    result
}

/// Check system health - look for hung processes
pub fn check_system_health() {
    let tracker = get_process_tracker();
    let _result = match tracker.lock() {
        Ok(mut tracker) => {
            // Clean up old completed processes (older than 5 minutes)
            tracker.cleanup_old_processes(Duration::from_secs(300));
            
            // Check for processes that might be hung (running longer than 2 minutes)
            let hung_processes = tracker.check_for_hung_processes(Duration::from_secs(120));
            
            if !hung_processes.is_empty() {
                crate::utils::debug_log("PROCESS_MONITOR", &format!("⚠️ Found {} potentially hung processes:", hung_processes.len()));
                for (id, info) in hung_processes {
                    let duration = info.start_time.elapsed();
                    crate::utils::debug_log("PROCESS_MONITOR", &format!("  Process {} (PID: {:?}) running for {:?} - command: '{}'", 
                        id, info.pid, duration, info.command));
                }
            }
            
            // Log current status
            let total = tracker.processes.len();
            let completed = tracker.processes.values().filter(|info| info.completed).count();
            let running = total - completed;
            
            if total > 0 {
                crate::utils::debug_log("PROCESS_MONITOR", &format!("System health: {} total processes ({} running, {} completed)", total, running, completed));
            }
            ()
        },
        Err(_) => {
            crate::utils::debug_log("PROCESS_MONITOR", "Failed to acquire tracker lock for health check");
        }
    };
}

/// Show detailed process status (for debugging)
pub fn show_process_status() {
    let tracker = get_process_tracker();
    let _result = match tracker.lock() {
        Ok(tracker) => {
            let statuses = tracker.get_status();
            
            if statuses.is_empty() {
                crate::utils::debug_log("PROCESS_MONITOR", "No tracked processes");
                return ();
            }
            
            crate::utils::debug_log("PROCESS_MONITOR", &format!("=== PROCESS STATUS ({} processes) ===", statuses.len()));
            for (id, info) in statuses {
                let duration = info.start_time.elapsed();
                let status = if info.completed { "COMPLETED" } else { "RUNNING" };
                crate::utils::debug_log("PROCESS_MONITOR", &format!("Process {}: {} for {:?} - PID: {:?} - command: '{}'", 
                    id, status, duration, info.pid, info.command));
            }
            ()
        },
        Err(_) => {
            crate::utils::debug_log("PROCESS_MONITOR", "Failed to acquire tracker lock for status display");
        }
    };
}