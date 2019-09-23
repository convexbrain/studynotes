// UNDER DEVELOPMENT AND EXPERIMENT

#![no_main]
#![no_std]
//#![feature(asm)]
#![feature(alloc_error_handler)]

use cortex_m::asm;
use cortex_m::peripheral::NVIC;

use cortex_m_rt::entry;

use core::panic::PanicInfo;

use nrf52840_pac::{
    P0, TIMER0,
    interrupt, Interrupt};


#[macro_use] // TODO: use instead macro_use
pub mod minimult;
use minimult::{Minimult, MTStack};


use alloc_cortex_m::CortexMHeap;

#[global_allocator]
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();

#[alloc_error_handler]
pub fn alloc_error_handler(_layout: core::alloc::Layout) -> !
{
    panic!();
}


static mut O_P0: Option<P0> = None;
static mut O_TIMER0: Option<TIMER0> = None;

static mut LED_CNT: u32 = 64_000_000 / 4; // 1/4 sec


#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        asm::bkpt();
    }
}

#[entry]
fn main() -> ! {
    {
        let start = cortex_m_rt::heap_start() as usize;
        let size = 1024; // in bytes
        unsafe { ALLOCATOR.init(start, size) }
    }

    //

    let mut cmperi = cortex_m::Peripherals::take().unwrap();

    let mut stack0 = minimult_stack!(1024);
    let mut stack1 = minimult_stack!(1024);
    let mut stack2 = minimult_stack!(1024);

    let v1 = 64_000_000 / 16 /*1/16sec*/;
    let v2 = 64_000_000 / 4 /*1/4sec*/;

    let mt = Minimult::create(&mut cmperi)
        .register(0, &mut stack0, move || led_tgl())
        .register(1, &mut stack1, move || led_cnt(v1))
        .register(2, &mut stack2, move || led_cnt(v2));

    //

    let peri = nrf52840_pac::Peripherals::take().unwrap();

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

        let cycles = 1_000_000 * 2;
        timer0.cc[0].write(|w| unsafe { w.cc().bits(cycles) }); // 2 sec
        timer0.tasks_clear.write(|w| w.tasks_clear().set_bit());
        timer0.tasks_start.write(|w| w.tasks_start().set_bit());
        //
        unsafe { O_TIMER0 = Some(timer0) }
    }

    //

    mt.start()
}

fn led_tgl()
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

fn led_cnt(cnt: u32)
{
    loop {
        unsafe {
            LED_CNT = cnt;
        }
        
        //MinMT::req_task_switch();
    }
}

#[interrupt]
fn TIMER0()
{
    unsafe {
        let timer0 = O_TIMER0.as_mut().unwrap();

        timer0.events_compare[0].write(|w| {w.events_compare().bit(false)});
    }

    Minimult::req_task_switch();
}
