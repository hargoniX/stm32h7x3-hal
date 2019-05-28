#![no_main]
#![no_std]

use panic_semihosting as _;
use stm32h7x3_hal as _;
use cortex_m_semihosting::hprintln;

use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    hprintln!("Hello World").unwrap();
    loop {}
}