//! Subprocess management and monitoring
//! 
//! This module tracks spawned subprocesses to detect hangs, monitor completion,
//! and manage the lifecycle of child processes created by the system.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use std::process::Child;
use std::thread;

/// Global process tracker
static PROCESS_TRACKER: std::sync::OnceLock<Arc<Mutex<ProcessTracker>>> = std::sync::OnceLock::new();

#[derive(Debug, Clone)]
pub struct ProcessInfo {
    pub command: String,
    pub start_time: Instant,
    pub pid: Option<u32>,
    pub completed: bool,
}

pub(crate) struct ProcessTracker {
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
            super::logging::detailed_log("SUBPROCESS", &format!("Monitoring process {} (PID: {}) - command: '{}'", id, pid, command));
            
            match child.wait() {
                Ok(status) => {
                    let duration = start_time.elapsed();
                    let status_msg = if status.success() {
                        "SUCCESS"
                    } else {
                        "FAILED"
                    };
                    super::logging::detailed_log("SUBPROCESS", &format!("Process {} completed: {} in {:?}", id, status_msg, duration));
                    
                    // Mark as completed
                    if let Ok(mut tracker) = tracker.lock() {
                        if let Some(info) = tracker.processes.get_mut(&id) {
                            info.completed = true;
                        }
                    }
                }
                Err(e) => {
                    let duration = start_time.elapsed();
                    super::logging::detailed_log("SUBPROCESS", &format!("Process {} wait failed after {:?}: {}", id, duration, e));
                }
            }
        });
        
        super::logging::detailed_log("SUBPROCESS", &format!("Registered process {} (PID: {}) for monitoring", id, pid));
        id
    }

    /// Get status of all processes
    pub fn get_status(&self) -> Vec<(u32, ProcessInfo)> {
        self.processes.iter().map(|(id, info)| (*id, info.clone())).collect()
    }

    /// Clean up completed processes older than threshold
    pub fn cleanup_old_processes(&mut self, max_age: Duration) {
        let now = Instant::now();
        self.processes.retain(|id, info| {
            if info.completed && now.duration_since(info.start_time) > max_age {
                super::logging::detailed_log("SUBPROCESS", &format!("Cleaned up completed process {}", id));
                false
            } else {
                true
            }
        });
    }
}

/// Get the global process tracker instance
fn get_process_tracker() -> Arc<Mutex<ProcessTracker>> {
    PROCESS_TRACKER.get_or_init(|| {
        Arc::new(Mutex::new(ProcessTracker::new()))
    }).clone()
}

/// Register a subprocess for monitoring
pub fn register_process(child: Child, command: String) -> u32 {
    let tracker = get_process_tracker();
    let result = match tracker.lock() {
        Ok(mut tracker_guard) => tracker_guard.register_process(child, command),
        Err(_) => {
            super::logging::log_error("Failed to acquire process tracker lock");
            0
        }
    };
    result
}

/// Check system health by reporting on running/hung processes
pub fn check_system_health() {
    let tracker = get_process_tracker();
    let processes = {
        match tracker.lock() {
            Ok(mut tracker_guard) => {
                // Clean up old completed processes (> 5 minutes old)
                tracker_guard.cleanup_old_processes(Duration::from_secs(300));
                tracker_guard.get_status()
            },
            Err(_) => {
                super::logging::log_error("Failed to acquire process tracker lock");
                return;
            }
        }
    };
    
    let now = Instant::now();
    
    super::logging::log("=== System Health Check ===");
    super::logging::log(&format!("Total tracked processes: {}", processes.len()));
    
    for (id, info) in processes {
        let age = now.duration_since(info.start_time);
        let status = if info.completed { "Completed" } else { "Running" };
        
        super::logging::log(&format!("Process {}: {} (Age: {:?}, Status: {})", 
            id, info.command, age, status));
        
        // Warn about potentially hung processes (> 30 seconds and still running)
        if !info.completed && age > Duration::from_secs(30) {
            super::logging::log(&format!("⚠️  WARNING: Process {} may be hung (running for {:?})", id, age));
        }
    }
    super::logging::log("=========================");
}

/// Show current process status (for debugging)
pub fn show_process_status() {
    let tracker = get_process_tracker();
    let processes = match tracker.lock() {
        Ok(tracker_guard) => tracker_guard.get_status(),
        Err(_) => {
            println!("Failed to acquire process tracker lock");
            return;
        }
    };
    
    println!("\n=== Process Monitor Status ===");
    println!("Total tracked processes: {}", processes.len());
    
    let now = Instant::now();
    for (id, info) in processes {
        let age = now.duration_since(info.start_time);
        let status = if info.completed { "✓ Completed" } else { "⏳ Running" };
        
        println!("[{}] {} - {} (Age: {:.1}s)", 
            id, status, info.command, age.as_secs_f32());
        
        if let Some(pid) = info.pid {
            println!("    PID: {}", pid);
        }
    }
    println!("=============================\n");
}