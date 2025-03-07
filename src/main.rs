extern crate libc;
use std::{io, process};
use std::ffi::CString;

fn handle_error(message: &str) {
    #[cfg(target_os = "macos")]
    let errno = unsafe{ *libc::__error() };
    #[cfg(not(target_os = "macos"))]
    let errno: i32 = unsafe { *libc::__errno_location() };
    let error_message: String = io::Error::from_raw_os_error(errno).to_string();
    eprintln!("{}: {}", message, error_message);
}

fn main() {
    unsafe {
        let path = CString::new(".").expect("CString::new failed");
        let message_key = libc::ftok(path.as_ptr() as *const i8, 'M' as i32);
        if message_key == -1 {
            handle_error("Cannot create key");
            process::exit(1);
        }

        let message_id = libc::msgget(message_key, libc::IPC_CREAT | 0o660);
        if message_id == -1 {
            handle_error("Cannot create message queue");
            process::exit(2);
        }
    }
}
