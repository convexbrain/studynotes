#![no_main]
#![no_std]

use cortex_m::asm;
use cortex_m_rt::entry;

use core::panic::PanicInfo;

use nrf52840_pac::{
    Peripherals, P0, TIMER0,
    /*CorePeripherals,*/ NVIC,
    interrupt, Interrupt};

static mut O_P0: Option<P0> = None;
static mut O_TIMER0: Option<TIMER0> = None;

static mut LED_ST: bool = false;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        asm::bkpt();
    }
}

#[entry]
fn main() -> ! {
    let peri = Peripherals::take().unwrap();
    //let cperi = CorePeripherals::take().unwrap();

    {
        let p0 = peri.P0;
        p0.outclr.write(|w| w.pin7().set_bit());
        p0.pin_cnf[7].write(|w| w
            .dir().output()
            .input().disconnect()
            .pull().disabled()
            .drive().s0s1()
            .sense().disabled());
        //
        unsafe { O_P0 = Some(p0) }
    }

    {
        let timer0 = peri.TIMER0;
        timer0.shorts.write(|w| w
            .compare0_clear().enabled()
            .compare0_stop().disabled());
        timer0.prescaler.write(|w| unsafe { w.prescaler().bits(4) }); // 1 MHz
        timer0.bitmode.write(|w| w.bitmode()._32bit());
        timer0.intenset.modify(|_, w| w.compare0().set());

        unsafe { NVIC::unmask(Interrupt::TIMER0) }

        let cycles = 500000u32;
        timer0.cc[0].write(|w| unsafe { w.cc().bits(cycles) }); // 0.5 sec
        timer0.tasks_clear.write(|w| w.tasks_clear().set_bit());
        timer0.tasks_start.write(|w| w.tasks_start().set_bit());
        //
        unsafe { O_TIMER0 = Some(timer0) }
    }

    loop {}
}

#[interrupt]
fn TIMER0() {
    unsafe {
        let p0 = O_P0.as_mut().unwrap();
        let timer0 = O_TIMER0.as_mut().unwrap();

        if LED_ST {
            p0.outclr.write(|w| w.pin7().set_bit());
            LED_ST = false;
        }
        else {
            p0.outset.write(|w| w.pin7().set_bit());
            LED_ST = true;
        }

        timer0.events_compare[0].write(|w| {w.events_compare().bit(false)});
    }
}
