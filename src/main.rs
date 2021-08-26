// #![deny(warnings)]
#![deny(unsafe_code)]
#![no_main]
#![no_std]

extern crate cortex_m_rt as rt;
extern crate panic_halt;
extern crate stm32g0xx_hal as hal;

use core::fmt::Write;

use hal::prelude::*;
use hal::serial::*;
use hal::stm32;
use rt::entry;
use rtt_target::{rprintln, rtt_init_print};

#[entry]
fn main() -> ! {
    let dp = stm32::Peripherals::take().expect("cannot take peripherals");
    let mut rcc = dp.RCC.constrain();
    let gpioa = dp.GPIOA.split(&mut rcc);

    let mut usart2 = dp
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

    rtt_init_print!();
    rprintln!("Hello USART2\n");

    let (mut tx1, mut rx1) = usart2.split();

    let mut cnt = 0;
    loop {
        if rx1.fifo_threshold_reached() {
            loop {
                match rx1.read() {
                    Err(nb::Error::WouldBlock) => {
                        // no more data available in fifo
                        break;
                    }
                    Err(nb::Error::Other(_err)) => {
                        // Handle other error Overrun, Framing, Noise or Parity
                        rprintln!("nb::Error::Other(_err)) =>");
                    }
                    Ok(byte) => {
                        rprintln!("{}: {}\n", cnt, byte);
                        cnt += 1;
                    }
                }
            }
        }
    }
}
