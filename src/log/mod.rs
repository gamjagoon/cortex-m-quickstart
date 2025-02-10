use core::ptr;

#[repr(u8)]
#[derive(PartialEq, PartialOrd)]
#[allow(dead_code)]
pub enum LogLevel {
    None,
    Error,
    Notice,
    Info,
}

const LOG_START: *mut u8 = 0x28100000 as *mut u8;
const LOG_END: *mut u8 = 0x28200000 as *mut u8;
const LOG_LEVEL: LogLevel = LogLevel::Info;

static mut LOG_CURRENT: *mut u8 = LOG_START;

pub fn print_error(msg: &str) {
    if LOG_LEVEL >= LogLevel::Error {
        write_log(msg);
    }
}
#[allow(dead_code)]
pub fn print_notice(msg: &str) {
    if LOG_LEVEL >= LogLevel::Notice {
        write_log(msg);
    }
}

pub fn print_info(msg: &str) {
    if LOG_LEVEL >= LogLevel::Info {
        write_log(msg);
    }
}

fn write_log(msg: &str) {
    let msg_bytes = msg.as_bytes();
    unsafe {
        for byte in msg_bytes {
            if LOG_CURRENT >= LOG_END {
                LOG_CURRENT = LOG_START;
            }

            ptr::write_volatile(LOG_CURRENT, *byte);
            LOG_CURRENT = LOG_CURRENT.add(1);
        }

        if LOG_CURRENT >= LOG_END {
            LOG_CURRENT = LOG_START;
        }
        ptr::write_volatile(LOG_CURRENT, b'\n');
        LOG_CURRENT = LOG_CURRENT.add(1);
    }
}
