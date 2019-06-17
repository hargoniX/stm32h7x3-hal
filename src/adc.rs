use hal::adc::{Channel, OneShot};
use hal::blocking::delay::DelayUs;

use stm32h7::stm32h7x3::{ADC1, ADC2, ADC3};

use crate::gpio::{Analog, AF0};
use crate::gpio::gpioa::{PA0, PA1, PA2, PA3, PA4, PA5, PA6, PA7};
use crate::gpio::gpiob::{PB0, PB1};
use crate::gpio::gpioc::{PC0, PC1, PC2, PC3, PC4, PC5};
use crate::gpio::gpiof::{PF3, PF4, PF5, PF6, PF7, PF8, PF9, PF10, PF11, PF12, PF13, PF14};
use crate::gpio::gpioh::{PH2, PH3, PH4, PH5};
use crate::delay::Delay;
use crate::rcc::{AHB1, AHB4, D3CCIPR};

pub struct Adc<ADC> {
    rb: ADC,
    sample_time: AdcSampleTime,
    resolution: AdcSampleResolution,
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[allow(non_camel_case_types)]

/// ADC sampling time
///
/// Options for the sampling time, each is T + 0.5 ADC clock cycles.
//
// Refer to RM0433 Rev 6 - Chapter 24.4.13
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

impl AdcSampleTime {
    pub fn default() -> Self {
        AdcSampleTime::T_32
    }
}

// Refer to RM0433 Rev 6 - Chapter 24.4.13
impl From<AdcSampleTime> for u8 {
    fn from(val: AdcSampleTime) -> u8 {
        match val {
            AdcSampleTime::T_1 => 0b000,
            AdcSampleTime::T_2 => 0b001,
            AdcSampleTime::T_8 => 0b010,
            AdcSampleTime::T_16 => 0b011,
            AdcSampleTime::T_32 => 0b100,
            AdcSampleTime::T_64 => 0b101,
            AdcSampleTime::T_387 => 0b110,
            AdcSampleTime::T_810 => 0b111,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[allow(non_camel_case_types)]

/// ADC sampling resolution
/// 
/// Options for sampling resolution
//
// Refer to RM0433 Rev 6 - Chapter 24.2
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

// Refer to RM0433 Rev 6 - Chapter 24.4.27 (Table 205)
impl From<AdcSampleResolution> for u8 {
    fn from(val: AdcSampleResolution) -> u8 {
        match val {
            AdcSampleResolution::B_16 => 0b000,
            AdcSampleResolution::B_14 => 0b001,
            AdcSampleResolution::B_12 => 0b010,
            AdcSampleResolution::B_10 => 0b011,
            // Revision model Y
            #[cfg(not(feature = "rev_v"))]
            AdcSampleResolution::B_8 => 0b100,
            // Revision model V
            #[cfg(feature = "rev_v")]
            AdcSampleResolution::B_8 => 0b111,
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
//
// Refer to DS12110 Rev 7 - Chapter 5 (Table 9)
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
pub struct StoredConfig(AdcSampleTime, AdcSampleResolution);

#[allow(unused_macros)]
macro_rules! adc_hal {
    ($(
        $ADC:ident: (
            $init:ident,
            $adcxen:ident,
            $adcxrst:ident,
            $AHB:ident
        )
    ),+ $(,)*) => {
        $(
            impl Adc<$ADC> {
                /// Init a new Adc
                ///
                /// Sets all configurable parameters to one-shot defaults,
                /// performs a boot-time calibration.
                pub fn $init(adc: $ADC, ahb: &mut $AHB, delay: &mut Delay, d3ccipr: &mut D3CCIPR) -> Self
                {
                    let mut s = Self {
                        rb: adc,
                        sample_time: AdcSampleTime::default(),
                        resolution: AdcSampleResolution::default(),
                    };
                    s.enable_clock(ahb, d3ccipr);
                    s.power_down();
                    s.reset(ahb);
                    s.configure();
                    s.power_up(delay);
                    s.disable();
                    s.calibrate();
                    s.enable();
                    s
                }

                /// Save current ADC config
                pub fn save_cfg(&mut self) -> StoredConfig {
                    StoredConfig(self.sample_time, self.resolution)
                }

                /// Restore saved ADC config
                pub fn restore_cfg(&mut self, cfg: StoredConfig) {
                    self.sample_time = cfg.0;
                    self.resolution = cfg.1;
                }

                /// Reset the ADC config to default, return existing config
                pub fn default_cfg(&mut self) -> StoredConfig {
                    let cfg = self.save_cfg();
                    self.sample_time = AdcSampleTime::default();
                    self.resolution = AdcSampleResolution::default();
                    cfg
                }

                /// Get ADC samping time
                pub fn get_sample_time(&self) -> AdcSampleTime {
                    self.sample_time
                }

                /// Get ADC sampling resolution
                pub fn get_resolution(&self) -> AdcSampleResolution {
                    self.resolution
                }

                /// Set ADC sampling time
                ///
                /// Options can be found in [AdcSampleTime](crate::adc::AdcSampleTime).
                pub fn set_sample_time(&mut self, t_samp: AdcSampleTime) {
                    self.sample_time = t_samp;
                }

                /// Set ADC sampling resolution
                /// 
                /// Options can be found in [AdcSampleResolution](crate::adc::AdcSampleResolution)
                pub fn set_resolution(&mut self, res: AdcSampleResolution) {
                    self.resolution = res;
                }

                /// Returns the largest possible sample value for the current settings
                pub fn max_sample(&self) -> u32 {
                    (1 << self.resolution as u32) - 1
                }

                /// Disables Deeppowerdown-mode and enables voltage regulator
                /// 
                /// Note: After power-up, a [`calibration`]: #method.calibrate shall be run
                //
                // Refer to RM0433 Rev 6 - Chapter 24.4.6
                pub fn power_up(&mut self, delay: &mut Delay) {
                    self.rb.cr.modify(|_, w| 
                        w.deeppwd().clear_bit()
                            .advregen().set_bit()
                    );
                    delay.delay_us(10_u8);
                }

                /// Enables Deeppowerdown-mode and disables voltage regulator
                /// 
                /// Note: This resets the [`calibration`]: #method.calibrate of the ADC
                //
                // Refer to RM0433 Rev 6 - Chapter 24.4.6
                pub fn power_down(&mut self) {
                    self.rb.cr.modify(|_, w| 
                        w.deeppwd().set_bit()
                            .advregen().clear_bit()
                    );
                }

                /// Turns ADC on
                //
                // Refer to RM0433 Rev 6 - Chapter 24.4.9
                pub fn enable(&mut self) {
                    self.rb.isr.modify(|_, w| w.adrdy().set_bit());
                    self.rb.cr.modify(|_, w| w.aden().set_bit());
                    while self.rb.isr.read().adrdy().bit_is_clear() {}
                    self.rb.isr.modify(|_, w| w.adrdy().set_bit());
                }

                /// Turns ADC off
                //
                // Refer to RM0433 Rev 6 - Chapter 24.4.9
                pub fn disable(&mut self) {
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
                    ahb.rstr().modify(|_, w| w.$adcxrst().clear_bit());
                }

                fn enable_clock(&mut self, ahb: &mut $AHB, d3ccipr: &mut D3CCIPR) {
                    d3ccipr.constrain().modify(|_, w| unsafe {w.adcsrc().bits(0b10)});
                    ahb.enr().modify(|_, w| w.$adcxen().set_bit());
                }

                fn configure(&mut self) {
                    // Single conversion mode, Software trigger
                    // Refer to RM0433 Rev 6 - Chapters 24.4.15, 24.4.19
                    self.rb.cfgr.modify(|_, w| unsafe {
                        w.cont().clear_bit()
                            .exten().bits(0b00)
                            .discen().set_bit()
                    });
                    // Enables boost mode since clock frequency > 20MHz
                    // 
                    // Refer to RM0433 Rev 6 - Chapter 24.4.3
                    self.rb.cr.modify(|_, w| w.boost().set_bit());
                }

                /// Calibrates the ADC in single channel mode
                /// 
                /// Note: The ADC must be disabled 
                //
                // Refer to RM0433 Rev 6 - Chapter 24.4.8
                pub fn calibrate(&mut self) {
                    self.check_calibration_conditions();
                    
                    // single channel (INNx equals to V_ref-)
                    self.rb.cr.modify(|_, w| 
                        w.adcaldif().clear_bit()
                            .adcallin().set_bit()
                    );
                    // calibrate
                    self.rb.cr.modify(|_, w| w.adcal().set_bit());
                    while self.rb.cr.read().adcal().bit_is_set() {}
                }

                fn check_calibration_conditions(&self) {
                    if self.rb.cr.read().aden().bit_is_set() {
                        panic!("Cannot start calibration when the ADC is enabled");
                    }
                    if self.rb.cr.read().deeppwd().bit_is_set() {
                        panic!("Cannot start calibration when the ADC is in deeppowerdown-mode");
                    }
                    if self.rb.cr.read().advregen().bit_is_clear() {
                        panic!("Cannot start calibration when the ADC voltage regulator is disabled");
                    }
                }

                fn set_chan_smp(&mut self, chan: u8) {
                    match chan {
                        // Couldn't find smp0 in smpr1 register so I need to manually write to that register
                        0 => self.rb.smpr1.modify(|r, w| unsafe { w.bits((r.bits() & !0b111) | self.sample_time as u32) }),
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

                // Refer to RM0433 Rev 6 - Chapter 24.4.16
                fn convert(&mut self, chan: u8) -> u32 {
                    assert!(chan <= 19);
                    // Ensure that no conversions are ongoing
                    assert!(self.rb.cr.read().adstart().bit_is_clear() && self.rb.cr.read().jadstart().bit_is_clear());  

                    // Set resolution
                    self.rb.cfgr.modify(|_, w| unsafe { w.res().bits(self.resolution.into()) });

                    // Select channel (with preselection, refer to RM0433 Rev 6 - Chapter 24.4.12)
                    self.rb.pcsel.modify(|r, w| unsafe { w.pcsel().bits(r.pcsel().bits() | (1 << chan)) });
                    self.set_chan_smp(chan);
                    self.rb.sqr1.modify(|_, w| unsafe { 
                        w.sq1().bits(chan)
                            .l3().bits(0)
                    });

                    // Perform conversion
                    self.rb.cr.modify(|_, w| w.adstart().set_bit());

                    // Wait until conversion finished
                    while self.rb.isr.read().eoc().bit_is_clear() {}

                    // Disable preselection of this channel, refer to RM0433 Rev 6 - Chapter 24.4.12
                    self.rb.pcsel.modify(|r, w| unsafe { w.pcsel().bits(r.pcsel().bits() & !(1 << chan)) });

                    // Retrieve result
                    let result = self.rb.dr.read().bits();
                    result
                }

            }

            impl<WORD, PIN> OneShot<$ADC, WORD, PIN> for Adc<$ADC>
            where
                WORD: From<u32>, 
                PIN: Channel<$ADC, ID = u8>,
            {
                type Error = ();

                fn read(&mut self, _pin: &mut PIN) -> nb::Result<WORD, Self::Error> {
                    let res = self.convert(PIN::channel());
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
