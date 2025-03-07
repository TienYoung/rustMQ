extern crate libc;
use std::io;
use std::ffi::CString;

fn main() {
    unsafe {
        let path = CString::new(".").expect("CString::new failed");
        let message_key = libc::ftok(path.as_ptr() as *const i8, 'M' as i32);
        if message_key == -1 {
            #[cfg(target_os = "macos")]
            let errno = *libc::__error();
            #[cfg(not(target_os = "macos"))]
            let errno = *libc::__errno_location();
            let error_message = io::Error::from_raw_os_error(errno).to_string();
            eprintln!("ftok failed: {}", error_message);
        }
        else {
            println!("message_key: {}", message_key);
        }
    }
}
