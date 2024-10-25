use std::ffi::{c_void, CString};
use libc::{c_char, c_int, open, pid_t, ssize_t, write, O_WRONLY, O_CREAT};

fn main() {
    unsafe {
        let path = CString::new("my_file").unwrap();
        let file_desc: c_int = open(path.as_ptr() as *const c_char, O_WRONLY | O_CREAT);
        println!("{}", file_desc);
        let pid: pid_t = libc::fork();

        if pid < 0 {
            panic!("fork failed\n");
        } else if pid == 0 {
            let msg = "hi this is the child!\n";
            let result: ssize_t = write(file_desc, msg.as_ptr() as *const c_void, 22);
            if result != 22 {
                println!("{}", result);
                panic!("unable to write child message")
            }
            println!("wrote child message");
        } else {
            let msg = "hi this is the parent!\n";
            let result: ssize_t = write(file_desc, msg.as_ptr() as *const c_void, 23);
            if result != 23 {
                println!("{}", result);
                panic!("unable to write parent message")
            }
            println!("wrote parent message");
        }
    }
}
