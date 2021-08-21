#![deny(warnings)]
#![deny(unsafe_code)]
#![no_main]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_rt as rt;
extern crate nb;
extern crate panic_halt;
extern crate stm32g0xx_hal as hal;

use hal::prelude::*;
use hal::stm32;
use nb::block;
use rt::entry;

#[entry]
fn main() -> ! {
    let dp = stm32::Peripherals::take().expect("cannot take peripherals");
    let mut rcc = dp.RCC.constrain();

    let gpioc = dp.GPIOC.split(&mut rcc);
    let mut led = gpioc.pc6.into_push_pull_output();

    let mut timer = dp.TIM17.timer(&mut rcc);
    timer.start(500.ms());

    loop {
        led.toggle().unwrap();
        block!(timer.wait()).unwrap();
    }
}
