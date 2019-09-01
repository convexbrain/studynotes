#![no_main]
#![no_std]

//#![allow(deprecated)] // to suppress "warning: use of deprecated item ... Users should use the traits in digital::v2.""

use core::panic::PanicInfo;
use cortex_m::asm;
use cortex_m_rt::entry;

use nrf52840_hal::target::Peripherals;
use nrf52840_hal::target::CorePeripherals;
use nrf52840_hal::gpio::*;
use nrf52840_hal::gpio::p0::*;
use nrf52840_hal::prelude::*;
use nrf52840_hal::target::interrupt;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        asm::bkpt();
    }
}

static mut O_LED: Option<P0_07<Output<PushPull>>> = None;
static mut O_SW: Option<P0_13<Input<Floating>>> = None;
static mut LED_ST: bool = false;

#[entry]
fn main() -> ! {
    let p = Peripherals::take().unwrap();
    let port0 = p.P0.split();

    unsafe {
        O_LED = Some(port0.p0_07.into_push_pull_output(Level::Low));
        O_SW = Some(port0.p0_13.into_floating_input());
    }

    let mut cp = CorePeripherals::take().unwrap();

    let mut t = p.TIMER0.constrain();
    t.enable_interrupt(&mut cp.NVIC);
    t.start(1000000u32);

    loop {}
}

#[interrupt]
fn TIMER0() -> ! {
    unsafe {
        let led = O_LED.as_mut().unwrap();

        if LED_ST {
            led.set_low();
            LED_ST = false;
        }
        else {
            led.set_high();
            LED_ST = true;
        }
    }
}
