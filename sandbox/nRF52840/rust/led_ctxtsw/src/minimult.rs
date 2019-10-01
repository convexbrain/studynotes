use cortex_m::peripheral::SCB;

use core::mem::MaybeUninit;
use core::marker::PhantomData;


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

struct RefFnOnce
{
    data: usize,
    vtbl: usize
}

fn inf_loop() -> !
{
    let tm = unsafe { O_TASKMGR.as_mut().unwrap() };
    tm.none();
    
    loop {}
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
    }
}

//

// TODO: macro
const NUM_TASKS: usize = 4;

#[derive(Clone, Copy)]
enum MTState
{
    None,
    Idle,
    Ready,
    Waiting
}

struct MTTaskMgr
{
    // TODO: better data structure
    sp_loops: usize,
    sp: [usize; NUM_TASKS],
    kick_excnt: [usize; NUM_TASKS],
    wakeup_cnt: [usize; NUM_TASKS],
    signal_excnt: [usize; NUM_TASKS],
    wait_cnt: [usize; NUM_TASKS],
    state: [MTState; NUM_TASKS],
    tid: Option<usize>
}

impl MTTaskMgr
{
    // Main context

    fn new() -> MTTaskMgr
    {
        MTTaskMgr {
            sp_loops: 0,
            sp: [0; NUM_TASKS],
            kick_excnt: [0; NUM_TASKS],
            wakeup_cnt: [0; NUM_TASKS],
            signal_excnt: [0; NUM_TASKS],
            wait_cnt: [0; NUM_TASKS],
            state: [MTState::None; NUM_TASKS],
            tid: None
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
        let fn_infloop = inf_loop as *const fn() -> !;
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
        MTTaskMgr::setup_task_once(sp, data, call_once);

        self.sp[tid] = sp;
        self.state[tid] = MTState::Idle;
    }

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
        
        Minimult::schedule();
    }

    fn none(&mut self)
    {
        self.state[self.tid.unwrap()] = MTState::None;
        
        Minimult::schedule();
    }

    fn wait(&mut self)
    {
        self.state[self.tid.unwrap()] = MTState::Waiting;
        
        Minimult::schedule();
    }

    fn signal(&mut self, tid: usize)
    {
        unsafe {
            ex_countup(&mut self.signal_excnt[tid]);
        }

        if let Some(curr_tid) = self.tid {
            if curr_tid <= tid { // TODO: task priority
                return;
            }
        }

        Minimult::schedule();
    }

    // Interrupt context

    fn task_switch(&mut self, curr_sp: usize) -> usize
    {
        SCB::clear_pendsv();

        for i in 0.. NUM_TASKS {
            match self.state[i] {
                MTState::Idle => {
                    if self.kick_excnt[i] != self.wakeup_cnt[i] {
                        self.wakeup_cnt[i] = self.wakeup_cnt[i].wrapping_add(1);
                        self.state[i] = MTState::Ready;
                    }
                },
                MTState::Waiting => {
                    if self.signal_excnt[i] != self.wait_cnt[i] {
                        self.wait_cnt[i] = self.signal_excnt[i];
                        self.state[i] = MTState::Ready;
                    }
                },
                _ => {}
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

    fn kick(&mut self, tid: usize)
    {
        unsafe {
            ex_countup(&mut self.kick_excnt[tid]);
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
static mut LOOP_STARTED: bool = false;

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
        MTStack(unsafe { MaybeUninit::<S>::uninit().assume_init() })
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
    phantom: PhantomData<&'a ()>
}

impl<'a> Minimult<'a>
{
    // Main context

    pub fn create() -> Self
    {
        unsafe {
            O_TASKMGR = Some(MTTaskMgr::new());
        }

        Minimult {
            phantom: PhantomData
        }
    }

    pub fn register_once<T, S>(self, tid: usize, stack: &'a mut MTStack<S>, t: T) -> Minimult<'a>
    where T: FnOnce() + Send + 'static
    {
        let tm = unsafe { O_TASKMGR.as_mut().unwrap() };

        let sp = stack.head() + stack.size();
        tm.register_once(tid, sp, t);

        self
    }

    pub fn register_mut<T, S>(self, tid: usize, stack: &'a mut MTStack<S>, t: T) -> Minimult<'a>
    where T: FnMut() + Send + 'static
    {
        let tm = unsafe { O_TASKMGR.as_mut().unwrap() };

        let sp = stack.head() + stack.size();
        tm.register_mut(tid, sp, t);

        self
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
            LOOP_STARTED = true;
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
            if !LOOP_STARTED {
                return;
            }
        }

        SCB::set_pendsv();
    }

    pub fn kick(tid: usize)
    {
        unsafe {
            if let Some(tm) = O_TASKMGR.as_mut() {
                tm.kick(tid);
            }
        }
    }

    pub fn signal(tid: usize)
    {
        unsafe {
            if let Some(tm) = O_TASKMGR.as_mut() {
                tm.signal(tid);
            }
        }
    }

    pub fn wait()
    {
        unsafe {
            if let Some(tm) = O_TASKMGR.as_mut() {
                tm.wait();
            }
        }
    }

    pub fn curr_tid() -> Option<usize>
    {
        unsafe {
            if let Some(tm) = O_TASKMGR.as_ref() {
                tm.tid
            }
            else {
                None
            }
        }
    }
}

//

// TODO: macro
const DEPTH: usize = 8;

fn wrap_inc(x: usize, bound: usize) -> usize
{
    let y = x + 1;
    if y < bound {y} else {0}
}

pub struct MTMsgSender<M>
{
    wr_idx: usize,
    rd_idx: usize,
    wr_tid: Option<usize>,
    rd_tid: Option<usize>,
    a: [MaybeUninit<Option<M>>; DEPTH]
}

impl<M> MTMsgSender<M>
{
    pub fn new() -> MTMsgSender<M>
    {
        MTMsgSender {
            wr_idx: 0,
            rd_idx: 0,
            wr_tid: None,
            rd_tid: None,
            a: unsafe { MaybeUninit::uninit().assume_init() },
        }
    }

    pub fn receiver(&mut self) -> MTMsgReceiver<M>
    {
        MTMsgReceiver {
            sender: self
        }
    }

    pub fn send(&mut self, v: M)
    {
        self.wr_tid = Minimult::curr_tid();

        let curr_wr_idx = self.wr_idx;
        let next_wr_idx = wrap_inc(curr_wr_idx, self.a.len());

        loop {
            if next_wr_idx == self.rd_idx {
                Minimult::wait();
            }
            else {
                break;
            }
        }

        self.a[curr_wr_idx] = MaybeUninit::new(Some(v));

        self.wr_idx = next_wr_idx;

        if let Some(rd_tid) = self.rd_tid {
            Minimult::signal(rd_tid);
        }
    }
}

pub struct MTMsgReceiver<M>
{
    sender: *mut MTMsgSender<M>
}

unsafe impl<M> Send for MTMsgReceiver<M> {}

impl<M> MTMsgReceiver<M>
{
    pub fn receive<F>(&mut self, f: F)
    where F: FnOnce(&M)
    {
        let snd = unsafe { self.sender.as_mut().unwrap() };

        snd.rd_tid = Minimult::curr_tid();

        let curr_rd_idx = snd.rd_idx;
        let next_rd_idx = wrap_inc(curr_rd_idx, snd.a.len());

        loop {
            if curr_rd_idx == snd.wr_idx {
                Minimult::wait();
            }
            else {
                break;
            }
        }

        let rp = unsafe { core::mem::transmute::<_, &mut Option<M>>(snd.a[curr_rd_idx].as_mut_ptr()) };
        f(rp.as_ref().unwrap());
        rp.take().unwrap();

        snd.rd_idx = next_rd_idx;

        if let Some(wr_tid) = snd.wr_tid {
            Minimult::signal(wr_tid);
        }
    }
} 
