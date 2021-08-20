#![no_std]
#![no_main]
#![deny(warnings)]

extern crate cortex_m;
extern crate cortex_m_rt as rt;
extern crate panic_semihosting;
extern crate rtic;
extern crate stm32g0xx_hal as hal;

use cortex_m_semihosting::hprintln;
use hal::exti::Event;
use hal::gpio;
use hal::gpio::{Output, PushPull, SignalEdge};
use hal::prelude::*;
// use hal::rtc::Rtc;
// use hal::stm32::{self, Interrupt};
use hal::stm32::{self};
use hal::timer::Timer;
use rtic::app;

#[app(device = hal::stm32, peripherals = true)]
const APP: () = {
    struct Resources {
        exti: stm32::EXTI,
        exti_indicator: gpio::gpiob::PB9<Output<PushPull>>,
        heartbeat_timer: Timer<stm32::TIM16>,
        heartbeat: gpio::gpiob::PB8<Output<PushPull>>,
    }

    // Just borrowed from examples for now.

    #[init]
    fn init(mut ctx: init::Context) -> init::LateResources {
        let mut rcc = ctx.device.RCC.constrain();
        // rtic::pend(Interrupt::USART2);

        let gpioa = ctx.device.GPIOA.split(&mut rcc);
        let gpiob = ctx.device.GPIOB.split(&mut rcc);

        let mut heartbeat_timer = ctx.device.TIM16.timer(&mut rcc);
        heartbeat_timer.start(3.hz());
        heartbeat_timer.listen();

        gpioa.pa4.listen(SignalEdge::All, &mut ctx.device.EXTI);
        // gpioa.pa5.listen(SignalEdge::All, &mut ctx.device.EXTI);
        // gpioa.pa6.listen(SignalEdge::All, &mut ctx.device.EXTI);

        // let mut rtc = ctx.device.RTC.constrain(&mut rcc);

        hprintln!("Hello Rust").unwrap();

        init::LateResources {
            heartbeat_timer,
            exti: ctx.device.EXTI,
            exti_indicator: gpiob.pb9.into_push_pull_output(),
            heartbeat: gpiob.pb8.into_push_pull_output(),
        }
    }

    #[task(binds = TIM16, resources = [heartbeat, heartbeat_timer])]
    fn timer_tick(ctx: timer_tick::Context) {
        ctx.resources.heartbeat.toggle().unwrap();
        ctx.resources.heartbeat_timer.clear_irq();
    }

    #[task(binds = EXTI4_15, resources = [exti, exti_indicator])]
    fn button_click(ctx: button_click::Context) {
        hprintln!("Button pressed").unwrap();
        ctx.resources.exti.unpend(Event::GPIO14);
        ctx.resources.exti_indicator.toggle().unwrap();
    }
};
