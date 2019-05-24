//! Inter Integrated Circuit implementation
// I2C implementation, largely taken over from japaric's HAL like so many other features here

use crate::gpio::gpioa::PA8;
use crate::gpio::gpiob::{PB10, PB11, PB6, PB7, PB8, PB9};
use crate::gpio::gpioc::PC9;
use crate::gpio::gpiod::{PD12, PD13};
use crate::gpio::gpiof::{PF0, PF1, PF14, PF15};
use crate::gpio::gpioh::{PH11, PH12, PH4, PH5, PH7, PH8};
use crate::gpio::{AF4, Output, OpenDrain};
use crate::rcc::{Clocks, APB1L, APB4};
use crate::time::Hertz;
use hal::blocking::i2c::{Write, WriteRead, Read};
use stm32h7::stm32h7x3::{I2C1, I2C2, I2C3, I2C4};
use cast::{u8, u16};


/// I2C error
#[derive(Debug)]
pub enum Error {
    /// Bus error
    Bus,
    /// Arbitration loss
    Arbitration,
    // Overrun, // slave mode only
    // Pec, // SMBUS mode only
    // Timeout, // SMBUS mode only
    // Alert, // SMBUS mode only
    #[doc(hidden)]
    _Extensible,
}

/// A trait to represent the SCL Pin of an I2C Port
pub unsafe trait SclPin<I2C> {}

/// A trait to represent the SDL Pin of an I2C Port
pub unsafe trait SdaPin<I2C> {}

// I2C1_SCL
unsafe impl SclPin<I2C1> for PB8<Output<OpenDrain>, AF4> {}

// I2C1_SDA
unsafe impl SdaPin<I2C1> for PB7<Output<OpenDrain>, AF4> {}
unsafe impl SdaPin<I2C1> for PB9<Output<OpenDrain>, AF4> {}

// I2C2_SCL
unsafe impl SclPin<I2C2> for PB10<Output<OpenDrain>, AF4> {}
unsafe impl SclPin<I2C2> for PF1<Output<OpenDrain>, AF4> {}
unsafe impl SclPin<I2C2> for PH4<Output<OpenDrain>, AF4> {}

// I2C2_SDA
unsafe impl SdaPin<I2C2> for PB11<Output<OpenDrain>, AF4> {}
unsafe impl SdaPin<I2C2> for PF0<Output<OpenDrain>, AF4> {}
unsafe impl SdaPin<I2C2> for PH5<Output<OpenDrain>, AF4> {}

// I2C3_SCL
unsafe impl SclPin<I2C3> for PA8<Output<OpenDrain>, AF4> {}
unsafe impl SclPin<I2C3> for PH7<Output<OpenDrain>, AF4> {}

// I2C3_SDA
unsafe impl SdaPin<I2C3> for PC9<Output<OpenDrain>, AF4> {}
unsafe impl SdaPin<I2C3> for PH8<Output<OpenDrain>, AF4> {}

// I2C4_SCL
unsafe impl SclPin<I2C4> for PD12<Output<OpenDrain>, AF4> {}
unsafe impl SclPin<I2C4> for PF14<Output<OpenDrain>, AF4> {}
unsafe impl SclPin<I2C4> for PH11<Output<OpenDrain>, AF4> {}
unsafe impl SclPin<I2C4> for PB6<Output<OpenDrain>, AF4> {}
unsafe impl SclPin<I2C4> for PB8<Output<OpenDrain>, AF4> {}

// I2C4_SDA
unsafe impl SdaPin<I2C4> for PB7<Output<OpenDrain>, AF4> {}
unsafe impl SdaPin<I2C4> for PB9<Output<OpenDrain>, AF4> {}
unsafe impl SdaPin<I2C4> for PD13<Output<OpenDrain>, AF4> {}
unsafe impl SdaPin<I2C4> for PF15<Output<OpenDrain>, AF4> {}
unsafe impl SdaPin<I2C4> for PH12<Output<OpenDrain>, AF4> {}

pub struct I2c<I2C, PINS> {
    i2c: I2C,
    pins: PINS,
}

macro_rules! busy_wait {
    ($i2c:expr, $flag:ident) => {
        loop {
            let isr = $i2c.isr.read();

            if isr.berr().bit_is_set() {
                return Err(Error::Bus);
            } else if isr.arlo().bit_is_set() {
                return Err(Error::Arbitration);
            } else if isr.$flag().bit_is_set() {
                break;
            } else {
                // try again
            }
        }
    };
}

macro_rules! i2c {
    ($($I2CX:ident: ($i2cX:ident, $i2cXen:ident, $i2cXrst:ident, $APBX:ident, $PCLKX:ident),)+) => {
        $(
            impl<SCL, SDA> I2c<$I2CX, (SCL, SDA)> {
                /// Basically a new function for the I2C peripheral
                pub fn $i2cX<F> (
                    i2c: $I2CX,
                    pins: (SCL, SDA),
                    freq: F,
                    clocks: Clocks,
                    apb: &mut $APBX
                ) -> Self where
                    F: Into<Hertz>,
                    SCL: SclPin<$I2CX>,
                    SDA: SdaPin<$I2CX>,
                {
                    apb.enr().modify(|_, w| w.$i2cXen().set_bit());
                    apb.rstr().modify(|_, w| w.$i2cXrst().set_bit());
                    apb.rstr().modify(|_, w| w.$i2cXrst().clear_bit());

                    let freq = freq.into().0;

                    assert!(freq <= 1_000_000);

                    let i2cclk = clocks.$PCLKX().0;    

                    // Refer to figure 539 for this:
                    // Clear PE bit in I2C_CR1
                    unsafe { &(*I2C1::ptr()).cr1.modify(|_, w| w.pe().clear_bit())};

                    // Enable the Analog Noise Filter by setting ANFOFF (Analog Noise Filter OFF) to 0
                    // This is usually enabled by default but you never know
                    unsafe { &(*I2C1::ptr()).cr1.modify(|_, w| w.anfoff().clear_bit())};

                    // experimental, not sure if this works
                                    // TODO review compliance with the timing requirements of I2C
                    // t_I2CCLK = 1 / PCLK1
                    // t_PRESC  = (PRESC + 1) * t_I2CCLK
                    // t_SCLL   = (SCLL + 1) * t_PRESC
                    // t_SCLH   = (SCLH + 1) * t_PRESC
                    //
                    // t_SYNC1 + t_SYNC2 > 4 * t_I2CCLK
                    // t_SCL ~= t_SYNC1 + t_SYNC2 + t_SCLL + t_SCLH
                    let ratio = i2cclk / freq - 4;
                    let (presc, scll, sclh, sdadel, scldel) = if freq > 100_000 {
                        // fast-mode or fast-mode plus
                        // here we pick SCLL + 1 = 2 * (SCLH + 1)
                        let presc = ratio / 387;

                        let sclh = ((ratio / (presc + 1)) - 3) / 3;
                        let scll = 2 * (sclh + 1) - 1;

                        let (sdadel, scldel) = if freq > 400_000 {
                            // fast-mode plus
                            let sdadel = 0;
                            let scldel = i2cclk / 4_000_000 / (presc + 1) - 1;

                            (sdadel, scldel)
                        } else {
                            // fast-mode
                            let sdadel = i2cclk / 8_000_000 / (presc + 1);
                            let scldel = i2cclk / 2_000_000 / (presc + 1) - 1;

                            (sdadel, scldel)
                        };

                        (presc, scll, sclh, sdadel, scldel)
                    } else {
                        // standard-mode
                        // here we pick SCLL = SCLH
                        let presc = ratio / 514;
                        let sclh = ((ratio / (presc + 1)) - 2) / 2;
                        let scll = sclh;

                        let sdadel = i2cclk / 2_000_000 / (presc + 1);
                        let scldel = i2cclk / 800_000 / (presc + 1) - 1;

                        (presc, scll, sclh, sdadel, scldel)
                    };

                    let presc = u8(presc).unwrap();
                    //assert!(presc < 16);
                    let scldel = u8(scldel).unwrap();
                    //assert!(scldel < 16);
                    let sdadel = u8(sdadel).unwrap();
                    //assert!(sdadel < 16);
                    let sclh = u8(sclh).unwrap();
                    let scll = u8(scll).unwrap();

                    // Configure for "fast mode" (400 KHz)
                    i2c.timingr.write(|w| 
                        w.presc()
                            .bits(presc)
                            .scll()
                            .bits(scll)
                            .sclh()
                            .bits(sclh)
                            .sdadel()
                            .bits(sdadel)
                            .scldel()
                            .bits(scldel)
                    );

                    // Enable the peripheral
                    i2c.cr1.write(|w| w.pe().set_bit());

                    I2c { i2c, pins}

                }
                
                /// Releases the I2C peripheral and associated pins
                pub fn free(self) -> ($I2CX, (SCL, SDA)) {
                    (self.i2c, self.pins)
                }
            }
            impl<PINS> Write for I2c<$I2CX, PINS> {
                type Error = Error;

                fn write(&mut self, addr: u8, bytes: &[u8]) -> Result<(), Error> {
                    // TODO support transfers of more than 255 bytes
                    assert!(bytes.len() < 256 && bytes.len() > 0);

                    // START and prepare to send `bytes`
                    self.i2c.cr2.write(|w| {
                        w.start()
                            .set_bit()
                            .sadd()
                            .bits(u16(addr << 1 | 0))
                            .add10().clear_bit()
                            .rd_wrn()
                            .clear_bit()
                            .nbytes()
                            .bits(bytes.len() as u8)
                            .autoend()
                            .set_bit()
                    });

                    for byte in bytes {
                        // Wait until we are allowed to send data (START has been ACKed or last byte
                        // when through)
                        busy_wait!(self.i2c, txis);

                        // put byte on the wire
                        self.i2c.txdr.write(|w| w.txdata().bits(*byte));
                    }

                    // Wait until the last transmission is finished ???
                    // busy_wait!(self.i2c, busy);

                    // automatic STOP

                    Ok(())
                }
            }

            impl<PINS> WriteRead for I2c<$I2CX, PINS> {
                type Error = Error;

                fn write_read(
                    &mut self,
                    addr: u8,
                    bytes: &[u8],
                    buffer: &mut [u8],
                ) -> Result<(), Error> {
                    // TODO support transfers of more than 255 bytes
                    assert!(bytes.len() < 256 && bytes.len() > 0);
                    assert!(buffer.len() < 256 && buffer.len() > 0);

                    // TODO do we have to explicitly wait here if the bus is busy (e.g. another
                    // master is communicating)?

                    // START and prepare to send `bytes`
                    self.i2c.cr2.write(|w| {
                        w.start()
                            .set_bit()
                            .sadd()
                            .bits(u16(addr << 1 | 0))
                            .add10().clear_bit()
                            .rd_wrn()
                            .clear_bit()
                            .nbytes()
                            .bits(bytes.len() as u8)
                            .autoend()
                            .clear_bit()

                    });
                    //busy_wait!(self.i2c, addr);
                    for byte in bytes {
                        // Wait until we are allowed to send data (START has been ACKed or last byte
                        // when through)
                        // put byte on the wire
                        busy_wait!(self.i2c, txis);
                        self.i2c.txdr.write(|w| w.txdata().bits(*byte));
                        
                    }

                    // Wait until the last transmission is finished
                    busy_wait!(self.i2c, tc);

                    // reSTART and prepare to receive bytes into `buffer`
                    self.i2c.cr2.write(|w| {
                        w.sadd()
                            .bits(u16(addr << 1 | 1))
                            .add10().clear_bit()
                            .rd_wrn()
                            .set_bit()
                            .nbytes()
                            .bits(buffer.len() as u8)
                            .start()
                            .set_bit()
                            .autoend()
                            .set_bit()
                    });

                    for byte in buffer {
                        // Wait until we have received something
                        busy_wait!(self.i2c, rxne);

                        *byte = self.i2c.rxdr.read().rxdata().bits();
                    }

                    // automatic STOP

                    Ok(())
                }
            }

            impl<PINS> Read for I2c<$I2CX, PINS> {
            type Error = Error;

            fn read(
                &mut self,
                addr: u8,
                buffer: &mut [u8],
            ) -> Result<(), Error> {
                // TODO support transfers of more than 255 bytes
                assert!(buffer.len() < 256 && buffer.len() > 0);

                // TODO do we have to explicitly wait here if the bus is busy (e.g. another
                // master is communicating)?

                // reSTART and prepare to receive bytes into `buffer`
                self.i2c.cr2.write(|w| {
                    w.sadd()
                        .bits((addr << 1 | 0) as u16)
                        .rd_wrn()
                        .set_bit()
                        .nbytes()
                        .bits(buffer.len() as u8)
                        .start()
                        .set_bit()
                        .autoend()
                        .set_bit()
                });

                for byte in buffer {
                    // Wait until we have received something
                    busy_wait!(self.i2c, rxne);

                    *byte = self.i2c.rxdr.read().rxdata().bits();
                }

                // automatic STOP

                Ok(())
            }
            }
        )+
    };
}

i2c!(
    I2C1: (i2c1, i2c1en, i2c1rst, APB1L, pclk1),
    I2C2: (i2c2, i2c2en, i2c2rst, APB1L, pclk4),
    I2C3: (i2c3, i2c3en, i2c3rst, APB1L, pclk4),
    I2C4: (i2c4, i2c4en, i2c4rst, APB4, pclk4),
);
