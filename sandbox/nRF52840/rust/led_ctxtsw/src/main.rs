// UNDER DEVELOPMENT AND EXPERIMENT
// TODO: make it stable is enough

#![no_main]
#![no_std]

use cortex_m::asm;
use cortex_m::peripheral::NVIC;

use cortex_m_rt::entry;

use core::panic::PanicInfo;

use nrf52840_pac::{
    P0, TIMER0,
    interrupt, Interrupt};


pub mod minimult;
use minimult::{Minimult, MTMsgSender, MTMsgReceiver};


// TODO: remove static mut
//static mut LED_CNT: u32 = 64_000_000 / 64; // 1/64 sec


#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        asm::bkpt();
    }
}

#[entry]
fn main() -> ! {
    let mut mem = Minimult::memory::<[u8; 4096]>();
    let mut mt = Minimult::create(&mut mem);

    // ----- ----- ----- ----- -----

    let peri = nrf52840_pac::Peripherals::take().unwrap();

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

    // ----- ----- ----- ----- -----

    let (snd, rcv) = mt.msg_queue::<u32>(4);

    let v1 = 64_000_000 / 16 /*1/16sec*/;
    let v2 = 64_000_000 / 4 /*1/4sec*/;

    mt.register(0, 256, move || led_cnt(timer0, snd, v1, v2));
    mt.register(3, 256, move || led_tgl(p0, rcv));
    
    // ----- ----- ----- ----- -----

    mt.loops()
}

fn led_tgl(p0: P0, mut rcv: MTMsgReceiver<u32>)
{
    let mut cnt = 64_000_000 / 64; // 1/64 sec

    loop {
        rcv.receive(|v| {cnt = *v});

        p0.outset.write(|w| w.pin7().set_bit());

        //let cnt = unsafe { LED_CNT };
        asm::delay(cnt);

        p0.outclr.write(|w| w.pin7().set_bit());

        //let cnt = unsafe { LED_CNT };
        asm::delay(cnt);
    }
}

fn led_cnt(timer0: TIMER0, mut snd: MTMsgSender<u32>, cnt_t: u32, cnt_f: u32)
{
    let mut flag = false;

    loop {
        Minimult::idle();

        //

        timer0.events_compare[0].write(|w| {w.events_compare().bit(false)});
        NVIC::unpend(Interrupt::TIMER0);
        unsafe { NVIC::unmask(Interrupt::TIMER0) }

        //

        if flag {
            //unsafe { LED_CNT = cnt_t; }
            snd.send(cnt_t);
        }
        else {
            //unsafe { LED_CNT = cnt_f; }
            snd.send(cnt_f);
        }
        flag = !flag;
    }
}

#[interrupt]
fn TIMER0()
{
    NVIC::mask(Interrupt::TIMER0);
    
    Minimult::kick(0);
}
