use cortex_m::peripheral::SCB;

/* TODO: delete soon

extern crate alloc;
use alloc::boxed::Box;

struct BoxedFnOnce(usize, usize);

fn task_start_box(a0: usize, a1: usize) -> !
{
    let bfo = BoxedFnOnce(a0, a1);

    let bf = unsafe { core::mem::transmute::<BoxedFnOnce, Box<dyn FnOnce()>>(bfo) };

    bf();

    infloop()
}

//

struct RefFnOnce
{
    data: usize,
    vtbl: usize
}

fn infloop() -> !
{
    loop {}
}

*/

fn align_up(x: usize) -> usize
{
    let size = core::mem::size_of::<usize>();
    let y = (x + size - 1) / size;
    let y = y * size;
    y
}

fn align_down(x: usize) -> usize
{
    let size = core::mem::size_of::<usize>();
    let y = x / size;
    let y = y * size;
    y
}

extern "C" {
    fn ex_countup(exc: &mut usize);
}

//

struct RefFnMut
{
    data: usize,
    vtbl: usize
}

fn task_start_mut(data: usize, vtbl: usize) -> !
{
    let rfo = RefFnMut {
        data,
        vtbl
    };

    let rf = unsafe { core::mem::transmute::<RefFnMut, &mut dyn FnMut()>(rfo) };

    loop {
        rf();

        let tm = unsafe { O_TASKMGR.as_mut().unwrap() };
        tm.idle();
        
        Minimult::schedule();
    }
}

//

const NUM_TASKS: usize = 4;

#[derive(Clone, Copy)]
enum MTState
{
    None,
    Idle,
    Ready
}

struct MTTaskMgr
{
    // TODO: better data structure
    sp_loops: usize,
    sp: [usize; NUM_TASKS],
    trigger_exc: [usize; NUM_TASKS],
    start_cnt: [usize; NUM_TASKS],
    state: [MTState; NUM_TASKS],
    tid: Option<usize>
}

impl MTTaskMgr
{
    // Main context

    /* TODO: delete soon

    fn setup_task_box(sp: usize, bfo: BoxedFnOnce)
    {
        // TODO: magic number
        let r0 = sp + (8 + 0) * 4;
        let r0 = r0 as *mut usize;
        unsafe { *r0 = bfo.0 }

        let r1 = sp + (8 + 1) * 4;
        let r1 = r1 as *mut usize;
        unsafe { *r1 = bfo.1 }

        let ret_addr = sp + (8 + 6) * 4;
        let ret_addr = ret_addr as *mut usize;
        let fn_task_start = task_start_box as *const fn(usize, usize) -> !;
        unsafe { *ret_addr = fn_task_start as usize }

        let xpsr = sp + (8 + 7) * 4;
        let xpsr = xpsr as *mut usize;
        unsafe { *xpsr = 0x01000000 } // xPSR: set T-bit since Cortex-M has only Thumb instructions
    }

    fn register_box<T>(&mut self, tid: usize, sp: usize, t: T)
    where T: FnOnce() + Send + 'static
    {
        let bf: Box<dyn FnOnce()> = Box::new(t);
        let bfo = unsafe { core::mem::transmute::<Box<dyn FnOnce()>, BoxedFnOnce>(bf) };

        let sp = align_down(sp) - align_up(128); // TODO: magic number
        TaskMgr::setup_task_box(sp, bfo);

        if tid == 0 {
            self.sp0 = sp;
        } else if tid == 1 {
            self.sp1 = sp;
        } else if tid == 2 {
            self.sp2 = sp;
        } else if tid == 3 {
            self.sp3 = sp;
        } else {
            panic!();
        }
    }

    fn setup_task_once(sp: usize, data: usize, call_once: usize)
    {
        // TODO: magic number
        let r0 = sp + (8 + 0) * 4;
        let r0 = r0 as *mut usize;
        unsafe { *r0 = data as usize }

        let lr = sp + (8 + 5) * 4;
        let lr = lr as *mut usize;
        let fn_infloop = infloop as *const fn() -> !;
        unsafe { *lr = fn_infloop as usize }

        let ret_addr = sp + (8 + 6) * 4;
        let ret_addr = ret_addr as *mut usize;
        unsafe { *ret_addr = call_once }

        let xpsr = sp + (8 + 7) * 4;
        let xpsr = xpsr as *mut usize;
        unsafe { *xpsr = 0x01000000 } // xPSR: set T-bit since Cortex-M has only Thumb instructions
    }

    fn register_once<T>(&mut self, tid: usize, sp: usize, t: T)
    where T: FnOnce() + Send + 'static
    {
        let sz = core::mem::size_of::<T>();
        let rfo = unsafe { core::mem::transmute::<&dyn FnOnce(), RefFnOnce>(&t) };

        let sp = align_down(sp) - align_up(sz);
        let data = sp;
        unsafe {
            core::intrinsics::copy(rfo.data as *const u8, data as *mut u8, sz)
        }

        let call_once_addr = (rfo.vtbl + core::mem::size_of::<usize>() * 3) as *const usize; // TODO: magic number
        let call_once = unsafe {*call_once_addr};

        let sp = sp - align_up(128); // TODO: magic number
        TaskMgr::setup_task_once(sp, data, call_once);

        if tid == 0 {
            self.sp0 = sp;
        } else if tid == 1 {
            self.sp1 = sp;
        } else if tid == 2 {
            self.sp2 = sp;
        } else if tid == 3 {
            self.sp3 = sp;
        } else {
            panic!();
        }
    }

    */

    fn setup_task_mut(sp: usize, data: usize, vtbl: usize)
    {
        // TODO: magic number
        let r0 = sp + (8 + 0) * 4;
        let r0 = r0 as *mut usize;
        unsafe { *r0 = data as usize }

        let r1 = sp + (8 + 1) * 4;
        let r1 = r1 as *mut usize;
        unsafe { *r1 = vtbl as usize }

        let ret_addr = sp + (8 + 6) * 4;
        let ret_addr = ret_addr as *mut usize;
        let fn_task_start = task_start_mut as *const fn(usize, usize) -> !;
        unsafe { *ret_addr = fn_task_start as usize }

        let xpsr = sp + (8 + 7) * 4;
        let xpsr = xpsr as *mut usize;
        unsafe { *xpsr = 0x01000000 } // xPSR: set T-bit since Cortex-M has only Thumb instructions
    }

    fn register_mut<T>(&mut self, tid: usize, sp: usize, mut t: T)
    where T: FnMut() + Send + 'static
    {
        let sz = core::mem::size_of::<T>();
        let rfo = unsafe { core::mem::transmute::<&mut dyn FnMut(), RefFnMut>(&mut t) };

        let sp = align_down(sp) - align_up(sz);
        let data = sp;
        unsafe {
            core::intrinsics::copy(rfo.data as *const u8, data as *mut u8, sz)
        }

        let sp = sp - align_up(128); // TODO: magic number
        MTTaskMgr::setup_task_mut(sp, data, rfo.vtbl);

        self.sp[tid] = sp;
        self.state[tid] = MTState::Idle;
    }

    // Task context

    fn idle(&mut self)
    {
        self.state[self.tid.unwrap()] = MTState::Idle;
    }

    // Interrupt context

    fn task_switch(&mut self, curr_sp: usize) -> usize
    {
        SCB::clear_pendsv();

        for i in 0.. NUM_TASKS {
            if let MTState::Idle = self.state[i] {
                if self.trigger_exc[i] > self.start_cnt[i] {
                    self.start_cnt[i] = self.start_cnt[i].wrapping_add(1);
                    self.state[i] = MTState::Ready;
                }
            }
        }

        if let Some(curr_tid) = self.tid {
            self.sp[curr_tid] = curr_sp;
        }
        else {
            self.sp_loops = curr_sp;
        }

        let mut next_tid = None;
        let mut next_sp = self.sp_loops;
        for i in 0.. NUM_TASKS { // TODO: task priority
            if let MTState::Ready = self.state[i] {
                next_tid = Some(i);
                next_sp = self.sp[i];
                break;
            }
        }

        self.tid = next_tid;

        next_sp
    }

    // Task and Interrupt context

    fn trigger(&mut self, tid: usize)
    {
        unsafe {
            ex_countup(&mut self.trigger_exc[tid]);
        }

        if let Some(curr_tid) = self.tid {
            if curr_tid <= tid { // TODO: task priority
                return;
            }
        }

        Minimult::schedule();
    }
}

//

static mut O_TASKMGR: Option<MTTaskMgr> = None;

#[no_mangle]
pub extern fn task_switch(curr_sp: usize) -> usize
{
    let tm = unsafe { O_TASKMGR.as_mut().unwrap() };
    tm.task_switch(curr_sp)
}

//

pub struct MTStack<S>(S);

impl<S> MTStack<S>
{
    pub fn new() -> MTStack<S>
    {
        MTStack(unsafe { core::mem::MaybeUninit::<S>::uninit().assume_init() })
    }

    fn size(&self) -> usize
    {
        core::mem::size_of::<S>()
    }

    fn head(&self) -> usize
    {
        let ptr = &self.0;
        let ptr = ptr as *const S;
        let ptr = ptr as usize;
        ptr
    }
}


pub struct Minimult<'a>
{
    tm: MTTaskMgr,
    phantom: core::marker::PhantomData<&'a ()>
}

impl<'a> Minimult<'a>
{
    // Main context

    pub fn create() -> Self
    {
        let tm = MTTaskMgr {
            sp_loops: 0,
            sp: [0; NUM_TASKS],
            trigger_exc: [0; NUM_TASKS],
            start_cnt: [0; NUM_TASKS],
            state: [MTState::None; NUM_TASKS],
            tid: None
        };

        Minimult {
            tm,
            phantom: core::marker::PhantomData
        }
    }

    /* TODO: delete soon

    pub fn register_box<T, S>(mut self, tid: usize, stack: &'a mut MTStack<S>, t: T) -> Minimult<'a>
    where T: FnOnce() + Send + 'static
    {
        let sp = stack.head() + stack.size();
        self.tm.register_box(tid, sp, t);

        self
    }

    pub fn register_once<T, S>(mut self, tid: usize, stack: &'a mut MTStack<S>, t: T) -> Minimult<'a>
    where T: FnOnce() + Send + 'static
    {
        let sp = stack.head() + stack.size();
        self.tm.register_once(tid, sp, t);

        self
    }

    */

    fn register_mut<T, S>(mut self, tid: usize, stack: &'a mut MTStack<S>, t: T) -> Minimult<'a>
    where T: FnMut() + Send + 'static
    {
        let sp = stack.head() + stack.size();
        self.tm.register_mut(tid, sp, t);

        self
    }

    pub fn register<T, S>(self, tid: usize, stack: &'a mut MTStack<S>, t: T) -> Minimult<'a>
    where T: FnMut() + Send + 'static
    {
        self.register_mut(tid, stack, t)
    }

    pub fn register_ready<T, S>(mut self, tid: usize, stack: &'a mut MTStack<S>, t: T) -> Minimult<'a>
    where T: FnMut() + Send + 'static
    {
        self.tm.trigger(tid);
        self.register_mut(tid, stack, t)
    }

    pub fn loops(self) -> !
    {
        let control = cortex_m::register::control::read();
        assert!(control.spsel().is_msp()); // CONTROL.SPSEL: SP_main

        let scb_ptr = SCB::ptr();
        unsafe {
            (*scb_ptr).aircr.write(0x05fa0700); // PRIGROUP: 7 - no exception preempts each other
        }

        unsafe {
            O_TASKMGR = Some(self.tm);
        }
        
        Minimult::schedule();

        loop {
            cortex_m::asm::wfi(); // sleep to wait interrupt
        }
    }

    // Task and Interrupt context

    pub fn schedule()
    {
        unsafe {
            if O_TASKMGR.is_none() {
                return;
            }
        }

        SCB::set_pendsv();
    }

    pub fn trigger(tid: usize)
    {
        unsafe {
            if O_TASKMGR.is_none() {
                return;
            }

            O_TASKMGR.as_mut().unwrap().trigger(tid);
        }
    }
}
