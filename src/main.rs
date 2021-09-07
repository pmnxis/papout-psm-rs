// RTIC v6 version.

#![no_std]
#![no_main]
#![allow(warnings)]

use panic_rtt_target as _panic_handler;

#[rtic::app(device = stm32g0xx_hal::stm32, peripherals = true)]
mod app {
    // use alloc::borrow::ToOwned;
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
        stm32::{EXTI, TIM14, TIM16, TIM17, USART2},
        timer::Timer,
    };

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
        BillDouble,
        NotEmit,
        LengthLong,
        LengthShort,
        RejOver,
        MotorLock,
        Incline,
        Ok, //?Maybe?
    }

    macro_rules! sign_u8 {
        ($foo: expr, $is_signed: expr) => {
            ($foo & 0x20) | (0x20 * (!$is_signed as u8))
        };
    }

    pub struct SerialTap {
        rx: Rx<USART2, BasicConfig>,
        buffer: [u8; 4],
        cnt: u8,
    }

    pub struct CommandAction {
        kind: CommandActionKind,
        data: u8,
    }

    pub struct ParallelInput {
        p_out_pulse: gpioa::PA7<Input<Floating>>,
        p_empty: gpioa::PA8<Input<Floating>>,
        p_error: gpioa::PA11<Input<Floating>>,
        prev_state: (bool, bool, bool, bool),
    }

    pub struct ParallelOutput {
        p_reset: gpioa::PA5<Output<PushPull>>,
        p_inhibit: gpioa::PA6<Output<PushPull>>,
    }

    // TypeCasting Internally for some pattern.
    type UartTx = Tx<USART2, BasicConfig>;
    type UartTxError = stm32g0xx_hal::serial::Error;

    pub struct MainTask {
        tx: UartTx,
    }

    pub struct PPulse200HzTask {
        p_pulse_timer: Timer<TIM16>,
        p_pulse: gpioa::PA4<Output<PushPull>>,
        tcnt: i16,
        data: i16,
    }

    /* resources shared across RTIC tasks */
    #[shared]
    struct Shared {
        /// the last observed position of the turret
        // Temporary use. (https://rtic.rs/dev/book/en/migration/migration_v5.html)
        tick: u32,
        ppulse_task: PPulse200HzTask,
    }

    /* resources local to specific RTIC tasks */
    #[local]

    struct Local {
        tick_timer: Timer<TIM17>,
        indicator: gpiob::PB9<Output<PushPull>>,
        heartbeat: gpiob::PB8<Output<PushPull>>,
        serial: SerialTap,
        p_in: ParallelInput,
        p_out: ParallelOutput,
        testpoint: gpiob::PB6<Output<PushPull>>,
        main_instance: MainTask,
    }

    #[init]
    #[allow(unused_mut)]
    fn init(ctx: init::Context) -> (Shared, Local, init::Monotonics) {
        let mut rcc = ctx.device.RCC.freeze(Config::hsi(Prescaler::NotDivided));
        let mut exti = ctx.device.EXTI;

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

        let mut ppulse_timer = ctx.device.TIM16.timer(&mut rcc);
        ppulse_timer.start(50.ms());
        // pio_timer.listen();

        let mut tick_timer = ctx.device.TIM17.timer(&mut rcc);
        tick_timer.start(1.ms());
        tick_timer.listen();

        let mut sharing: u32 = 0;

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

        (
            Shared {
                tick: sharing,
                ppulse_task: PPulse200HzTask {
                    p_pulse_timer: ppulse_timer,
                    p_pulse: gpioa.pa4.into_push_pull_output(),
                    tcnt: 0,
                    data: 0,
                },
            },
            Local {
                tick_timer: tick_timer,
                indicator: gpiob.pb9.into_push_pull_output(),
                heartbeat: gpiob.pb8.into_push_pull_output(),
                serial: SerialTap {
                    rx: uart_rx,
                    buffer: [0, 0, 0, 0],
                    cnt: 0,
                },
                p_out: ParallelOutput {
                    p_reset: gpioa.pa5.into_push_pull_output(),
                    p_inhibit: gpioa.pa6.into_push_pull_output(),
                },
                p_in: ParallelInput {
                    p_out_pulse: gpioa
                        .pa7
                        .into_floating_input()
                        .listen(SignalEdge::All, &mut exti),
                    p_empty: gpioa
                        .pa8
                        .into_floating_input()
                        .listen(SignalEdge::All, &mut exti),
                    p_error: gpioa
                        .pa11
                        .into_floating_input()
                        .listen(SignalEdge::All, &mut exti),
                    prev_state: (false, false, false, false),
                },

                testpoint: testpoint,
                main_instance: MainTask { tx: uart_tx },
            },
            init::Monotonics(),
        )
    }

    fn uart_write(tx: &mut UartTx, packet: (u8, u8, u8)) -> nb::Result<(), UartTxError> {
        // Follow Return type from "FullDuplex<Word>::send(&mut self, word: Word)"
        let array: [u8; 5] = [
            b'$',
            packet.0,
            packet.1,
            packet.2,
            packet.0 + packet.1 + packet.2,
        ];

        for byte in array {
            match tx.write(byte) {
                Err(uart_error) => {
                    return Err(uart_error);
                }
                Ok(_) => {}
            }
        }

        Ok(())
    }

    macro_rules! error_state_write {
        ($txd: expr, $byte3: expr, $is_signed: expr) => {
            uart_write(
                $txd,
                (
                    sign_u8!(b's', !$is_signed),
                    sign_u8!(b'e', !$is_signed),
                    $byte3,
                ),
            )
        };
    }

    #[idle(shared = [tick], local = [
        indicator, p_out, main_instance])]
    fn idle(mut ctx: idle::Context) -> ! {
        // Scratch
        let example_error = ErrorKind::Jam;
        let is_signed: bool = false;
        let rev_sign = !is_signed;
        let example_send = match example_error {
            ErrorKind::Empty => {
                error_state_write!(&mut ctx.local.main_instance.tx, 0x81, is_signed)
            }
            ErrorKind::Jam => {
                error_state_write!(&mut ctx.local.main_instance.tx, 0x82, is_signed)
            }
            ErrorKind::BillDouble => {
                error_state_write!(&mut ctx.local.main_instance.tx, 0x83, is_signed)
            }
            ErrorKind::NotEmit => {
                error_state_write!(&mut ctx.local.main_instance.tx, 0x84, is_signed)
            }
            ErrorKind::LengthLong => {
                error_state_write!(&mut ctx.local.main_instance.tx, 0x85, is_signed)
            }
            ErrorKind::LengthShort => {
                error_state_write!(&mut ctx.local.main_instance.tx, 0x86, is_signed)
            }
            ErrorKind::RejOver => {
                error_state_write!(&mut ctx.local.main_instance.tx, 0x87, is_signed)
            }
            ErrorKind::MotorLock => {
                error_state_write!(&mut ctx.local.main_instance.tx, 0x8a, is_signed)
            }
            ErrorKind::Incline => {
                error_state_write!(&mut ctx.local.main_instance.tx, 0x8e, is_signed)
            }
            // without macro, this is original pattern.
            ErrorKind::Ok => uart_write(
                &mut ctx.local.main_instance.tx,
                (sign_u8!(b's', rev_sign), sign_u8!(b'e', rev_sign), 0x80),
            ),
            // Check it's 0x80 is ok later.
            _ => Ok({}),
        };

        // End of scratch

        loop {
            // lock`
            let mut copied: u32 = 0;
            ctx.shared.tick.lock(|x| copied = u32::clone(x));
            // end of copy lock

            // ctx.local.indicator.toggle().unwrap();
            // rprintln!("Lambda : I need chewru : Shared : {}", copied);
            // block!(ctx.local.tick_timer.wait()).unwrap();

            // // 50ms
            // ctx.local.p_out.p_pulse.set_high();
            // ctx.local.p_out.p_reset.set_low();
            // ctx.local.p_out.p_inhibit.set_low();

            // block!(ctx.local.tick_timer.wait()).unwrap();

            // // 50ms
            // ctx.local.p_out.p_pulse.set_low();

            // block!(ctx.local.tick_timer.wait()).unwrap();

            // // 500ms
            // for _ in 0..(2000 / 50) {
            //     block!(ctx.local.tick_timer.wait()).unwrap();
            // }
        }
    }

    #[task(binds = TIM17, shared = [tick], local = [tick_timer])]
    fn timer_tick(mut ctx: timer_tick::Context) {
        ctx.shared.tick.lock(|x| *x = *x + 1);
        ctx.local.tick_timer.clear_irq();
    }

    #[task(binds = TIM16, shared = [ppulse_task])]
    fn p_pulse_200Hz_task(mut ctx: p_pulse_200Hz_task::Context) {
        ctx.shared.ppulse_task.lock(|task| {
            let valid_task = 0 < task.data;
            let wait_time = task.tcnt < 0;
            let toggle_run = task.tcnt < task.data;
            match (valid_task, wait_time, toggle_run) {
                (true, true, true) => {
                    task.p_pulse.set_low();
                }
                (true, false, true) => match (task.tcnt & 0b1 == 0) {
                    true => {
                        task.p_pulse.set_high();
                    }
                    false => {
                        task.p_pulse.set_low();
                    }
                },
                _ => {
                    task.p_pulse.set_low();
                    task.p_pulse_timer.unlisten();
                }
            }
            task.p_pulse_timer.clear_irq();
        });
    }

    #[task(binds = EXTI4_15, shared = [tick], local = [p_in])]
    fn parallel_input_handler(mut ctx: parallel_input_handler::Context) {
        // -- OBDL1000 [Active Low / Normal High]
        // -> 74hc4049 [Active High / Normal Low]
        // -> is_low() [Active Low / Normal High] Invert again
        let current_state = (
            ctx.local.p_in.p_out_pulse.is_low().unwrap(),
            ctx.local.p_in.p_empty.is_low().unwrap(),
            ctx.local.p_in.p_error.is_low().unwrap(),
        );
        let mut copied_tick: u32 = 0;
        ctx.shared.tick.lock(|x| copied_tick = u32::clone(x));

        if (current_state.0 != ctx.local.p_in.prev_state.0)
            || (false != ctx.local.p_in.prev_state.3)
        {}
        if (current_state.1 != ctx.local.p_in.prev_state.1)
            || (false != ctx.local.p_in.prev_state.3)
        {}
        if (current_state.2 != ctx.local.p_in.prev_state.2)
            || (false != ctx.local.p_in.prev_state.3)
        {}
    }

    #[task(binds = USART2, shared = [tick], local = [serial])]
    fn usart_isr(mut ctx: usart_isr::Context) {
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
            (Ok(b'$'), _) => {
                // Start String Force Match but "DxS" pattern cannot.
                if ctx.local.serial.cnt == 2 && (ctx.local.serial.buffer[1] & 0x20 == b'D') {
                    ctx.local.serial.buffer[ctx.local.serial.cnt as usize] = b'$';
                    ctx.local.serial.cnt + 1
                } else {
                    ctx.local.serial.buffer[0] = b'$';
                    0
                }
            }
            (Ok(byte), 4) => {
                // Filled
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
                    CommandActionKind::WrongCommand
                    | CommandActionKind::WrongStart
                    | CommandActionKind::WrongHash
                    | CommandActionKind::WrongStartHash => {
                        rprintln!("Serial : WrongCommand");
                    }
                    _ => {
                        rprintln!("Serial : YesCommand");
                    }
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
