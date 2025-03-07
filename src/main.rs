extern crate libc;
use std::io;

fn main() {
    unsafe {
        let message_key = libc::ftok(".".as_ptr() as *const i8, 'M' as i32);
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
