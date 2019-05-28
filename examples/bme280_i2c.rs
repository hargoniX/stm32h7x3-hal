#![no_main]
#![no_std]

use panic_semihosting as _;

use stm32h7x3_hal::{
    prelude::*,
    stm32h7x3,
    delay::Delay,
    i2c::I2c,
};

use cortex_m_rt::entry;

use cortex_m_semihosting::hprintln;

use bme280::BME280;

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let p = stm32h7x3::Peripherals::take().unwrap();

    let mut rcc = p.RCC.constrain();
    let mut flash = p.FLASH.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    let gpiob = p.GPIOB.split(&mut rcc.ahb4);

    // Configure the SCL and the SDA pin for our I2C bus
    let scl = gpiob.pb8.into_alternate_af4().set_open_drain();
    let sda = gpiob.pb9.into_alternate_af4().set_open_drain();

    let i2c_bus = I2c::i2c1(
        p.I2C1,
        (scl, sda),
        100.khz(),
        clocks,
        &mut rcc.apb1l,
    );

    // instantiate and initialize the bme280 chip
    let mut bme280 = BME280::new_primary(i2c_bus, Delay::new(cp.SYST, clocks));
    bme280.init().unwrap();

    // Fetch and print the measurements of the bme280 forever
    loop {
        let measurements = bme280.measure().unwrap();

        hprintln!("Relative Humidity = {}%", measurements.humidity).unwrap();
        hprintln!("Temperature = {} deg C", measurements.temperature).unwrap();
        hprintln!("Pressure = {} pascals", measurements.pressure).unwrap();
    }
}