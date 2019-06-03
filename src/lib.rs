#![no_std]

extern crate cast;
extern crate cortex_m;
extern crate embedded_hal as hal;
extern crate nb;
extern crate void;

pub mod gpio;
pub mod flash;
pub mod i2c;
pub mod rcc;
pub mod time;
pub mod delay;
pub mod watchdog;
pub mod prelude;
pub mod serial;
pub use stm32h7::stm32h7x3;
pub use stm32h7::stm32h7x3 as pac;
pub use compile_time_calculations::*;
