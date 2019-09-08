#![no_main]
#![no_std]
#![feature(asm)]

use cortex_m::asm;
use cortex_m_rt::entry;

use core::panic::PanicInfo;

use nrf52840_pac::{
    Peripherals, P0, TIMER0,
    CorePeripherals, NVIC,
    interrupt, Interrupt};

static mut O_P0: Option<P0> = None;
static mut O_TIMER0: Option<TIMER0> = None;

static mut LED_CNT: u32 = 64_000_000 / 2;

struct Task
{
    f0: *const fn() -> !,
    f1: *const fn() -> !,
    f2: *const fn() -> !,
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        asm::bkpt();
    }
}

#[entry]
fn main() -> ! {
    let peri = Peripherals::take().unwrap();
    let mut cperi = CorePeripherals::take().unwrap();

    {
        assert!(cortex_m::register::control::read().spsel().is_msp()); // CONTROL.SPSEL: SP_main
        unsafe { cperi.SCB.set_priority(cortex_m::peripheral::scb::SystemHandler::SVCall, 255) } // SVCall: lowest priority
    }

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

        let cycles = 1_000_000;
        timer0.cc[0].write(|w| unsafe { w.cc().bits(cycles) }); // 1 sec
        timer0.tasks_clear.write(|w| w.tasks_clear().set_bit());
        timer0.tasks_start.write(|w| w.tasks_start().set_bit());
        //
        unsafe { O_TIMER0 = Some(timer0) }
    }

    {
        // TODO
        let t = Task {
            f0: led_tgl as *const fn() -> !,
            f1: led_slow as *const fn() -> !,
            f2: led_fast as *const fn() -> !,
        };
    }

    loop {}
}

fn led_tgl() -> !
{
    loop {
        unsafe {
            let p0 = O_P0.as_mut().unwrap();

            p0.outclr.write(|w| w.pin7().set_bit());
            asm::delay(LED_CNT);

            p0.outset.write(|w| w.pin7().set_bit());
            asm::delay(LED_CNT);
        }
    }
}

fn led_slow() -> !
{
    loop {
        unsafe {
            LED_CNT = 64_000_000 / 4;
        }

        req_task_switch();
    }
}

fn led_fast() -> !
{
    loop {
        unsafe {
            LED_CNT = 64_000_000 / 8;
        }

        req_task_switch();
    }
}

fn req_task_switch()
{
    unsafe { asm!("svc 0" : : : : "volatile") }
}

#[interrupt]
fn TIMER0()
{
    unsafe {
        let timer0 = O_TIMER0.as_mut().unwrap();

        timer0.events_compare[0].write(|w| {w.events_compare().bit(false)});
    }

    req_task_switch();
}

#[no_mangle]
pub extern fn task_switch(curr_sp: u32) -> u32
{
    // TODO
    curr_sp
}
