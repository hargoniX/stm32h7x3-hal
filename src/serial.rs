use core::marker::PhantomData;
use core::ptr;

use hal::serial;
use nb;
use void::Void;
use crate::gpio::{Floating, Input, Output, PushPull, AF4, AF7};
use crate::gpio::gpioa::{PA2, PA3, PA9, PA10};
use crate::gpio::gpiob::{PB6, PB7, PB10, PB11, PB14, PB15};
use crate::gpio::gpioc::{PC6, PC10, PC11};
use crate::gpio::gpiod::{PD5, PD6, PD8, PD9};
use crate::gpio::gpiog::{PG9, PG14};
use crate::stm32h7x3::{USART1, USART2, USART3, USART6};
use crate::time::Bps;
use crate::rcc::{APB1L, APB2, Clocks};

/// Interrupt event
pub enum Event {
    /// New data has been received
    Rxne,
    /// New data can be sent
    Txe,
}

/// Serial error
#[derive(Debug)]
pub enum Error {
    /// Framing error
    Framing,
    /// Noise error
    Noise,
    /// RX buffer overrun
    Overrun,
    /// Parity check error
    Parity,
    #[doc(hidden)]
    _Extensible,
}

/// TX pin - DO NOT IMPLEMENT THIS TRAIT
pub unsafe trait TxPin<USART> {}

/// RX pin - DO NOT IMPLEMENT THIS TRAIT
pub unsafe trait RxPin<USART> {}

// USART1 TX
unsafe impl TxPin<USART1> for PB14<Output<PushPull>, AF4> {}
unsafe impl TxPin<USART1> for PA9<Output<PushPull>, AF7> {}
unsafe impl TxPin<USART1> for PB6<Output<PushPull>, AF7> {}

// USART2 TX
unsafe impl TxPin<USART2> for PA2<Output<PushPull>, AF7> {}
unsafe impl TxPin<USART2> for PD5<Output<PushPull>, AF7> {}

// USART3 TX
unsafe impl TxPin<USART3> for PB10<Output<PushPull>, AF7> {}
unsafe impl TxPin<USART3> for PD8<Output<PushPull>, AF7> {}
unsafe impl TxPin<USART3> for PC10<Output<PushPull>, AF7> {}

// USART6 TX
unsafe impl TxPin<USART6> for PC6<Output<PushPull>, AF7> {}
unsafe impl TxPin<USART6> for PG14<Output<PushPull>, AF7> {}

// USART1 RX
unsafe impl RxPin<USART1> for PB15<Input<Floating>, AF4> {}
unsafe impl RxPin<USART1> for PA10<Input<Floating>, AF7> {}
unsafe impl RxPin<USART1> for PB7<Input<Floating>, AF7> {}

// USART2 RX
unsafe impl RxPin<USART2> for PA3<Input<Floating>, AF7> {}
unsafe impl RxPin<USART2> for PD6<Input<Floating>, AF7> {}

// USART3 RX
unsafe impl RxPin<USART3> for PB11<Input<Floating>, AF7> {}
unsafe impl RxPin<USART3> for PD9<Input<Floating>, AF7> {}
unsafe impl RxPin<USART3> for PC11<Input<Floating>, AF7> {}

// USART6 RX
unsafe impl RxPin<USART6> for PC6<Input<Floating>, AF7> {}
unsafe impl RxPin<USART6> for PG9<Input<Floating>, AF7> {}

/// Serial abstraction
pub struct Serial<USART, PINS> {
    usart: USART,
    pins: PINS,
}

/// Serial receiver
pub struct Rx<USART> {
    _usart: PhantomData<USART>,
}

/// Serial transmitter
pub struct Tx<USART> {
    _usart: PhantomData<USART>,
}

macro_rules! hal {
    ($(
        $USARTX:ident: ($usartX:ident, $APB:ident, $usartXen:ident, $usartXrst:ident, $pclkX:ident),
    )+) => {
        $(
            impl<TX, RX> Serial<$USARTX, (TX, RX)> {
                /// Configures a USART peripheral to provide serial communication
                pub fn $usartX(
                    usart: $USARTX,
                    pins: (TX, RX),
                    baud_rate: Bps,
                    clocks: Clocks,
                    apb: &mut $APB,
                ) -> Self
                where
                    TX: TxPin<$USARTX>,
                    RX: RxPin<$USARTX>,
                {
                    // enable or reset $USARTX
                    apb.enr().modify(|_, w| w.$usartXen().set_bit());
                    apb.rstr().modify(|_, w| w.$usartXrst().set_bit());
                    apb.rstr().modify(|_, w| w.$usartXrst().clear_bit());

                    // Word length = 8 bit
                    usart.cr1.write(|w|  w.m0().clear_bit().m1().clear_bit());

                    // Configure the baud rate
                    let brr = clocks.$pclkX().0 / baud_rate.0;
                    assert!(brr >= 16, "impossible baud rate");
                    usart.brr.write(|w| unsafe { w.bits(brr) });

                    // Set stop bits to 1
                    usart.cr2.write(|w| w.stop().bits(0b00));

                    // UE: enable USART
                    // RE: enable receiver
                    // TE: enable transceiver
                    usart
                        .cr1
                        .write(|w| w.ue().set_bit().re().set_bit().te().set_bit());

                    Serial { usart, pins }
                }

                /// Starts listening for an interrupt event
                pub fn listen(&mut self, event: Event) {
                    match event {
                        Event::Rxne => {
                            self.usart.cr1.modify(|_, w| w.rxneie().set_bit())
                        },
                        Event::Txe => {
                            self.usart.cr1.modify(|_, w| w.txeie().set_bit())
                        },
                    }
                }

                /// Starts listening for an interrupt event
                pub fn unlisten(&mut self, event: Event) {
                    match event {
                        Event::Rxne => {
                            self.usart.cr1.modify(|_, w| w.rxneie().clear_bit())
                        },
                        Event::Txe => {
                            self.usart.cr1.modify(|_, w| w.txeie().clear_bit())
                        },
                    }
                }

                /// Splits the `Serial` abstraction into a transmitter and a receiver half
                pub fn split(self) -> (Tx<$USARTX>, Rx<$USARTX>) {
                    (
                        Tx {
                            _usart: PhantomData,
                        },
                        Rx {
                            _usart: PhantomData,
                        },
                    )
                }

                /// Releases the USART peripheral and associated pins
                pub fn free(self) -> ($USARTX, (TX, RX)) {
                    (self.usart, self.pins)
                }
            }

            impl serial::Read<u8> for Rx<$USARTX> {
                type Error = Error;

                fn read(&mut self) -> nb::Result<u8, Error> {
                    // NOTE(unsafe) atomic read with no side effects
                    let isr = unsafe { (*$USARTX::ptr()).isr.read() };

                    Err(if isr.pe().bit_is_set() {
                        nb::Error::Other(Error::Parity)
                    } else if isr.fe().bit_is_set() {
                        nb::Error::Other(Error::Framing)
                    } else if isr.nf().bit_is_set() {
                        nb::Error::Other(Error::Noise)
                    } else if isr.ore().bit_is_set() {
                        nb::Error::Other(Error::Overrun)
                    } else if isr.rxne().bit_is_set() {
                        // NOTE(read_volatile) see `write_volatile` below
                        return Ok(unsafe {
                            ptr::read_volatile(&(*$USARTX::ptr()).rdr as *const _ as *const _)
                        });
                    } else {
                        nb::Error::WouldBlock
                    })
                }
            }

            impl serial::Write<u8> for Tx<$USARTX> {
                // NOTE(Void) See section "29.7 USART interrupts"; the only possible errors during
                // transmission are: clear to send (which is disabled in this case) errors and
                // framing errors (which only occur in SmartCard mode); neither of these apply to
                // our hardware configuration
                type Error = Void;

                fn flush(&mut self) -> nb::Result<(), Void> {
                    // NOTE(unsafe) atomic read with no side effects
                    let isr = unsafe { (*$USARTX::ptr()).isr.read() };

                    if isr.tc().bit_is_set() {
                        Ok(())
                    } else {
                        Err(nb::Error::WouldBlock)
                    }
                }

                fn write(&mut self, byte: u8) -> nb::Result<(), Void> {
                    // NOTE(unsafe) atomic read with no side effects
                    let isr = unsafe { (*$USARTX::ptr()).isr.read() };

                    if isr.txe().bit_is_set() {
                        // NOTE(unsafe) atomic write to stateless register
                        // NOTE(write_volatile) 8-bit write that's not possible through the svd2rust API
                        unsafe {
                            ptr::write_volatile(&(*$USARTX::ptr()).tdr as *const _ as *mut _, byte)
                        }

                        // NOTE(point 8) we maybe gotta implement point 8 on page 2031 of the reference manual here if we encounter bugs
                        Ok(())
                    } else {
                        Err(nb::Error::WouldBlock)
                    }
                }
            }
        )+
    }
}

hal! {
    USART1: (usart1, APB2, usart1en, usart1rst, pclk2),
    USART2: (usart2, APB1L, usart2en, usart2rst, pclk1),
    USART3: (usart3, APB1L, usart3en, usart3rst, pclk1),
    USART6: (usart6, APB2, usart6en, usart6rst, pclk2),
}