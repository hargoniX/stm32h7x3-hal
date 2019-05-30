//! Serial peripheral interface implementation

use hal::spi::{FullDuplex, Mode, Phase, Polarity};
use stm32h7::stm32h7x3::{SPI1, SPI2, SPI3, SPI4, SPI5, SPI6};
use crate::gpio::gpioa::{PA5, PA7, PA9, PA12};
use crate::gpio::gpiob::{PB2, PB3, PB5, PB10, PB13, PB15};
use crate::gpio::gpioc::{PC1, PC3, PC10, PC12};
use crate::gpio::gpiod::{PD3, PD6, PD7};
use crate::gpio::gpioe::{PE2, PE6, PE12, PE14};
use crate::gpio::gpiof::{PF7, PF9, PF11};
use crate::gpio::gpiog::{PG11, PG13, PG14};
use crate::gpio::gpioh::{PH6};
use crate::gpio::gpioi::{PI1, PI3};
use crate::gpio::gpioj::{PJ10};
use crate::gpio::gpiok::{PK0};
use crate::gpio::{AF5, AF6, AF7, AF8, Output, PushPull};

/// SCK pin -- DO NOT IMPLEMENT THIS TRAIT
pub unsafe trait SckPin<SPI> {}

/// MISO pin -- DO NOT IMPLEMENT THIS TRAIT
pub unsafe trait MisoPin<SPI> {}

/// MOSI pin -- DO NOT IMPLEMENT THIS TRAIT
pub unsafe trait MosiPin<SPI> {}

// All SCK pins for SPI1
unsafe impl SckPin<SPI1> for PA5<Output<PushPull>, AF5> {}
unsafe impl SckPin<SPI1> for PB3<Output<PushPull>, AF5> {}
unsafe impl SckPin<SPI1> for PG11<Output<PushPull>, AF5> {}

// All SCK pins for SPI2
unsafe impl SckPin<SPI2> for PA9<Output<PushPull>, AF5> {}
unsafe impl SckPin<SPI2> for PA12<Output<PushPull>, AF5> {}
unsafe impl SckPin<SPI2> for PB10<Output<PushPull>, AF5> {}
unsafe impl SckPin<SPI2> for PB13<Output<PushPull>, AF5> {}
unsafe impl SckPin<SPI2> for PD3<Output<PushPull>, AF5> {}
unsafe impl SckPin<SPI2> for PI1<Output<PushPull>, AF5> {}

// All SCK pins for SPI3
unsafe impl SckPin<SPI3> for PB3<Output<PushPull>, AF6> {}
unsafe impl SckPin<SPI3> for PC10<Output<PushPull>, AF6> {}

// All SCK pins for SPI4
unsafe impl SckPin<SPI4> for PE2<Output<PushPull>, AF5> {}
unsafe impl SckPin<SPI4> for PE12<Output<PushPull>, AF5> {}

// All SCK pins for SPI5
unsafe impl SckPin<SPI5> for PF7<Output<PushPull>, AF5> {}
unsafe impl SckPin<SPI5> for PH6<Output<PushPull>, AF5> {}
unsafe impl SckPin<SPI5> for PK0<Output<PushPull>, AF5> {}

// All SCK pins for SPI6
unsafe impl SckPin<SPI6> for PA5<Output<PushPull>, AF8> {}
unsafe impl SckPin<SPI6> for PB3<Output<PushPull>, AF8> {}
unsafe impl SckPin<SPI6> for PG13<Output<PushPull>, AF5> {}

// All MOSI pins for SPI1
unsafe impl MosiPin<SPI1> for PA7<Output<PushPull>, AF5> {}
unsafe impl MosiPin<SPI1> for PB5<Output<PushPull>, AF5> {}
unsafe impl MosiPin<SPI1> for PD7<Output<PushPull>, AF5> {}

// All MOSI pins for SPI2
unsafe impl MosiPin<SPI2> for PB15<Output<PushPull>, AF5> {} 
unsafe impl MosiPin<SPI2> for PC1<Output<PushPull>, AF5> {}
unsafe impl MosiPin<SPI2> for PC3<Output<PushPull>, AF5> {}
unsafe impl MosiPin<SPI2> for PI3<Output<PushPull>, AF5> {}

// All MOSI pins for SPI3
unsafe impl MosiPin<SPI3> for PB2<Output<PushPull>, AF7> {}
unsafe impl MosiPin<SPI3> for PC12<Output<PushPull>, AF6> {}
unsafe impl MosiPin<SPI3> for PD6<Output<PushPull>, AF5> {}

// All MOSI pins for SPI4
unsafe impl MosiPin<SPI4> for PE6<Output<PushPull>, AF5> {}
unsafe impl MosiPin<SPI4> for PE14<Output<PushPull>, AF5> {}

// All MOSI pins for SPI5
unsafe impl MosiPin<SPI5> for PF9<Output<PushPull>, AF5> {}
unsafe impl MosiPin<SPI5> for PF11<Output<PushPull>, AF5> {}
unsafe impl MosiPin<SPI5> for PJ10<Output<PushPull>, AF5> {}

// All MOSI pins for SPI6
unsafe impl MosiPin<SPI6> for PA7<Output<PushPull>, AF8> {}
unsafe impl MosiPin<SPI6> for PB5<Output<PushPull>, AF8> {}
unsafe impl MosiPin<SPI6> for PG14<Output<PushPull>, AF5> {}

// All MISO pins SPI1