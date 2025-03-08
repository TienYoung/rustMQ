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

// define a message body
struct Message {
    msg_type: libc::c_long,
    msg_data: [libc::c_char; 64],
}

fn main() {
    unsafe {
        let path = CString::new(".").expect("CString::new failed");
        let message_key: libc::key_t = libc::ftok(path.as_ptr() as *const i8, 'M' as i32);
        if message_key == -1 {
            eprintln!("Cannot create key: {}", errno());
            process::exit(1);
        }

        let message_queue_id: libc::c_int = libc::msgget(message_key, libc::IPC_CREAT | 0o0660);
        if message_queue_id == -1 {
            eprintln!("Cannot create message queue: {}", errno());
            process::exit(2);
        }

        println!("Message queue ID: {}\n", message_queue_id);

        let size_of_data = std::mem::size_of::<Message>() - std::mem::size_of::<libc::c_long>();

        while true {
            let mut message = Message {
                msg_type: 0,
                msg_data: [0; 64],
            };

            let a= libc::msgrcv(message_queue_id, message.msg_data.as_mut_ptr() as *mut libc::c_void, size_of_data, 1, 0);

        }
    }
}
