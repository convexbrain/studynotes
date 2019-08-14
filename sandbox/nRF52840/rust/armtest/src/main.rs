#![no_main]
#![no_std]

use core::panic::PanicInfo;
use cortex_m::asm;
use cortex_m_rt::entry;

use nrf52840_hal::nrf52840_pac::Peripherals;
use nrf52840_hal::gpio::*;
//use nrf52840_hal::prelude::*;
use embedded_hal::digital::v2::OutputPin;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        asm::bkpt();
    }
}

#[entry]
fn main() -> ! {
    let p = Peripherals::take().unwrap();
    let port0 = p.P0.split();

    let mut led1 = port0.p0_07.into_push_pull_output(Level::High);
    led1.set_high().unwrap();

    loop {
        asm::bkpt();
    }
}
