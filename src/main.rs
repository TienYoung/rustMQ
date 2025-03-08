extern crate libc;
use std::{io, process};
use std::ffi::CString;

#[cfg(target_os = "macos")]
fn errno() -> io::Error {
    io::Error::from_raw_os_error(unsafe { *libc::__error() })
}

#[cfg(target_os = "linux")]
fn errno() -> io::Error {
    io::Error::from_raw_os_error(unsafe { *libc::__errno_location() })
}

fn main() {
    unsafe {
        let path = CString::new(".").expect("CString::new failed");
        let message_key: libc::key_t = libc::ftok(path.as_ptr() as *const i8, 'M' as i32);
        if message_key == -1 {
            eprintln!("Cannot create key: {}", errno());
            process::exit(1);
        }

        let message_qeueue_id: libc::c_int = libc::msgget(message_key, libc::IPC_CREAT | 0o660);
        if message_qeueue_id == -1 {
            eprintln!("Cannot create message queue: {}", errno());
            process::exit(2);
        }

        println!("Message queue ID: {}\n", message_qeueue_id);

        
    }
}
