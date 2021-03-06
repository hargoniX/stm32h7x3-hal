//! System Window Watchdog implementation

use stm32h7::stm32h7x3::WWDG;
use embedded_hal::watchdog::{Watchdog, WatchdogEnable};
use cast::u8;
use crate::time::MilliSeconds;
use crate::rcc::Clocks;
use crate::rcc::APB3;


/// Implements the System Window Watchdog
pub struct SystemWindowWatchdog {
    wwdg: WWDG,
    clocks: Clocks,
    down_counter: u8,
}


impl SystemWindowWatchdog {
    /// Returns a System Window Watchdog object with down_counter intialized to zero
    /// to indicate the clock has not been used yet
    pub fn new(wwdg: WWDG, clocks: Clocks, apb3: &mut APB3) -> Self {
        // enable the peripheral inside the APB3
        apb3.enr().modify(|_, w| w.wwdg1en().set_bit());
        SystemWindowWatchdog {wwdg, clocks, down_counter: 0}
    }
}

impl Watchdog for SystemWindowWatchdog {
    /// Feeds the watchdog in order to avoid a reset, only executes properly if the watchdog
    /// has already been started or the down_counter is not 0 anymore
    fn feed(&mut self) {
        // if this value is 0 it is assumed that the watchdog has not yet been started
        assert!(self.down_counter != 0);
        self.wwdg.cr.modify(|_, w| w.t().bits(self.down_counter));
    }
}

impl WatchdogEnable for SystemWindowWatchdog {
    type Time = MilliSeconds;
    /// Starts the watchdog with a given timeout period, if this period is out of bounds the function
    /// is going to panic
    fn start<T>(&mut self, period: T) 
    where
        T: Into<MilliSeconds>,
    {
        let period = period.into().0;
        let maximum = (4096 * 2u32.pow(7) * 64) / (self.clocks.pclk3().0 / 1000);
        assert!(period <= maximum);

        // cant approximate this at compile time as the apb clock frequency is not known at compile time
        // TODO: find a better way for this
        let mut best_config: (u32, u32) = (0, 0);
        let mut closest:u32 = 33334;
        for wdgtb in 0..8 {
            for t in 1..64 {
                // timeout = pclk * 4096 * 2^WDGTB[2:0] * (t[5:0] +1)
                let current_timeout =(4096 * (1 << wdgtb) * (t + 1))/(self.clocks.pclk3().0 / 1000);
                if period > current_timeout {
                    let difference = period - current_timeout;
                    if difference < closest {
                        closest = difference;
                        best_config = (wdgtb, t);
                    }
                }
            }
        }

        let wdgtb = u8(best_config.0).unwrap();
        self.down_counter = u8(best_config.1).unwrap() | (1<<6);

        // write the config values, matching the set timeout the most
        self.wwdg.cfr.modify(|_, w| w.wdgtb().bits(wdgtb));
        self.wwdg.cfr.modify(|_, w| w.w().bits(self.down_counter));
        self.wwdg.cr.modify(|_, w| w.t().bits(self.down_counter));
        // enable the watchdog
        self.wwdg.cr.modify(|_, w| w.wdga().set_bit());
    }
}
