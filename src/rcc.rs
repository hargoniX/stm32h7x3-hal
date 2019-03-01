use stm32h7::stm32h7x3::{rcc, RCC};

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
            cfgr: CFGR{}
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

/// TODO
pub struct CFGR {

}