use rtic::rtic_monotonic::{
    embedded_time::{clock::Error, fraction::Fraction},
    Clock, Instant, Monotonic,
};
use stm32g0xx_hal::{cortex_m::asm::delay, rcc::*, rtc::*};

pub struct RtcMonotonic {
    rtc: Rtc,
}

impl RtcMonotonic {
    pub fn new(rtc: Rtc) -> RtcMonotonic {
        RtcMonotonic { rtc: rtc }
    }

    pub fn tim_now(&self) -> u32 {
        // get value
        self.rtc.rb.tr.read().bits()
    }
}

impl Clock for RtcMonotonic {
    type T = u32;

    /* LSI Freq <Vdd=3.0V, Temp^A = 30 DegreeC> : Min 31040Hz ~ 32960Hz */
    const SCALING_FACTOR: Fraction = Fraction::new(1, 32000);

    fn try_now(&self) -> Result<Instant<Self>, Error> {
        Ok(Instant::new(self.tim_now()))
    }
}

impl Monotonic for RtcMonotonic {
    const DISABLE_INTERRUPT_ON_EMPTY_QUEUE: bool = true;

    unsafe fn reset(&mut self) {
        // TODO
    }

    fn set_compare(&mut self, instant: &Instant<Self>) {
        // TODO
    }

    fn clear_compare_flag(&mut self) {
        // NOOP with SysTick interrupt
    }
}
