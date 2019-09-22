// UNDER DEVELOPMENT AND EXPERIMENT

#![no_main]
#![no_std]
//#![feature(asm)]
#![feature(alloc_error_handler)]

use cortex_m::asm;
use cortex_m::peripheral::{NVIC, SCB};

use cortex_m_rt::entry;

use core::panic::PanicInfo;

use nrf52840_pac::{
    P0, TIMER0,
    interrupt, Interrupt};

extern crate alloc;
use alloc::boxed::Box;
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

static mut STACK0: [usize; 1024] = [0; 1024];
static mut STACK1: [usize; 1024] = [0; 1024];
static mut STACK2: [usize; 1024] = [0; 1024];
static mut STACK3: [usize; 1024] = [0; 1024];

struct BoxedFnOnce(usize, usize);

fn task_start(a0: usize, a1: usize) -> !
{
    let bfo = BoxedFnOnce(a0, a1);

    let bf = unsafe { core::mem::transmute::<BoxedFnOnce, Box<dyn FnOnce()>>(bfo) };

    bf();

    loop {}
}

struct RefFnOnce
{
    data: *const u8,
    vtbl: *const usize
}

fn infloop() -> !
{
    loop {}
}

struct TaskMgr
{
    sp0: *mut usize,
    sp1: *mut usize,
    sp2: *mut usize,
    sp3: *mut usize,
    tid: Option<usize>,
    num_tasks: usize,
}

impl TaskMgr
{
    fn setup_task(sp: *mut usize, bfo: BoxedFnOnce)
    {
        let fn_task_strart = task_start as *const fn(usize, usize) -> !;

        let r0 = sp as usize + (8 + 0) * 4;
        let r0 = r0 as *mut usize;
        unsafe { *r0 = bfo.0 }

        let r1 = sp as usize + (8 + 1) * 4;
        let r1 = r1 as *mut usize;
        unsafe { *r1 = bfo.1 }

        let ret_addr = sp as usize + (8 + 6) * 4;
        let ret_addr = ret_addr as *mut usize;
        unsafe { *ret_addr = fn_task_strart as usize }

        let xpsr = sp as usize + (8 + 7) * 4;
        let xpsr = xpsr as *mut usize;
        unsafe { *xpsr = 0x01000000 } // xPSR: set T-bit since Cortex-M has only Thumb instructions
    }

    fn register<T>(self, tid: usize, t: T) -> Self
    where T: FnOnce() + Send + 'static
    {
        let bf: Box<dyn FnOnce()> = Box::new(t);
        let bfo = unsafe { core::mem::transmute::<Box<dyn FnOnce()>, BoxedFnOnce>(bf) };

        let sp = if tid == 0 {
            self.sp0
        } else if tid == 1 {
            self.sp1
        } else if tid == 2 {
            self.sp2
        } else if tid == 3 {
            self.sp3
        } else {
            panic!();
        };

        TaskMgr::setup_task(sp, bfo);

        self
    }

    fn setup_task_rfo(sp: usize, data: usize, call_once: usize)
    {
        let fn_infloop = infloop as *const fn() -> !;

        let r0 = sp as usize + (8 + 0) * 4;
        let r0 = r0 as *mut usize;
        unsafe { *r0 = data as usize }

        let lr = sp as usize + (8 + 5) * 4;
        let lr = lr as *mut usize;
        unsafe { *lr = fn_infloop as usize }

        let ret_addr = sp as usize + (8 + 6) * 4;
        let ret_addr = ret_addr as *mut usize;
        unsafe { *ret_addr = call_once }

        let xpsr = sp as usize + (8 + 7) * 4;
        let xpsr = xpsr as *mut usize;
        unsafe { *xpsr = 0x01000000 } // xPSR: set T-bit since Cortex-M has only Thumb instructions
    }

    fn register_rfo<T>(mut self, tid: usize, t: T) -> Self
    where T: FnOnce() + Send + 'static
    {
        let sz = core::mem::size_of::<T>();
        let rfo = unsafe { core::mem::transmute::<&dyn FnOnce(), RefFnOnce>(&t) };

        let sp = if tid == 0 {
            self.sp0
        } else if tid == 1 {
            self.sp1
        } else if tid == 2 {
            self.sp2
        } else if tid == 3 {
            self.sp3
        } else {
            panic!();
        } as usize;

        let sp = sp - sz;
        let data = sp;
        unsafe {
            core::intrinsics::copy(rfo.data, data as *mut u8, sz)
        }

        let call_once_addr = ((rfo.vtbl as usize) + core::mem::size_of::<usize>() * 3) as *const usize;
        let call_once = unsafe {*call_once_addr};

        let sp = sp - 128;
        TaskMgr::setup_task_rfo(sp, data, call_once);

        if tid == 0 {
            self.sp0 = sp as *mut usize;
        } else if tid == 1 {
            self.sp1 = sp as *mut usize;
        } else if tid == 2 {
            self.sp2 = sp as *mut usize;
        } else if tid == 3 {
            self.sp3 = sp as *mut usize;
        } else {
            panic!();
        };

        self
    }
}

static mut O_TASKMGR: Option<TaskMgr> = None;

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

    {
        let v = 64_000_000 / 32;
        unsafe {
            O_TASKMGR = Some(
                TaskMgr {
                    sp0: &mut STACK0[1024 - 128],
                    sp1: &mut STACK1[1024 - 128],
                    sp2: &mut STACK2[1024 - 128],
                    sp3: &mut STACK3[1024 - 128],
                    tid: None,
                    num_tasks: 4,
                }
                .register(0, move || led_tgl())
                //.register(1, move || led_cnt(64_000_000 / 16 /*1/16sec*/))
                .register_rfo(1, move || led_cnt(v))
                .register(2, move || led_cnt(64_000_000 / 4 /*1/4sec*/))
            );
        }
    }

    let peri = nrf52840_pac::Peripherals::take().unwrap();

    {
        let control = cortex_m::register::control::read();
        assert!(control.spsel().is_msp()); // CONTROL.SPSEL: SP_main

        let mut cmperi = cortex_m::Peripherals::take().unwrap();
        unsafe { cmperi.SCB.set_priority(cortex_m::peripheral::scb::SystemHandler::PendSV, 255) } // PendSV: lowest priority
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

        let cycles = 1_000_000 * 2;
        timer0.cc[0].write(|w| unsafe { w.cc().bits(cycles) }); // 2 sec
        timer0.tasks_clear.write(|w| w.tasks_clear().set_bit());
        timer0.tasks_start.write(|w| w.tasks_start().set_bit());
        //
        unsafe { O_TIMER0 = Some(timer0) }
    }

    req_task_switch();

    loop {}
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
        
        //req_task_switch();
    }
}

fn req_task_switch()
{
    SCB::set_pendsv();
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
pub extern fn task_switch(curr_sp: *mut usize) -> *mut usize
{
    SCB::clear_pendsv();

    let next_sp;

    unsafe {
        let mut t = O_TASKMGR.as_mut().unwrap();

        let next_tid;

        if let Some(curr_tid) = t.tid {
            if curr_tid == 0 {
                t.sp0 = curr_sp;
            }
            else if curr_tid == 1 {
                t.sp1 = curr_sp;
            }
            else if curr_tid == 2 {
                t.sp0 = curr_sp;
            }
            else if curr_tid == 3 {
                t.sp2 = curr_sp;
            }
            else {
                panic!();
            }

            next_tid = if curr_tid + 1 < t.num_tasks {curr_tid + 1} else {0};
        }
        else {
            next_tid = 0;
        };

        t.tid = Some(next_tid);
        if next_tid == 0 {
            next_sp = t.sp0;
        }
        else if next_tid == 1 {
            next_sp = t.sp1;
        }
        else if next_tid == 2 {
            next_sp = t.sp0;
        }
        else if next_tid == 3 {
            next_sp = t.sp2;
        }
        else {
            panic!();
        }
    }

    next_sp
}
