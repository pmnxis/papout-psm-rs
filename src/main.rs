// RTIC v6 version.

#![no_std]
#![no_main]
#![allow(warnings)]

use panic_rtt_target as _panic_handler;

#[rtic::app(device = stm32g0xx_hal::stm32, peripherals = true)]
mod app {
    use core::convert::TryInto;
    use heapless::spsc::Queue;
    /* bring dependencies into scope */
    use nb::block;
    use rtt_target::{rprintln, rtt_init_print};
    use stm32g0xx_hal::{
        cortex_m::asm::delay,
        gpio::*,
        prelude::*,
        rcc::*,
        serial::*,
        // stm::{NVIC vector list what you use.}
        stm32::{TIM16, TIM17, USART2},
        timer::Timer,
    };

    pub struct SerialTap {
        rx: Rx<USART2, BasicConfig>,
        buffer: [u8; 4],
        cnt: u8,
    }

    pub enum CommandActionKind {
        SayHi,
        Init,
        Dispense,
        HaltAction,
        HaltActionCancel,
        RemoveCount,
        GetTotalDispensed,
        RemoveTotalCount,
        StateCheck,
        ErrorCheck,
        WrongCommand,
        WrongStart,
        WrongHash,
        WrongStartHash,
        WrongUnknown,
    }

    pub enum StateKind {
        Idle,
        WhileDispensing,
        ActionHalted,
        SuccessDispense,
        ProblemDispense,
    }

    pub enum ErrorKind {
        Empty,
        Jam,
        Double,
        NotEmit,
        LengthShort,
        LengthLong,
        NotSlide,
        MotorLock,
        Incline,
        Ok, //?Maybe?
    }

    pub struct CommandAction {
        kind: CommandActionKind,
        data: u8,
    }

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
        serial: SerialTap,
        output_pulse: gpioa::PA7<Output<PushPull>>,
        output_reset: gpioa::PA8<Output<PushPull>>,
        output_inhibit: gpioa::PA11<Output<PushPull>>,
        testpoint: gpiob::PB6<Output<PushPull>>,
    }

    #[init]
    #[allow(unused_mut)]
    fn init(ctx: init::Context) -> (Shared, Local, init::Monotonics) {
        let mut rcc = ctx.device.RCC.freeze(Config::hsi(Prescaler::NotDivided));

        let mut gpioa = ctx.device.GPIOA.split(&mut rcc);
        let gpiob = ctx.device.GPIOB.split(&mut rcc);

        // temporary
        let mut testpoint = gpiob.pb6.into_push_pull_output();
        testpoint.set_low();
        delay(10000);
        testpoint.set_high();
        // I don't know reason. for now MCU halt and restart.
        // end of temporary

        // Rtt Debug start.
        rtt_init_print!();
        rprintln!("LambdaEE!");
        // Rtt Debug setup end.

        let mut heartbeat_timer = ctx.device.TIM16.timer(&mut rcc);
        heartbeat_timer.start(500.ms());
        heartbeat_timer.listen();

        let mut indicator_timer = ctx.device.TIM17.timer(&mut rcc);
        indicator_timer.start(50.ms());

        let mut sharing: i32 = 0;

        let mut usart2 = ctx
            .device
            .USART2
            .usart(
                gpioa.pa2,
                gpioa.pa3,
                BasicConfig::default().baudrate(9600.bps()),
                &mut rcc,
            )
            .unwrap();

        usart2.listen(Event::Rxne);

        let (mut uart_tx, mut uart_rx) = usart2.split();

        let mut serial_tap = SerialTap {
            rx: uart_rx,
            buffer: [0, 0, 0, 0],
            cnt: 0,
        };

        (
            Shared {
                shared_integer: sharing,
            },
            Local {
                indicator_timer: indicator_timer,
                indicator: gpiob.pb9.into_push_pull_output(),
                heartbeat_timer: heartbeat_timer,
                heartbeat: gpiob.pb8.into_push_pull_output(),
                serial: serial_tap,
                output_pulse: gpioa.pa7.into_push_pull_output(),
                output_reset: gpioa.pa8.into_push_pull_output(),
                output_inhibit: gpioa.pa11.into_push_pull_output(),
                testpoint: testpoint,
            },
            init::Monotonics(),
        )
    }

    #[idle(shared = [shared_integer], local = [
        indicator, indicator_timer, output_pulse, output_reset, output_inhibit])]
    fn idle(mut ctx: idle::Context) -> ! {
        // Scratch
        let rx_packet: [u8; 4] = [0; 4];

        // End of scratch

        loop {
            // lock`
            let mut copied: i32 = 0;
            ctx.shared.shared_integer.lock(|x| copied = i32::clone(x));
            // end of copy lock

            ctx.local.indicator.toggle().unwrap();
            rprintln!("Lambda : I need chewru : Shared : {}", copied);
            block!(ctx.local.indicator_timer.wait()).unwrap();

            // 50ms
            ctx.local.output_pulse.set_high();
            ctx.local.output_reset.set_low();
            ctx.local.output_inhibit.set_low();

            block!(ctx.local.indicator_timer.wait()).unwrap();

            // 50ms
            ctx.local.output_pulse.set_low();

            block!(ctx.local.indicator_timer.wait()).unwrap();

            // 500ms
            for _ in 0..(2000 / 50) {
                block!(ctx.local.indicator_timer.wait()).unwrap();
            }
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
        ctx.local.serial.cnt = match (ctx.local.serial.rx.read(), ctx.local.serial.cnt) {
            (Err(nb::Error::WouldBlock), _) => {
                // no more data available in fifo
                // Nothing to do
                rprintln!("Serial : nb::Error::WouldBlock : {}", ctx.local.serial.cnt);
                ctx.local.serial.cnt
            }
            (Err(nb::Error::Other(_err)), _) => {
                // Handle other error Overrun, Framing, Noise or Parity
                rprintln!("Serial : Error-Other");
                0
            }
            (Ok(byte), 4) => {
                // Filled
                // rprintln!("Serial : {}", byte);
                rprintln!("Serial : Ok - {}", ctx.local.serial.cnt);

                let hash: u8 = ctx.local.serial.buffer[1]
                    + ctx.local.serial.buffer[2]
                    + ctx.local.serial.buffer[3];

                let serialTuple: (u8, u8, u8) = (
                    ctx.local.serial.buffer[1],
                    ctx.local.serial.buffer[2],
                    ctx.local.serial.buffer[3],
                );

                let parsed: CommandAction = match (ctx.local.serial.buffer[0], hash == byte) {
                    (b'$', true) => {
                        match serialTuple {
                            (b'H', b'I', b'?') => CommandAction {
                                kind: CommandActionKind::SayHi,
                                data: 0,
                            },
                            (b'I' | b'i', 0x00, 0x00) => CommandAction {
                                kind: CommandActionKind::Init,
                                data: 0,
                            },
                            (b'D', _, b'S') | (b'd', _, b's') => CommandAction {
                                kind: CommandActionKind::Dispense,
                                data: serialTuple.1,
                            },
                            (b'H' | b'h', 0x00, 0x00) => CommandAction {
                                kind: CommandActionKind::HaltAction,
                                data: 0,
                            },
                            (b'H', b'C', b'?') | (b'h', b'c', b'?') => CommandAction {
                                kind: CommandActionKind::HaltActionCancel,
                                data: 0,
                            },
                            (b'R', b'E', b'M') | (b'r', b'e', b'm') => CommandAction {
                                kind: CommandActionKind::RemoveCount,
                                data: 0,
                            },
                            (b'G', b'T', b'?') | (b'g', b't', b'?') => CommandAction {
                                kind: CommandActionKind::GetTotalDispensed,
                                data: 0,
                            },
                            (b'C', b'T', b'C') | (b'c', b't', b'c') => CommandAction {
                                kind: CommandActionKind::RemoveTotalCount,
                                data: 0,
                            },
                            (b'S' | b's', 0x00, 0x00) => CommandAction {
                                kind: CommandActionKind::StateCheck,
                                data: 0,
                            },
                            (b'S', b'E', b'R') | (b's', b'e', b'r') => CommandAction {
                                kind: CommandActionKind::ErrorCheck,
                                data: 0,
                            },
                            // default => (error)
                            _ => CommandAction {
                                kind: CommandActionKind::WrongCommand,
                                data: 0,
                            },
                        }
                    }
                    (_, true) => CommandAction {
                        kind: CommandActionKind::WrongStart,
                        data: 0,
                    },
                    (b'$', false) => CommandAction {
                        kind: CommandActionKind::WrongHash,
                        data: 0,
                    },
                    _ => CommandAction {
                        kind: CommandActionKind::WrongStartHash,
                        data: 0,
                    },
                };

                // Send to queue.
                match parsed.kind {
                    (CommandActionKind::WrongCommand
                    | CommandActionKind::WrongStart
                    | CommandActionKind::WrongHash
                    | CommandActionKind::WrongStartHash) => {}
                    _ => {}
                }

                0
            }
            (Ok(byte), 5) => {
                rprintln!("Serial : Buffer Is Full.");
                0
            }
            (Ok(byte), _) => {
                // Fill
                ctx.local.serial.buffer[ctx.local.serial.cnt as usize] = byte;
                ctx.local.serial.cnt + 1
            }
        }
    }
}
