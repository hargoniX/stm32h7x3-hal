#![no_std]

extern crate cast;
extern crate cortex_m;
extern crate embedded_hal as hal;
extern crate nb;
pub extern crate stm32h7;
extern crate void;

pub mod gpio;
pub mod flash;
pub mod i2c;
pub mod rcc;
pub mod time;
pub mod delay;
pub use clock_calc_macro::calc_config;