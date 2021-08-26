// #![deny(warnings)]
#![deny(unsafe_code)]
#![no_main]
#![no_std]

extern crate cortex_m_rt as rt;
extern crate panic_halt;
extern crate stm32g0xx_hal as hal;

use hal::prelude::*;
use hal::serial::*;
use hal::stm32;
use nb::block;
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
            BasicConfig::default().baudrate(9600.bps()),
            &mut rcc,
        )
        .unwrap();

    rtt_init_print!();
    rprintln!("Hello USART2\r\n");

    let mut cnt = 0;
    loop {
        let byte = block!(usart2.read()).unwrap();
        rprintln!("{}: {}\r", cnt, byte);
        cnt += 1;
    }
}
