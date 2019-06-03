//! Serial peripheral interface implementation

use core::ptr;

use hal::spi::{FullDuplex, Mode, Phase, Polarity};
use stm32h7::stm32h7x3::{SPI1, SPI2, SPI3, SPI4, SPI5, SPI6};
use crate::gpio::gpioa::{PA5, PA6, PA7, PA9, PA12};
use crate::gpio::gpiob::{PB2, PB3, PB4, PB5, PB10, PB13, PB14, PB15};
use crate::gpio::gpioc::{PC1, PC2, PC3, PC10, PC11, PC12};
use crate::gpio::gpiod::{PD3, PD6, PD7};
use crate::gpio::gpioe::{PE2, PE5, PE6, PE12, PE13, PE14};
use crate::gpio::gpiof::{PF7, PF8, PF9, PF11};
use crate::gpio::gpiog::{PG9, PG11, PG12, PG13, PG14};
use crate::gpio::gpioh::{PH6, PH7};
use crate::gpio::gpioi::{PI1, PI2, PI3};
use crate::gpio::gpioj::{PJ10, PJ11};
use crate::gpio::gpiok::{PK0};
use crate::gpio::{AF5, AF6, AF7, AF8, Output, Input, PushPull, Floating};
use crate::time::Hertz;
use crate::rcc::{APB1L, APB2, APB4, Clocks};

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

// All MISO pins for SPI1
unsafe impl MisoPin<SPI1> for PA6<Input<Floating>, AF5> {}
unsafe impl MisoPin<SPI1> for PG9<Input<Floating>, AF5> {}
unsafe impl MisoPin<SPI1> for PB4<Input<Floating>, AF5> {}

// All MISO pins for SPI2
unsafe impl MisoPin<SPI2> for PC2<Input<Floating>, AF5> {}
unsafe impl MisoPin<SPI2> for PB14<Input<Floating>, AF5> {}
unsafe impl MisoPin<SPI2> for PI2<Input<Floating>, AF5> {}

unsafe impl MisoPin<SPI3> for PC11<Input<Floating>, AF6> {}
unsafe impl MisoPin<SPI3> for PB4<Input<Floating>, AF6> {}

unsafe impl MisoPin<SPI4> for PE5<Input<Floating>, AF5> {}
unsafe impl MisoPin<SPI4> for PE13<Input<Floating>, AF5> {}

unsafe impl MisoPin<SPI5> for PF8<Input<Floating>, AF5> {}
unsafe impl MisoPin<SPI5> for PH7<Input<Floating>, AF5> {}
unsafe impl MisoPin<SPI5> for PJ11<Input<Floating>, AF5> {}

unsafe impl MisoPin<SPI6> for PA6<Input<Floating>, AF8> {}
unsafe impl MisoPin<SPI6> for PG12<Input<Floating>, AF5> {}
unsafe impl MisoPin<SPI6> for PB4<Input<Floating>, AF8> {}

#[derive(Debug)]
pub enum Error {
    /// Overrun occurred
    Overrun,
    /// Mode fault occurred
    ModeFault,
    /// CRC error
    Crc,
    #[doc(hidden)]
    _Extensible,  
}

pub struct Spi<SPI, PINS> {
    spi: SPI,
    pins: PINS,
}

macro_rules! hal_spi {
    ($($SPIX:ident: ($spiX:ident, $APBX:ident, $spiXen:ident, $spiXrst:ident, $pclkX:ident),)+) => {
        $(
            impl<SCK, MISO, MOSI> Spi<$SPIX, (SCK, MISO, MOSI)> {
                pub fn $spiX<F>(
                    spi: $SPIX,
                    pins: (SCK, MISO, MOSI),
                    mode: Mode,
                    freq: F,
                    clocks: Clocks,
                    apb: &mut $APBX,
                ) -> Self
                where
                    F: Into<Hertz>,
                    SCK: SckPin<$SPIX>,
                    MISO: MisoPin<$SPIX>,
                    MOSI: MisoPin<$SPIX>,
                {
                    let mbr = match clocks.$pclkX().0 / freq.into().0 {
                        0 => unreachable!(),
                        1...2 => 0b000,
                        3...5 => 0b001,
                        6...11 => 0b010,
                        12...23 => 0b011,
                        24...39 => 0b100,
                        40...95 => 0b101,
                        96...191 => 0b110,
                        _ => 0b111,
                    };

                    apb.enr().modify(|_, w| w.$spiXen().set_bit());
                    apb.rstr().modify(|_, w| w.$spiXrst().set_bit());
                    apb.rstr().modify(|_, w| w.$spiXrst().clear_bit());

                    spi.cfg1.modify(|_, w| unsafe {
                        w.mbr()
                            .bits(mbr)
                            .crcen()
                            .clear_bit()
                    });

                    spi.cfg2.modify(|_, w| {
                        w.ssm()
                            .set_bit()
                            .cpha()
                            .bit(mode.phase == Phase::CaptureOnSecondTransition)
                            .cpol()
                            .bit(mode.polarity == Polarity::IdleHigh)
                            .lsbfrst()
                            .clear_bit()
                            .afcntr()
                            .set_bit()
                    });

                    spi.cr1.modify(|_, w| {w.ssi().set_bit()});

                    Spi { spi, pins }
                }
            }
            
            impl<PINS> FullDuplex<u8> for Spi<$SPIX, PINS> {
                type Error = Error;

                fn read(&mut self) -> nb::Result<u8, Error> {
                    let sr = self.spi.sr.read();

                    Err(if sr.ovr().bit_is_set() {
                        nb::Error::Other(Error::Overrun)
                    } else if sr.modf().bit_is_set() {
                        nb::Error::Other(Error::ModeFault)
                    } else if sr.crce().bit_is_set() {
                        nb::Error::Other(Error::Crc)
                    } else if sr.rxwne().bit_is_set() {
                        // NOTE(read_volatile) read only 1 byte (the svd2rust API only allows
                        // reading a half-word)
                        return Ok(unsafe {
                            ptr::read_volatile(&self.spi.rxdr as *const _ as *const u8)
                        });
                    } else {
                        nb::Error::WouldBlock
                    })
                }

                fn send(&mut self, byte: u8) -> nb::Result<(), Error> {
                    let sr = self.spi.sr.read();

                    Err(if sr.ovr().bit_is_set() {
                        nb::Error::Other(Error::Overrun)
                    } else if sr.modf().bit_is_set() {
                        nb::Error::Other(Error::ModeFault)
                    } else if sr.crce().bit_is_set() {
                        nb::Error::Other(Error::Crc)

                    // NOTE(TXC) not sure if this bit is the proper one to use here
                    // If we encounter logic bugs later this might be the cause
                    } else if sr.txc().bit_is_set() {
                        // NOTE(write_volatile) see note above
                        unsafe { ptr::write_volatile(&self.spi.txdr as *const _ as *mut u8, byte) }
                        return Ok(());
                    } else {
                        nb::Error::WouldBlock
                    })
                }
            }

            impl<PINS> ::hal::blocking::spi::transfer::Default<u8> for Spi<$SPIX, PINS> {}

            impl<PINS> ::hal::blocking::spi::write::Default<u8> for Spi<$SPIX, PINS> {}
            
        )+
    };
}


hal_spi! {
    SPI1: (spi1, APB2, spi1en, spi1rst, pclk2),
    SPI2: (spi2, APB1L, spi2en, spi2rst, pclk1),
    SPI3: (spi3, APB1L, spi3en, spi3rst, pclk1),
    SPI4: (spi4, APB2, spi4en, spi4rst, pclk2),
    SPI5: (spi5, APB2, spi5en, spi5rst, pclk2),
    SPI6: (spi6, APB4, spi6en, spi6rst, pclk4),
}