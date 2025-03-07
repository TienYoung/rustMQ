extern crate libc;
use std::io;

fn main() {
    unsafe {
        let message_key = libc::ftok(".".as_ptr() as *const i8, 'M' as i32);
        if message_key == -1 {
            let errno = *libc::__error();
            let error_message = io::Error::from_raw_os_error(errno).to_string();
            eprint!("ftok failed: {}", error_message);
        }
        else {
            println!("message_key: {}", message_key);
        }
    }
}
