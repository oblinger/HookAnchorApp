mod popup;
mod cmd;

use anchor_selector::*;

fn main() -> Result<(), eframe::Error> {
    // Handle command-line operations first
    if cmd::handle_command_line().is_some() {
        return Ok(());
    }
    
    // Otherwise run GUI mode
    popup::run_gui()
}