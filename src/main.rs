#![no_std]
#![no_main]
#![deny(warnings)]

extern crate cortex_m;
extern crate cortex_m_rt as rt;
extern crate panic_semihosting;
extern crate rtic;
extern crate nb;
extern crate stm32g0xx_hal as hal;

use hal::gpio;
use hal::gpio::{Output, PushPull};
use hal::prelude::*;
// use hal::rtc::Rtc;
// use hal::stm32::{self, Interrupt};
use hal::stm32::{self};
use hal::rcc::Config;
use hal::rcc::Prescaler;
use hal::timer::Timer;
use nb::block;

#[rtic::app(device = hal::stm32, peripherals = true)]
const APP: () = {
    struct Resources {
        indicator_timer: Timer<stm32::TIM17>,
        indicator: gpio::gpiob::PB9<Output<PushPull>>,
        heartbeat_timer: Timer<stm32::TIM16>,
        heartbeat: gpio::gpiob::PB8<Output<PushPull>>,
    }

    #[init]
    #[allow(unused_mut)]
    fn init(mut ctx: init::Context) -> init::LateResources {
    let mut rcc = ctx.device.RCC.freeze(
        Config::hsi(Prescaler::NotDivided));

    let gpiob = ctx.device.GPIOB.split(&mut rcc);

    let mut heartbeat_timer = ctx.device.TIM16.timer(&mut rcc);
    heartbeat_timer.start(500.ms());
    heartbeat_timer.listen();

    
    let mut indicator_timer = ctx.device.TIM17.timer(&mut rcc);
    indicator_timer.start(500.ms());


        init::LateResources {
            indicator_timer,
            indicator: gpiob.pb9.into_push_pull_output(),
            heartbeat_timer,
            heartbeat: gpiob.pb8.into_push_pull_output(),
        }
    }

    #[idle(resources = [indicator, indicator_timer])]
    fn idle(ctx: idle::Context) -> ! {
        loop {
            ctx.resources.indicator.toggle().unwrap();
            block!(ctx.resources.indicator_timer.wait()).unwrap();
        }
    }

    #[task(binds = TIM16, resources = [heartbeat, heartbeat_timer])]
    fn timer_tick(ctx: timer_tick::Context) {
        ctx.resources.heartbeat.toggle().unwrap();
        ctx.resources.heartbeat_timer.clear_irq();
    }
};
