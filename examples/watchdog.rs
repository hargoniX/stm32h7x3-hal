#![no_main]
#![no_std]

use panic_semihosting as _;

use stm32h7x3_hal::{
    prelude::*,
    stm32h7x3,
    watchdog::SystemWindowWatchdog,
};
use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;

#[entry]
fn main() -> ! {
    let p = stm32h7x3::Peripherals::take().unwrap();

    let mut flash = p.FLASH.constrain();
    let mut rcc = p.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut watchdog = SystemWindowWatchdog::new(
        p.WWDG,
        clocks,
        &mut rcc.apb3
    );

    // If the watchdog is working correctly this print should
    // appear again and again as the chip gets restarted
    hprintln!("Watchdog restarted").unwrap();

    // Enable the watchdog with a limit of 100 ms and wait forever
    // -> restart the chip
    watchdog.start(100.ms());

    loop {}

}