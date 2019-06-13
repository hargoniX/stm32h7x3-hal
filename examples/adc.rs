#![deny(unsafe_code)]
#![no_main]
#![no_std]

use panic_semihosting as _;

use stm32h7x3_hal::{
    adc,
    stm32h7x3,
    delay::Delay,
    prelude::*,
};
use cortex_m_rt::entry;

use embedded_hal::adc::OneShot;

use cortex_m_semihosting::hprintln;

#[entry]
fn main() -> ! {
    // Aquire peripherals
    let cp = cortex_m::Peripherals::take().unwrap();
    let p = stm32h7x3::Peripherals::take().unwrap();

    let mut flash = p.FLASH.constrain();
    let mut rcc = p.RCC.constrain();

    // Configure ADC clocks
    // Default value is the slowest possible ADC clock: PCLK2 / 8. Meanwhile ADC
    // clock is configurable. So its frequency may be tweaked to meet certain
    // practical needs. User specified value is be approximated using supported
    // prescaler values 2/4/6/8.
    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    let mut delay = Delay::new(cp.SYST, clocks);

    // Setup ADC
    let mut adc1 = adc::Adc::adc1(p.ADC1, &mut rcc.ahb1, &mut delay);

    // Setup GPIOB
    let gpiob = p.GPIOB.split(&mut rcc.ahb4);

    // Configure pb0, pb1 as an analog input
    let mut ch0 = gpiob.pb1.into_analog();

    loop {
        let data: u32 = adc1.read(&mut ch0).unwrap();
        hprintln!("adc1: {}", data).unwrap();
    }
}