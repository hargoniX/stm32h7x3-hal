use stm32h7::stm32h7x3::{rcc, RCC};
use cast::{u8, u16};
use crate::time::Hertz;
use crate::flash::ACR;

/// Extension trait that constrains the `RCC` peripheral
pub trait RccExt {
    /// Constrains the `RCC` peripheral so it plays nicely with the other abstractions
    fn constrain(self) -> Rcc;
}

impl RccExt for RCC {
    fn constrain(self) -> Rcc {
        Rcc {
            ahb1: AHB1 { _0: ()},
            ahb2: AHB2 { _0: ()},
            ahb3: AHB3 { _0: ()},
            ahb4: AHB4 { _0: ()},
            apb1h: APB1H { _0: ()},
            apb1l: APB1L { _0: ()},
            apb2: APB2 { _0: ()},
            apb3: APB3 { _0: ()},
            apb4: APB4 { _0: ()},
            cfgr: CFGR {
                hclk1: None,
                hclk2: None,
                hclk3: None,
                hclk4: None,
                pclk1: None,
                pclk2: None,
                pclk3: None,
                pclk4: None,
                sys_ck: None,
                divp: None,
                divn: None,
                divm: None,
            }    
        }
    }
}

/// Constrained RCC peripheral
pub struct Rcc {
    /// AMBA High-performance Bus 1 (AHB) registers 
    pub ahb1: AHB1,
    /// AMBA High-performance Bus 2 (AHB) registers
    pub ahb2: AHB2,
    /// AMBA High-performance Bus 3 (AHB) registers
    pub ahb3: AHB3,
    /// AMBA High-performance Bus 4 (AHB) registers
    pub ahb4: AHB4,
    /// Advanced Peripheral Bus 1 L (APB1) registers
    pub apb1l: APB1L,
    /// Advanced Peripheral Bus 1 H (APB1) registers
    pub apb1h: APB1H,
    /// Advanced Peripheral Bus 2 (APB1) registers
    pub apb2: APB2,
    /// Advanced Peripheral Bus 3 (APB1) registers
    pub apb3: APB3,
    /// Advanced Peripheral Bus 4 (APB1) registers
    pub apb4: APB4,
    /// Clock configuration
    pub cfgr: CFGR,
}

macro_rules! ahb {
    ($($AHBx:ident: ($ahbx:ident, $AHBXENR:ident, $ahbxenr:ident, $AHBXRSTR:ident, $ahbxrstr:ident),)+) => {
        $(
            pub struct $AHBx {
                _0: (),
            }

            impl $AHBx {
                pub(crate) fn enr(&mut self) -> &rcc::$AHBXENR {
                    unsafe {&(*RCC::ptr()).$ahbxenr}
                }

                pub(crate) fn rstr(&mut self) -> &rcc::$AHBXRSTR {
                    unsafe {&(*RCC::ptr()).$ahbxrstr}
                }
            }
        )+
    };
}

macro_rules! apb {
    ($($APBx:ident: ($apbx:ident, $APBXENR:ident, $apbxenr:ident, $APBXRSTR:ident, $apbxrstr:ident),)+) => {
        $(
            pub struct $APBx {
                _0: (),
            }

            impl $APBx {
                pub(crate) fn enr(&mut self) -> &rcc::$APBXENR {
                    unsafe {&(*RCC::ptr()).$apbxenr}
                }

                pub(crate) fn rstr(&mut self) -> &rcc::$APBXRSTR {
                    unsafe {&(*RCC::ptr()).$apbxrstr}
                }
            }
        )+
    };
}

ahb!(
    AHB1: (abh1, AHB1ENR, ahb1enr, AHB1RSTR, ahb1rstr),
    AHB2: (ahb2, AHB2ENR, ahb2enr, AHB2RSTR, ahb2rstr),
    AHB3: (ahb3, AHB3ENR, ahb3enr, AHB3RSTR, ahb3rstr),
    AHB4: (ahb4, AHB4ENR, ahb4enr, AHB4RSTR, ahb4rstr),
);

apb!(
    APB1H: (apb1h, APB1HENR, apb1henr, APB1HRSTR, apb1hrstr),
    APB1L: (apb1l, APB1LENR, apb1lenr, APB1LRSTR, apb1lrstr),
    APB2: (apb2, APB2ENR, apb2enr, APB2RSTR, apb2rstr),
    APB3: (apb3, APB3ENR, apb3enr, APB3RSTR, apb3rstr),
    APB4: (apb4, APB4ENR, apb4enr, APB4RSTR, apb4rstr),
);

const HSI: u32 = 64_000_000; // Hz

/// Clock configuration
pub struct CFGR {
    hclk1: Option<u32>,
    hclk2: Option<u32>,
    hclk3: Option<u32>,
    hclk4: Option<u32>,
    pclk1: Option<u32>,
    pclk2: Option<u32>,
    pclk3: Option<u32>,
    pclk4: Option<u32>,
    sys_ck: Option<u32>,
    divm: Option<u32>,
    divn: Option<u32>,
    divp: Option<u32>,
}

impl CFGR {
    /// Sets a frequency for AHB1 bus
    pub fn hclk1<F>(mut self, freq: F) -> Self
    where 
        F: Into<Hertz>,
    {
        self.hclk1 = Some(freq.into().0);
        self
    }

    /// Sets a frequency for AHB2 bus
    pub fn hclk2<F>(mut self, freq: F) -> Self
    where 
        F: Into<Hertz>,
    {
        self.hclk2 = Some(freq.into().0);
        self
    }

    /// Sets a frequency for AHB3 bus
    pub fn hclk3<F>(mut self, freq: F) -> Self
    where 
        F: Into<Hertz>,
    {
        self.hclk3 = Some(freq.into().0);
        self
    }

    /// Sets a frequency for AHB4 bus
    pub fn hclk4<F>(mut self, freq: F) -> Self
    where 
        F: Into<Hertz>,
    {
        self.hclk4 = Some(freq.into().0);
        self
    }

    /// Sets a frequency for the APB1 bus
    pub fn pclk1<F>(mut self, freq: F) -> Self
    where
        F: Into<Hertz>,
    {
        self.pclk1 = Some(freq.into().0);
        self
    }

    /// Sets a frequency for the APB2 bus
    pub fn pclk2<F>(mut self, freq: F) -> Self
    where
        F: Into<Hertz>,
    {
        self.pclk2 = Some(freq.into().0);
        self
    }

    /// Sets a frequency for the APB3 bus
    pub fn pclk3<F>(mut self, freq: F) -> Self
    where
        F: Into<Hertz>,
    {
        self.pclk3 = Some(freq.into().0);
        self
    }

    /// Sets a frequency for the APB4 bus
    pub fn pclk4<F>(mut self, freq: F) -> Self
    where
        F: Into<Hertz>,
    {
        self.pclk4 = Some(freq.into().0);
        self
    }

    /// Sets the value for the registers used for sys_ck generation
    /// This function is expected to be used with values from the
    /// calc_config macro for now
    /// TODO: implement the calc_config macro using a const fn onc
    /// const fns support iteration
    pub fn sys_ck(mut self, divm: u32, divn: u32, divp:u32) -> Self
    {
        assert!(divm > 0 && divm < 64, "divm value was out of bounds");
        assert!(divn > 2 && divn < 513, "divn value was out of bounds");
        assert!(divp > 1 && divp < 129 && divp % 2 == 0, "divp value was out of bounds");
        self.divm = Some(divm);
        self.divp = Some(divp);
        self.divn = Some(divn);
        let ref_ck = HSI/divm;
        assert!(ref_ck > 1_000_000 && ref_ck < 16_000_000, "illegal config values for ref_ck");
        let pll_p_ck = (ref_ck * divn) / divp;
        assert!(pll_p_ck < 400_000_000, "illegal config values for pll_p_ck");
        self.sys_ck = Some(pll_p_ck);
        self
    }

    /// Freezes the clock configuration, making it effective
    pub fn freeze(self, acr: &mut ACR) -> Clocks {
        let mut sys_ck = self.sys_ck.unwrap_or(HSI);
        let rcc = unsafe { &*RCC::ptr()};
        
        // set the system clock
        let pll_frequency = if sys_ck == HSI {
            None
        }
        else {
            // this calculates the sys_ck frequency generated from the pll with the given config values
            // for closer details check the clock tree in the reference manual at page 323
            let frequency = ((HSI / self.divm.unwrap_or(0b100000)) * self.divn.unwrap_or(0x080)) / self.divp.unwrap_or(0b0000001);
            sys_ck = frequency;
            Some(frequency)
        };
        
        // Calculate the hpre divider value
        // As hclk 1,2,3 and 4 are generated from the same source we just need one value
        let hclk = self.hclk1.unwrap_or(self.hclk2.unwrap_or(self.hclk3.unwrap_or(self.hclk4.unwrap_or(if sys_ck > 240_000_000 {sys_ck/2} else {sys_ck}))));
        
        let hpre_bits: u8 = match sys_ck / hclk {
            0 => unreachable!(),
            1 => 0b0111,
            2 => 0b1000,
            3...5 => 0b1001,
            6...9 => 0b1010,
            10...16 => 0b1011,
            17...64 => 0b1100,
            65...128 => 0b1101,
            129...256 => 0b1110,
            257...512 => 0b1111,
            _ => 0b1111 
        };
        let hpre = 1 << (hpre_bits - 0b0111);
        let hclk = sys_ck / hpre;
        
        // set the hpre value
        //rcc.d1cfgr.modify(|_, w| unsafe {w.hpre().bits(u8(hpre).unwrap())});

        // adjust flash wait states
        // as VOS3 is the default VOS used only the values for VOS3 are implemented here
        let acr_config: (u8, u8) = match hclk {
            0...45_000_000 => (0, 0),
            45_000_001...90_000_000 => (1, 1),
            90_000_001...135_000_000 => (2, 1),
            135_000_001...180_000_000 => (3, 2),
            180_000_001...225_000_000 => (4, 2),
             _ => unreachable!(),
        };
        
        // calculate d1ppre
        let d1ppre_bits: u8 = match hclk / self.pclk3.unwrap_or(hclk) {
            0 => unreachable!(),
            1 => 0b011,
            2 => 0b100,
            3...4 => 0b101,
            5...8 => 0b110,
            9...16 => 0b111,
            _ => 0b111,
        };
        let d1ppre = 1 << (d1ppre_bits - 0b011);
        let pclk3 = hclk / d1ppre;

        // calculate d2ppre1
        let d2ppre1_bits: u8 = match hclk / self.pclk1.unwrap_or(hclk) {
            0 => unreachable!(),
            1 => 0b011,
            2 => 0b100,
            3...4 => 0b101,
            5...8 => 0b110,
            9...16 => 0b111,
            _ => 0b111,
        };
        let d2ppre1 = 1 << (d2ppre1_bits - 0b011);
        let pclk1 = hclk / d2ppre1;

        // calculate d2ppre2
        let d2ppre2_bits: u8 = match hclk / self.pclk2.unwrap_or(hclk) {
            0 => unreachable!(),
            1 => 0b011,
            2 => 0b100,
            3...4 => 0b101,
            5...8 => 0b110,
            9...16 => 0b111,
            _ => 0b111,
        };
        let d2ppre2 = 1 << (d2ppre2_bits - 0b011);
        let pclk2 = hclk / d2ppre2;

        //calculate d3ppre
        let d3ppre_bits: u8 = match hclk / self.pclk4.unwrap_or(hclk) {
            0 => unreachable!(),
            1 => 0b011,
            2 => 0b100,
            3...4 => 0b101,
            5...8 => 0b110,
            9...16 => 0b111,
            _ => 0b111,
        };
        let d3ppre = 1 << (d3ppre_bits - 0b011);
        let pclk4 = hclk / d3ppre;

        // write the flash wait states
        acr.acr().modify(|_, w| unsafe {w.latency().bits(acr_config.0).wrhighfreq().bits(acr_config.1)});

        // set the hpre value
        rcc.d1cfgr.modify(|_, w| unsafe {w.hpre().bits(hpre_bits)});
        
        // set all the AHB prescaler values
        rcc.d1cfgr.modify(|_, w| unsafe {
            w.d1ppre().bits(d1ppre_bits)
            
        });

        rcc.d2cfgr.modify(|_, w| unsafe {
            w
            .d2ppre1().bits(d2ppre1_bits)
            .d2ppre2().bits(d2ppre2_bits)
        });

        rcc.d3cfgr.modify(|_, w| unsafe {
            w.d3ppre().bits(d3ppre_bits)
        });

        // adjust sys_ck source
        if pll_frequency .is_some() {
            // use pll as sys_ck
            
            // set HSI as pll source
            rcc.pllckselr.modify(|_, w| unsafe {w.pllsrc().bits(00)});

            // set DIVN1
            rcc.pll1divr.modify(|_, w| unsafe { w.divn1().bits(u16(self.divn.unwrap_or(0x080)).unwrap())});

            // set divm1 value, set to default if not set by software
            rcc.pllckselr.modify(|_, w| unsafe{ w.divm1().bits(u8(self.divm.unwrap_or(0b100000)).unwrap())});

            // enable and set DIVP1
            rcc.pllcfgr.modify(|_, w| w.divp1en().set_bit());

            //disable frac mode of pll1
            rcc.pllcfgr.modify(|_, w| w.pll1fracen().clear_bit());

            let ref_ck = HSI / self.divm.unwrap_or(0b100000);

            // calculate and set the bits for the RGE register
            let rge_bits = match ref_ck  {
                1_000_000...2_000_000 => 0b00,
                2_000_001...4_000_000 => 0b01,
                4_000_001...8_000_000 => 0b10,
                8_000_001...16_000_000 => 0b11,
                _ => unreachable!(),
            };
            rcc.pllcfgr.modify(|_, w| unsafe{ w.pll1rge().bits(rge_bits)});

            // calculate and set the bits for the VCOSEL register
            // if the frequency of ref_ck is < 2 Mhz and > 1 Mhz set to 1 otherwise to 0
            let mut vcosel_bit = false;
            if ref_ck < 2_000_000 {
                vcosel_bit = true;
            }
            rcc.pllcfgr.modify(|_, w| w.pll1vcosel().bit(vcosel_bit));

            rcc.pll1divr.modify(|_, w| unsafe {w.divp1().bits(u8(self.divp.unwrap_or(0b0000001)).unwrap())});

            // enable pll1 and wait until its ready
            rcc.cr.modify(|_, w| w.pll1on().set_bit());
            while rcc.cr.read().pll1rdy().bit_is_set() {}

            // set pll1_p_ck as sys_ck
            rcc.cfgr.modify(|_, w| unsafe {w.sw().bits(0b011)});

            // wait until the clock switch is done
            while rcc.cfgr.read().sws().bits() != 0b011 {}
        }
        else {
            // use HSI AS CLOCK SOURCE
            // usually this value is set to what we write to it by default but you never know
            rcc.cfgr.modify(|_, w| unsafe {w.sw().bits(0b000)});
            while rcc.cfgr.read().sws().bits() != 0b000 {}
        }

        Clocks {
            sys_ck: Hertz(sys_ck),
            hclk1: Hertz(hclk),
            hclk2: Hertz(hclk),
            hclk3: Hertz(hclk),
            hclk4: Hertz(hclk),
            pclk1: Hertz(pclk1),
            pclk2: Hertz(pclk2),
            pclk3: Hertz(pclk3),
            pclk4: Hertz(pclk4),
            hpre: u8(hpre).unwrap(),
            d1ppre: u8(d1ppre).unwrap(),
            d2ppre1: u8(d2ppre1).unwrap(),
            d2ppre2: u8(d2ppre2).unwrap(),
            d3ppre: u8(d3ppre).unwrap(),
        }
    }
}

/// Frozen clock frequencies
///
/// The existence of this value indicates that the clock configuration can no longer be changed
#[derive(Clone, Copy)]
pub struct Clocks {
    sys_ck: Hertz,
    pclk1: Hertz,
    pclk2: Hertz,
    pclk3: Hertz,
    pclk4: Hertz,
    hclk1: Hertz,
    hclk2: Hertz,
    hclk3: Hertz,
    hclk4: Hertz,
    hpre: u8,
    d1ppre: u8,
    d2ppre1: u8,
    d2ppre2: u8,
    d3ppre: u8,
}


impl Clocks {
    pub fn sys_ck(&self) -> Hertz {
        self.sys_ck
    }

    pub fn pclk1(&self) -> Hertz {
        self.pclk1
    }

    pub fn pclk2(&self) -> Hertz {
        self.pclk2
    }

    pub fn pclk3(&self) -> Hertz {
        self.pclk3
    }

    pub fn pclk4(&self) -> Hertz {
        self.pclk4
    }

    pub fn hclk1(&self) -> Hertz {
        self.hclk1
    }

    pub fn hclk2(&self) -> Hertz {
        self.hclk2
    }

    pub fn hclk3(&self) -> Hertz {
        self.hclk3
    }

    pub fn hclk4(&self) -> Hertz {
        self.hclk4
    }

    pub fn d1ppre(&self) -> u8 {
        self.d1ppre
    }

    pub fn d2ppre1(&self) -> u8 {
        self.d2ppre1
    }

    pub fn d2ppre2(&self) -> u8 {
        self.d2ppre2
    }

    pub fn d3ppre(&self) -> u8 {
        self.d3ppre
    }
}