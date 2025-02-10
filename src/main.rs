#![no_main]
#![no_std]

mod log;

use cortex_m_rt::entry;
use cortex_m_semihosting::{debug, hprintln};
use panic_halt as _;

#[entry]
fn main() -> ! {
    log::print_info("fn main");

    run_time();

    hprintln!("End Main");
    debug::exit(debug::EXIT_SUCCESS);

    loop {}
}

#[inline(never)]
#[no_mangle]
fn run_time() {
    log::print_info("fn run_time");

    let pw = b"abcdefg123456789";
    let pw2 = b"abcdefg12";

    if let Err(_e) = input_pw(pw) {
        log::print_error("pw error");
        unsafe {
            core::arch::asm!("b .");
        }
    }

    if let Err(_e) = input_pw(pw2) {
        log::print_error("pw2 error");
        unsafe {
            core::arch::asm!("b .");
        }
    }
}

#[inline(never)]
#[no_mangle]
fn input_pw(pw: &[u8]) -> Result<(), u32> {
    log::print_info("fn input_pw");
    if pw.len() < 10 {
        log::print_error("Error");
        return Err(0xA0000001);
    }
    let pw2 = b"abcdefg123456789";

    for (idx, c) in pw.iter().enumerate() {
        if *c != pw2[idx] {
            log::print_error("pw invalid");
            return Err(0xA0000002);
        }
    }
    Ok(())
}
