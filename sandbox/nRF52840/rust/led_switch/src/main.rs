#![no_main]
#![no_std]

//#![allow(deprecated)] // to suppress "warning: use of deprecated item ... Users should use the traits in digital::v2.""

use core::panic::PanicInfo;
use cortex_m::asm;
use cortex_m_rt::entry;

use nrf52840_hal::nrf52840_pac::Peripherals;
use nrf52840_hal::gpio::*;
use nrf52840_hal::prelude::*;

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
        if sw.is_high() {
            led.set_low();
        }
        else {
            led.set_high();
        }
    }
}
