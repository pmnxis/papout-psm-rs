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
        rcc::*,
        serial::*,
        // stm::{NVIC vector list what you use.}
        stm32::{TIM16, TIM17, USART2},
        timer::Timer,
    };

    /* resources shared across RTIC tasks */
    #[shared]
    struct Shared {
        /// the last observed position of the turret
        // Temporary use. (https://rtic.rs/dev/book/en/migration/migration_v5.html)
        shared_integer: i32,
    }

    /* resources local to specific RTIC tasks */
    #[local]
    struct Local {
        indicator_timer: Timer<TIM17>,
        indicator: gpiob::PB9<Output<PushPull>>,
        heartbeat_timer: Timer<TIM16>,
        heartbeat: gpiob::PB8<Output<PushPull>>,
        serial: Serial<USART2, FullConfig>,
    }

    #[init]
    #[allow(unused_mut)]
    fn init(ctx: init::Context) -> (Shared, Local, init::Monotonics) {
        rtt_init_print!();
        rprintln!("LambdaEE!");

        let mut rcc = ctx.device.RCC.freeze(Config::hsi(Prescaler::NotDivided));

        let mut gpioa = ctx.device.GPIOA.split(&mut rcc);
        let gpiob = ctx.device.GPIOB.split(&mut rcc);

        let mut heartbeat_timer = ctx.device.TIM16.timer(&mut rcc);
        heartbeat_timer.start(500.ms());
        heartbeat_timer.listen();

        let mut indicator_timer = ctx.device.TIM17.timer(&mut rcc);
        indicator_timer.start(500.ms());

        let mut sharing: i32 = 0;

        let mut usart2 = ctx
            .device
            .USART2
            .usart(
                gpioa.pa2,
                gpioa.pa3,
                FullConfig::default()
                    .baudrate(9600.bps())
                    .fifo_enable()
                    .rx_fifo_enable_interrupt()
                    .rx_fifo_threshold(FifoThreshold::FIFO_4_BYTES),
                &mut rcc,
            )
            .unwrap();

        (
            Shared {
                shared_integer: sharing,
            },
            Local {
                indicator_timer: indicator_timer,
                indicator: gpiob.pb9.into_push_pull_output(),
                heartbeat_timer: heartbeat_timer,
                heartbeat: gpiob.pb8.into_push_pull_output(),
                serial: usart2,
            },
            init::Monotonics(),
        )
    }

    #[idle(shared = [shared_integer], local = [indicator, indicator_timer])]
    fn idle(mut ctx: idle::Context) -> ! {
        loop {
            // lock`
            let mut copied: i32 = 0;
            ctx.shared.shared_integer.lock(|x| copied = i32::clone(x));
            // end of copy lock

            ctx.local.indicator.toggle().unwrap();
            rprintln!("Lambda : I need chewru : Shared : {}", copied);
            block!(ctx.local.indicator_timer.wait()).unwrap();
        }
    }

    #[task(binds = TIM16, local = [heartbeat, heartbeat_timer])]
    fn timer_tick(ctx: timer_tick::Context) {
        ctx.local.heartbeat.toggle().unwrap();
        ctx.local.heartbeat_timer.clear_irq();
    }

    #[task(binds = USART2, shared = [shared_integer], local = [serial])]
    fn usart_isr(mut ctx: usart_isr::Context) {
        ctx.shared.shared_integer.lock(|x| *x = *x + 1);
        match ctx.local.serial.read() {
            Err(nb::Error::WouldBlock) => {
                // no more data available in fifo
                // Nothing to do
            }
            Err(nb::Error::Other(_err)) => {
                // Handle other error Overrun, Framing, Noise or Parity
                rprintln!("Serial : Error-Other");
            }
            Ok(byte) => {
                rprintln!("Serial : {}", byte);
            }
        }
    }
}
