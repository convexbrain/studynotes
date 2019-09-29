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


pub mod minimult;
use minimult::{Minimult, MTStack};


// TODO: remove if not necessary
/*
use alloc_cortex_m::CortexMHeap;

#[global_allocator]
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();

#[alloc_error_handler]
pub fn alloc_error_handler(_layout: core::alloc::Layout) -> !
{
    panic!();
}
*/


// TODO: remove static mut
static mut LED_CNT: u32 = 64_000_000 / 64; // 1/64 sec


#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        asm::bkpt();
    }
}

#[entry]
fn main() -> ! {
    // TODO: remove if not necessary
    /*
    {
        let start = cortex_m_rt::heap_start() as usize;
        let size = 1024; // in bytes
        unsafe { ALLOCATOR.init(start, size) }
    }
    */

    // ----- ----- ----- ----- -----

    let mt = Minimult::create();

    // ----- ----- ----- ----- -----

    let peri = nrf52840_pac::Peripherals::take().unwrap();
    //let mut cmperi = cortex_m::Peripherals::take().unwrap();

    // ----- ----- ----- ----- -----

    let p0 = peri.P0;
    p0.outclr.write(|w| w.pin7().set_bit());
    p0.pin_cnf[7].write(|w| w
        .dir().output()
        .input().disconnect()
        .pull().disabled()
        .drive().s0s1()
        .sense().disabled());

    // ----- ----- ----- ----- -----

    let mut timer0 = peri.TIMER0;
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

    // ----- ----- ----- ----- -----

    let mut stack0 = MTStack::<[usize; 1024]>::new();
    let mut stack3 = MTStack::<[usize; 1024]>::new();

    let v1 = 64_000_000 / 16 /*1/16sec*/;
    let v2 = 64_000_000 / 4 /*1/4sec*/;
    let mut flag = false;

    let mt = mt
        .register_mut(0, &mut stack0, move || led_cnt(&mut timer0, &mut flag, v1, v2))
        .register_once(3, &mut stack3, move || led_tgl(p0));

    Minimult::kick(3);
    
    // ----- ----- ----- ----- -----

    mt.loops()
}

fn led_tgl(p0: P0)
{
    loop {
        p0.outset.write(|w| w.pin7().set_bit());

        let led_cnt = unsafe { LED_CNT };
        asm::delay(led_cnt);

        p0.outclr.write(|w| w.pin7().set_bit());

        let led_cnt = unsafe { LED_CNT };
        asm::delay(led_cnt);
    }
}

fn led_cnt(timer0: &mut TIMER0, flag: &mut bool, cnt_t: u32, cnt_f: u32)
{
    timer0.events_compare[0].write(|w| {w.events_compare().bit(false)});
    NVIC::unpend(Interrupt::TIMER0);
    unsafe { NVIC::unmask(Interrupt::TIMER0) }

    //

    if *flag {
        *flag = false;
        unsafe { LED_CNT = cnt_t; }
    }
    else {
        *flag = true;
        unsafe { LED_CNT = cnt_f; }
    }
}

#[interrupt]
fn TIMER0()
{
    NVIC::mask(Interrupt::TIMER0);
    
    Minimult::kick(0);
}
