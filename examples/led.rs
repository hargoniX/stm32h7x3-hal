#![no_main]
#![no_std]

use panic_semihosting as _;

use stm32h7x3_hal::{
    prelude::*,
    stm32h7x3
};
use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    let p = stm32h7x3::Peripherals::take().unwrap();

    let mut rcc = p.RCC.constrain();
    let gpiob = p.GPIOB.split(&mut rcc.ahb4);

    // set the LED on the nucleo board as output
    let mut led = gpiob.pb0.into_push_pull_output();

    // enable the LED and wait forever
    led.set_high();
    loop {}
}