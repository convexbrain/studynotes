#![no_main]
#![no_std]

use core::panic::PanicInfo;
use cortex_m::asm;
use cortex_m_rt::entry;

use nrf52840_hal::nrf52840_pac::Peripherals;
use nrf52840_hal::gpio::*;
//use nrf52840_hal::prelude::*;
use embedded_hal::digital::v2::OutputPin;
use embedded_hal::digital::v2::InputPin;

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

    let mut led = port0.p0_07.into_push_pull_output(Level::Low);
    let sw = port0.p0_13.into_floating_input();

    loop {
        if sw.is_high().unwrap() {
            led.set_low().unwrap();
        }
        else {
            led.set_high().unwrap();
        }
    }
}
