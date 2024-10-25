use std::sync::Arc;
use std::io::Error;
use std::sync::atomic::{AtomicBool, Ordering};
use libc;

fn main() -> Result<(), Error> {
    unsafe {
        let pid: libc::pid_t = libc::fork();

        if pid < 0 {
            panic!("fork failed\n")
        } else if pid == 0 {
            println!("hello");
            Ok(())
        } else {
            // https://docs.rs/signal-hook/0.3.17/signal_hook/#examples
            let term = Arc::new(AtomicBool::new(false));
            signal_hook::flag::register(signal_hook::consts::SIGCHLD, Arc::clone(&term))?;
            // Busy waiting, this could use improvement, perhaps
            // switch to nix crate for proper signal handler
            while !term.load(Ordering::Relaxed) {
            }
            println!("goodbye");
            Ok(())
        }
    }
}
