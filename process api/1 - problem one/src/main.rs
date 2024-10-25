use libc;
use libc::{pid_t, wait};

fn main() {
    unsafe {
        let mut x: i64 = 100;
        let pid: pid_t = libc::fork();

        if pid < 0 {
            panic!("fork failed\n")
        } else if pid == 0 {
            println!("child: {}", x);
            x = 500;
            println!("child: {}", x)
        } else {
            let status = 0 as *mut i32;
            let _ = wait(status);
            println!("parent: {}", x);
        }
    }
}
