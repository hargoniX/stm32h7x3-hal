use crate::gpio::gpioa::PA8;
use crate::gpio::gpiob::{PB10, PB11, PB6, PB7, PB8, PB9};
use crate::gpio::gpioc::PC9;
use crate::gpio::gpiod::{PD12, PD13};
use crate::gpio::gpiof::{PF0, PF1, PF14, PF15};
use crate::gpio::gpioh::{PH11, PH12, PH4, PH5, PH7, PH8};
use crate::gpio::AF4;
use crate::rcc::{Clocks, APB1L, APB4};
use crate::time::Hertz;
use stm32h7::stm32h7x3::{I2C1, I2C2, I2C3, I2C4};

/// A trait to represent the SCL Pin of an I2C Port
pub unsafe trait SclPin<I2C> {}

/// A trait to represent the SDL Pin of an I2C Port
pub unsafe trait SdaPin<I2C> {}

// I2C1_SCL
unsafe impl SclPin<I2C1> for PB6<AF4> {}
unsafe impl SclPin<I2C1> for PB8<AF4> {}

// I2C1_SDA
unsafe impl SdaPin<I2C1> for PB7<AF4> {}
unsafe impl SdaPin<I2C1> for PB9<AF4> {}

// I2C2_SCL
unsafe impl SclPin<I2C2> for PB10<AF4> {}
unsafe impl SclPin<I2C2> for PF1<AF4> {}
unsafe impl SclPin<I2C2> for PH4<AF4> {}

// I2C2_SDA
unsafe impl SdaPin<I2C2> for PB11<AF4> {}
unsafe impl SdaPin<I2C2> for PF0<AF4> {}
unsafe impl SdaPin<I2C2> for PH5<AF4> {}

// I2C3_SCL
unsafe impl SclPin<I2C3> for PA8<AF4> {}
unsafe impl SclPin<I2C3> for PH7<AF4> {}

// I2C3_SDA
unsafe impl SdaPin<I2C3> for PC9<AF4> {}
unsafe impl SdaPin<I2C3> for PH8<AF4> {}

// I2C4_SCL
unsafe impl SclPin<I2C4> for PD12<AF4> {}
unsafe impl SclPin<I2C4> for PF14<AF4> {}
unsafe impl SclPin<I2C4> for PH11<AF4> {}
unsafe impl SclPin<I2C4> for PB6<AF4> {}
unsafe impl SclPin<I2C4> for PB8<AF4> {}

// I2C4_SDA
unsafe impl SdaPin<I2C4> for PB7<AF4> {}
unsafe impl SdaPin<I2C4> for PB9<AF4> {}
unsafe impl SdaPin<I2C4> for PD13<AF4> {}
unsafe impl SdaPin<I2C4> for PF15<AF4> {}
unsafe impl SdaPin<I2C4> for PH12<AF4> {}

pub struct I2c<I2C, PINS> {
    i2c: I2C,
    pins: PINS,
}

macro_rules! i2c {
    ($($I2CX:ident: ($i2cX:ident, $i2cXen:ident, $i2cXrst:ident, $APBX:ident),)+) => {
        $(
            impl<SCL, SDA> I2c<$I2CX, (SCL, SDA)> {
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

                    let i2cclk = clocks.pclk4();

                    // Refer to figure 539 for this:
                    // Clear PE bit in I2C_CR1
                    unsafe { &(*I2C1::ptr()).cr1.modify(|_, w| w.pe().clear_bit())};

                    // Enable the Analog Noise Filter by setting ANFOFF (Analog Noise Filter OFF) to 0
                    // This is usually enabled by default but you never know
                    unsafe { &(*I2C1::ptr()).cr1.modify(|_, w| w.anfoff().clear_bit())};


                    I2c { i2c, pins}

                }
            }
        )+
    };
}

i2c!(
    I2C1: (i2c1, i2c1en, i2c1rst, APB1L),
    I2C2: (i2c2, i2c2en, i2c2rst, APB1L),
    I2C3: (i2c3, i2c3en, i2c3rst, APB1L),
    I2C4: (i2c4, i2c4en, i2c4rst, APB4),
);
