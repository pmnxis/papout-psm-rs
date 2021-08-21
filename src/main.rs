#![no_std]
#![no_main]
#![deny(warnings)]

extern crate cortex_m;
extern crate cortex_m_rt as rt;
extern crate panic_semihosting;
extern crate rtic;
extern crate stm32g0xx_hal as hal;

use cortex_m_semihosting::hprintln;
use hal::gpio;
use hal::gpio::{Output, PushPull};
use hal::prelude::*;
// use hal::rtc::Rtc;
// use hal::stm32::{self, Interrupt};
use hal::stm32::{self};
use hal::timer::Timer;
use rtic::app;

#[app(device = hal::stm32, peripherals = true)]
const APP: () = {
    struct Resources {
        heartbeat_timer: Timer<stm32::TIM16>,
        heartbeat: gpio::gpioc::PC6<Output<PushPull>>,
    }

    // Just borrowed from examples for now.

    #[init]
    fn init(ctx: init::Context) -> init::LateResources {
        let mut rcc = ctx.device.RCC.constrain();
        // rtic::pend(Interrupt::USART2);

        let gpioc = ctx.device.GPIOC.split(&mut rcc);

        let mut heartbeat_timer = ctx.device.TIM16.timer(&mut rcc);
        heartbeat_timer.start(3.hz());
        heartbeat_timer.listen();

        hprintln!("Hello Rust").unwrap();

        init::LateResources {
            heartbeat_timer,
            heartbeat: gpioc.pc6.into_push_pull_output(),
        }
    }

    #[task(binds = TIM16, resources = [heartbeat, heartbeat_timer])]
    fn timer_tick(ctx: timer_tick::Context) {
        ctx.resources.heartbeat.toggle().unwrap();
        ctx.resources.heartbeat_timer.clear_irq();
    }
};
