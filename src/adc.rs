use embedded_hal::adc::{Channel, OneShot};
use stm32h7::stm32h7x3::{ADC1, ADC2, ADC3};
// use stm32h7::stm32h7x3::rcc::{AHB1RSTR, AHB1ENR, AHB4RSTR, AHB4ENR};

use crate::gpio::{Analog, AF0};
use crate::gpio::gpioa::{PA0, PA1, PA2, PA3, PA4, PA5, PA6, PA7};
use crate::gpio::gpiob::{PB0, PB1};
use crate::gpio::gpioc::{PC0, PC1, PC2, PC3, PC4, PC5};
use crate::gpio::gpiof::{PF3, PF4, PF5, PF6, PF7, PF8, PF9, PF10, PF11, PF12, PF13, PF14};
use crate::gpio::gpioh::{PH2, PH3, PH4, PH5};
use crate::delay::Delay;
use crate::rcc::{AHB1, AHB4};

use hal::blocking::delay::DelayUs;

pub struct Adc<'a, ADC> {
    rb: ADC,
    sample_time: AdcSampleTime,
    align: AdcAlign,
    resolution: AdcSampleResolution,
    delay: &'a mut Delay,
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[allow(non_camel_case_types)]
/// ADC sampling time
///
/// Options for the sampling time, each is T + 0.5 ADC clock cycles.
pub enum AdcSampleTime {
    /// 1.5 cycles sampling time
    T_1,
    /// 2.5 cycles sampling time
    T_2,
    /// 8.5 cycles sampling time
    T_8,
    /// 16.5 cycles sampling time
    T_16,
    /// 32.5 cycles sampling time
    T_32,
    /// 64.5 cycles sampling time
    T_64,
    /// 387.5 cycles sampling time
    T_387,
    /// 810.5 cycles sampling time
    T_810,
}

impl From<AdcSampleTime> for u8 {
    fn from(val: AdcSampleTime) -> u8 {
        match val {
            AdcSampleTime::T_1 => 0,
            AdcSampleTime::T_2 => 1,
            AdcSampleTime::T_8 => 2,
            AdcSampleTime::T_16 => 3,
            AdcSampleTime::T_32 => 4,
            AdcSampleTime::T_64 => 5,
            AdcSampleTime::T_387 => 6,
            AdcSampleTime::T_810 => 7,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
/// ADC sampling resolution
/// 
/// Options for sampling resolution
pub enum AdcSampleResolution {
    /// 16 bit resulution
    B_16,
    /// 14 bit resolution
    B_14,
    /// 12 bit resolution
    B_12,
    /// 10 bit resolution
    B_10,
    /// 8 bit resolution
    B_8,
}

impl AdcSampleResolution {
    pub fn default() -> Self {
        AdcSampleResolution::B_16
    }
}

impl From<AdcSampleResolution> for u8 {
    fn from(val: AdcSampleResolution) -> u8 {
        match val {
            AdcSampleResolution::B_16 => 0b000,
            AdcSampleResolution::B_14 => 0b001,
            AdcSampleResolution::B_12 => 0b010,
            AdcSampleResolution::B_10 => 0b011,
            AdcSampleResolution::B_8 => 0b111,
        }
    }
}

impl AdcSampleTime {
    pub fn default() -> Self {
        AdcSampleTime::T_32
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AdcAlign {
    Right,
    Left,
}

impl AdcAlign {
    pub fn default() -> Self {
        AdcAlign::Right
    }
}

impl From<AdcAlign> for bool {
    fn from(val: AdcAlign) -> Self {
        match val {
            AdcAlign::Left => false,
            AdcAlign::Right => true,
        }
    }
}

macro_rules! adc_pins {
    ($ADC:ident, $($pin:ident => $chan:expr),+ $(,)*) => {
        $(
            impl Channel<$ADC> for $pin<Analog, AF0> {
                type ID = u8;

                fn channel() -> u8 {
                    $chan
                }
            }
        )+
    };
}

// Not implementing Pxy_C adc pins
// Just implmenting INPx pins (INNx defaulting to V_ref-)
adc_pins!(ADC1,
    // No 0, 1
    PF11 => 2,
    PA6 => 3,
    PC4 => 4,
    PB1 => 5,
    PF12 => 6,
    PA7 => 7,
    PC5 => 8,
    PB0 => 9,
    PC0 => 10,
    PC1 => 11,
    PC2 => 12,
    PC3 => 13,
    PA2 => 14,
    PA3 => 15,
    PA0 => 16,
    PA1 => 17,
    PA4 => 18,
    PA5 => 19,
);

adc_pins!(ADC2,
    // No 0, 1
    PF13 => 2,
    PA6 => 3,
    PC4 => 4,
    PB1 => 5,
    PF14 => 6,
    PA7 => 7,
    PC5 => 8,
    PB0 => 9,
    PC0 => 10,
    PC1 => 11,
    PC2 => 12,
    PC3 => 13,
    PA2 => 14,
    PA3 => 15,
    // No 16, 17
    PA4 => 18,
    PA5 => 19,
);

adc_pins!(ADC3,
    // No 0, 1
    PF9 => 2,
    PF7 => 3,
    PF5 => 4,
    PF3 => 5,
    PF10 => 6,
    PF8 => 7,
    PF6 => 8,
    PF4 => 9,
    PC0 => 10,
    PC1 => 11,
    PC2 => 12,
    PH2 => 13,
    PH3 => 14,
    PH4 => 15,
    PH5 => 16,
    // No 17...19
);

/// Stored ADC config can be restored using the `Adc::restore_cfg` method
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct StoredConfig(AdcSampleTime, AdcAlign, AdcSampleResolution);

#[allow(unused_macros)]
macro_rules! adc_hal {
    ($(
        $ADC:ident: (
            $init:ident,
            $adcxen:ident,
            $adcxrst:ident,
            $AHB:ident
        ) $(,)*
    )+) => {
        $(
            impl<'a> Adc<'a, $ADC> {
                /// Init a new Adc
                ///
                /// Sets all configurable parameters to one-shot defaults,
                /// performs a boot-time calibration.
                pub fn $init(adc: $ADC, ahb: &mut $AHB, delay: &'a mut Delay) -> Self {
                    let mut s = Self {
                        rb: adc,
                        sample_time: AdcSampleTime::default(),
                        align: AdcAlign::default(),
                        resolution: AdcSampleResolution::default(),
                        delay,
                    };
                    s.enable_clock(ahb);
                    s.power_down();
                    s.reset(ahb);
                    s.configure();
                    s.power_up();
                    s.disable();
                    s.calibrate();
                    s.enable();
                    s.disable();
                    s.power_down();
                    s
                }

                /// Save current ADC config
                pub fn save_cfg(&mut self) -> StoredConfig {
                    StoredConfig(self.sample_time, self.align, self.resolution)
                }

                /// Restore saved ADC config
                pub fn restore_cfg(&mut self, cfg: StoredConfig) {
                    self.sample_time = cfg.0;
                    self.align = cfg.1;
                    self.resolution = cfg.2;
                }

                /// Reset the ADC config to default, return existing config
                pub fn default_cfg(&mut self) -> StoredConfig {
                    let cfg = self.save_cfg();
                    self.sample_time = AdcSampleTime::default();
                    self.align = AdcAlign::default();
                    self.resolution = AdcSampleResolution::default();
                    cfg
                }

                /// Set ADC sampling time
                ///
                /// Options can be found in [AdcSampleTime](crate::adc::AdcSampleTime).
                pub fn set_sample_time(&mut self, t_samp: AdcSampleTime) {
                    self.sample_time = t_samp;
                }

                /// Set the Adc result alignment
                ///
                /// Options can be found in [AdcAlign](crate::adc::AdcAlign).
                pub fn set_align(&mut self, align: AdcAlign) {
                    self.align = align;
                }

                /// Set ADC sampling resolution
                /// 
                /// Options can be found in [AdcSampleResolution](crate::adc::AdcSampleResolution)
                pub fn set_resolution(&mut self, res: AdcSampleResolution) {
                    self.resolution = res;
                }

                /// Returns the largest possible sample value for the current settings
                pub fn max_sample(&self) -> u32 {
                    match self.align {
                        // AdcAlign::Left => u16::max_value(),
                        AdcAlign::Left => u32::max_value() << (32 - self.resolution as u32),
                        AdcAlign::Right => (1 << self.resolution as u32) - 1,
                    }
                }

                fn power_up(&mut self) {
                    self.rb.cr.modify(|_, w| 
                        w.deeppwd().clear_bit()
                            .advregen().set_bit()
                    );
                    self.delay.delay_us(10_u8);
                }

                fn power_down(&mut self) {
                    self.rb.cr.modify(|_, w| 
                        w.deeppwd().set_bit()
                            .advregen().clear_bit()
                    );
                }

                fn enable(&mut self) {
                    self.rb.isr.modify(|_, w| w.adrdy().set_bit());
                    self.rb.cr.modify(|_, w| w.aden().set_bit());
                    while self.rb.isr.read().adrdy().bit_is_clear() {}
                    self.rb.isr.modify(|_, w| w.adrdy().set_bit());
                }

                fn disable(&mut self) {
                    if self.rb.cr.read().adstart().bit_is_set() || self.rb.cr.read().jadstart().bit_is_set() {
                        self.rb.cr.modify(|_, w|
                            w.adstp().set_bit()
                                .jadstp().set_bit()
                        );
                        while self.rb.cr.read().adstp().bit_is_set() {}
                        while self.rb.cr.read().jadstp().bit_is_set() {}
                    }
                    self.rb.cr.modify(|_, w| w.addis().set_bit());
                    while self.rb.cr.read().aden().bit_is_set() {}
                }

                fn reset(&mut self, ahb: &mut $AHB) {
                    ahb.rstr().modify(|_, w| w.$adcxrst().set_bit());
                }

                fn enable_clock(&mut self, ahb: &mut $AHB) {
                    ahb.enr().modify(|_, w| w.$adcxen().set_bit());
                }

                fn configure(&mut self) {
                    // Single conversion mode, Software trigger (no hardware trigger), context queue enabled
                    self.rb.cfgr.modify(|_, w| unsafe {
                        w.cont().clear_bit()
                            .exten().bits(0x0)
                            .jqdis().clear_bit()
                            .discen().set_bit()
                            .res().bits(self.resolution.into())
                    });
                    self.rb.jsqr.modify(|_, w| unsafe { w.jexten().bits(0x0) });
                }

                fn calibrate(&mut self) {
                    // single channel (INNx equals to V_ref-)
                    self.rb.cr.modify(|_, w| 
                        w.adcaldif().clear_bit()
                            .adcallin().clear_bit()
                    );
                    // calibrate
                    self.rb.cr.modify(|_, w| w.adcal().set_bit());
                    while self.rb.cr.read().adcal().bit_is_set() {}
                }

                fn set_chan_smps(&mut self, chan: u8) {
                    match chan {
                        1 => self.rb.smpr1.modify(|_, w| unsafe { w.smp1().bits(self.sample_time.into()) }),
                        2 => self.rb.smpr1.modify(|_, w| unsafe { w.smp2().bits(self.sample_time.into()) }),
                        3 => self.rb.smpr1.modify(|_, w| unsafe { w.smp3().bits(self.sample_time.into()) }),
                        4 => self.rb.smpr1.modify(|_, w| unsafe { w.smp4().bits(self.sample_time.into()) }),
                        5 => self.rb.smpr1.modify(|_, w| unsafe { w.smp5().bits(self.sample_time.into()) }),
                        6 => self.rb.smpr1.modify(|_, w| unsafe { w.smp6().bits(self.sample_time.into()) }),
                        7 => self.rb.smpr1.modify(|_, w| unsafe { w.smp7().bits(self.sample_time.into()) }),
                        8 => self.rb.smpr1.modify(|_, w| unsafe { w.smp8().bits(self.sample_time.into()) }),
                        9 => self.rb.smpr1.modify(|_, w| unsafe { w.smp9().bits(self.sample_time.into()) }),
                        10 => self.rb.smpr2.modify(|_, w| unsafe { w.smp10().bits(self.sample_time.into()) }),
                        11 => self.rb.smpr2.modify(|_, w| unsafe { w.smp11().bits(self.sample_time.into()) }),
                        12 => self.rb.smpr2.modify(|_, w| unsafe { w.smp12().bits(self.sample_time.into()) }),
                        13 => self.rb.smpr2.modify(|_, w| unsafe { w.smp13().bits(self.sample_time.into()) }),
                        14 => self.rb.smpr2.modify(|_, w| unsafe { w.smp14().bits(self.sample_time.into()) }),
                        15 => self.rb.smpr2.modify(|_, w| unsafe { w.smp15().bits(self.sample_time.into()) }),
                        16 => self.rb.smpr2.modify(|_, w| unsafe { w.smp16().bits(self.sample_time.into()) }),
                        17 => self.rb.smpr2.modify(|_, w| unsafe { w.smp17().bits(self.sample_time.into()) }),
                        18 => self.rb.smpr2.modify(|_, w| unsafe { w.smp18().bits(self.sample_time.into()) }),
                        19 => self.rb.smpr2.modify(|_, w| unsafe { w.smp19().bits(self.sample_time.into()) }),
                        _ => unreachable!(),
                    }
                }

                fn convert(&mut self, chan: u8) -> u32 {
                    assert!(chan <= 19);
                    // Ensure that no conversions are ongoing
                    assert!(self.rb.cr.read().adstart().bit_is_clear() && self.rb.cr.read().jadstart().bit_is_clear());

                    // Select channel
                    self.rb.pcsel.modify(|r, w| unsafe { w.pcsel().bits(r.pcsel().bits() | (1 << chan)) });
                    self.set_chan_smps(chan);
                    self.rb.sqr1.modify(|_, w| unsafe { 
                        w.sq1().bits(chan)
                            .l3().bits(0)
                    });

                    // Perform conversion
                    self.rb.cr.modify(|_, w| w.adstart().set_bit());

                    // Wait until conversion finished
                    while self.rb.isr.read().eos().bit_is_clear() {}

                    // Cleanup
                    self.rb.pcsel.modify(|r, w| unsafe { w.pcsel().bits(r.pcsel().bits() & !(1 << chan)) });

                    // Retrieve result
                    let result = self.rb.dr.read().bits();
                    result
                }

                pub fn free(self) -> (&'a mut Delay, $ADC) {
                    (self.delay, self.rb)
                }
            }

            impl<WORD, PIN> OneShot<$ADC, WORD, PIN> for Adc<'_, $ADC>
            where
                WORD: From<u32>, 
                PIN: Channel<$ADC, ID = u8>,
            {
                type Error = ();

                fn read(&mut self, _pin: &mut PIN) -> nb::Result<WORD, Self::Error> {
                    self.power_up();
                    self.enable();
                    let res = self.convert(PIN::channel());
                    self.disable();
                    self.power_down();
                    Ok(res.into())
                }
            }
        )+
    }
}

adc_hal! {
    ADC1: (
        adc1,
        adc12en,
        adc12rst,
        AHB1
    ),
    ADC2: (
        adc2,
        adc12en,
        adc12rst,
        AHB1
    ),
    ADC3: (
        adc3,
        adc3en,
        adc3rst,
        AHB4
    ),
}

