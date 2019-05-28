#![no_main]
#![no_std]

use panic_semihosting as _;

use stm32h7x3_hal::{
    prelude::*,
    stm32h7x3,
    delay::Delay
};
use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let p = stm32h7x3::Peripherals::take().unwrap();

    let mut flash = p.FLASH.constrain();
    let mut rcc = p.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let gpiob = p.GPIOB.split(&mut rcc.ahb4);
    // Configure our LED as output
    let mut led = gpiob.pb0.into_push_pull_output();
    let mut delay = Delay::new(cp.SYST, clocks);

    // Set the LED high, wait 500 ms and set low again for another 500 ms
    // -> blinking every half second
    loop {
        led.set_high();
        delay.delay_ms(500u32);
        led.set_low();
        delay.delay_ms(500u32);
    }
}