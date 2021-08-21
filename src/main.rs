// RTIC v6 version.

#![no_std]
#![no_main]
#![allow(warnings)]

use panic_rtt_target as _panic_handler;

#[rtic::app(device = stm32g0xx_hal::stm32, peripherals = true)]
mod app {
    /* bring dependencies into scope */
    use nb::block;
    use rtt_target::{rprintln, rtt_init_print};
    use stm32g0xx_hal::{
        gpio::*,
        prelude::*,
        rcc::{Config, Prescaler},
        stm32::{TIM16, TIM17},
        timer::Timer,
    };

    /* resources shared across RTIC tasks */
    #[shared]
    struct Shared {
        /// the last observed position of the turret
        shared_integer: u32,
    }

    /* resources local to specific RTIC tasks */
    #[local]
    struct Local {
        indicator_timer: Timer<TIM17>,
        indicator: gpiob::PB9<Output<PushPull>>,
        heartbeat_timer: Timer<TIM16>,
        heartbeat: gpiob::PB8<Output<PushPull>>,
    }

    #[init]
    #[allow(unused_mut)]
    fn init(ctx: init::Context) -> (Shared, Local, init::Monotonics) {
        rtt_init_print!();
        rprintln!("LambdaEE!");

        let mut rcc = ctx.device.RCC.freeze(Config::hsi(Prescaler::NotDivided));

        let gpiob = ctx.device.GPIOB.split(&mut rcc);

        let mut heartbeat_timer = ctx.device.TIM16.timer(&mut rcc);
        heartbeat_timer.start(500.ms());
        heartbeat_timer.listen();

        let mut indicator_timer = ctx.device.TIM17.timer(&mut rcc);
        indicator_timer.start(500.ms());

        let mut sharing: u32 = 0;

        (
            Shared {
                shared_integer: sharing,
            },
            Local {
                indicator_timer: indicator_timer,
                indicator: gpiob.pb9.into_push_pull_output(),
                heartbeat_timer: heartbeat_timer,
                heartbeat: gpiob.pb8.into_push_pull_output(),
            },
            init::Monotonics(),
        )
    }

    #[idle(local = [indicator, indicator_timer])]
    fn idle(ctx: idle::Context) -> ! {
        loop {
            ctx.local.indicator.toggle().unwrap();
            rprintln!("Lambda : I need chewru");
            block!(ctx.local.indicator_timer.wait()).unwrap();
        }
    }

    #[task(binds = TIM16, local = [heartbeat, heartbeat_timer])]
    fn timer_tick(ctx: timer_tick::Context) {
        ctx.local.heartbeat.toggle().unwrap();
        ctx.local.heartbeat_timer.clear_irq();
    }
}
