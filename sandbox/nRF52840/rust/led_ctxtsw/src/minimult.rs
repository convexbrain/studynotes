// TODO: module

use cortex_m::peripheral::SCB;

use core::mem::{MaybeUninit, size_of, align_of, transmute};
use core::marker::PhantomData;

//

fn align_up<T>(x: usize) -> usize
{
    let align = align_of::<T>();
    let y = (x + align - 1) / align;
    let y = y * align;
    y
}

fn align_down<T>(x: usize) -> usize
{
    let align = align_of::<T>();
    let y = x / align;
    let y = y * align;
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

const NUM_TASKS: usize = 4; // TODO: macro

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
        let sz = size_of::<T>();
        let rfo = unsafe { transmute::<&dyn FnOnce(), RefFnOnce>(&t) };

        let sp = align_down::<T>(sp - sz);
        let data = sp;
        unsafe {
            core::intrinsics::copy(rfo.data as *const u8, data as *mut u8, sz)
        }

        let call_once_addr = (rfo.vtbl + size_of::<usize>() * 3) as *const usize; // TODO: magic number
        let call_once = unsafe {*call_once_addr};

        let sp = align_down::<usize>(sp) - size_of::<usize>() * 32; // TODO: magic number
        MTTaskMgr::setup_task_once(sp, data, call_once);

        self.sp[tid] = sp;
        self.state[tid] = MTState::Ready;
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
        // TODO: sp underflow check
        if let Some(curr_tid) = self.tid {
            self.sp[curr_tid] = curr_sp;
        }
        else {
            self.sp_loops = curr_sp;
        }

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

pub struct MTMemory<M>(MaybeUninit<M>);

impl<M> MTMemory<M>
{
    const fn new() -> MTMemory<M>
    {
        MTMemory(MaybeUninit::<M>::uninit())
    }

    fn size(&self) -> usize
    {
        size_of::<M>()
    }

    fn head(&mut self) -> usize
    {
        self.0.as_mut_ptr() as usize
    }
}

//

struct MTAlloc<'a>
{
    cur_pos: usize,
    end_cap: usize,
    phantom: PhantomData<&'a ()>
}

impl<'a> MTAlloc<'a>
{
    fn new<M>(mem: &'a mut MTMemory<M>) -> MTAlloc<'a>
    {
        MTAlloc {
            cur_pos: mem.head(),
            end_cap: mem.head() + mem.size(),
            phantom: PhantomData
        }
    }

    fn get<T>(&mut self, len: usize) -> *mut T
    {
        let size = size_of::<T>() * len;

        let p = align_up::<T>(self.cur_pos);
        let e = p + size;

        assert!(e <= self.end_cap); // TODO: better message

        self.cur_pos = e;

        p as *mut T
    }
}

//

pub struct Minimult<'a>
{
    alloc: MTAlloc<'a>
}

impl<'a> Minimult<'a>
{
    // Main context

    pub const fn memory<M>() -> MTMemory<M>
    {
        MTMemory::new()
    }

    pub fn create<M>(mem: &'a mut MTMemory<M>) -> Minimult<'a>
    {
        unsafe {
            O_TASKMGR = Some(MTTaskMgr::new());
        }

        Minimult {
            alloc: MTAlloc::new(mem)
        }
    }

    pub fn msg_queue<L>(&mut self, len: usize) -> (MTMsgSender<L>, MTMsgReceiver<L>)
    {
        let q = self.alloc.get::<MTMsgQueue<L>>(1);
        let mem = self.alloc.get::<Option<L>>(len);

        unsafe {
            *q = MTMsgQueue::new(mem, len);
        }

        (MTMsgSender(q), MTMsgReceiver(q))
    }

    pub fn register<T>(&mut self, tid: usize, stack_len: usize, t: T)
    where T: FnOnce() + Send + 'static
    {
        let tm = unsafe { O_TASKMGR.as_mut().unwrap() };

        let sp = self.alloc.get::<usize>(stack_len) as usize;
        let sp = sp + size_of::<usize>() * stack_len;
        tm.register_once(tid, sp, t);
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

    pub fn idle()
    {
        unsafe {
            if let Some(tm) = O_TASKMGR.as_mut() {
                tm.idle();
            }
        }
    }

    /*pub*/ fn signal(tid: usize)
    {
        unsafe {
            if let Some(tm) = O_TASKMGR.as_mut() {
                tm.signal(tid);
            }
        }
    }

    /*pub*/ fn wait()
    {
        unsafe {
            if let Some(tm) = O_TASKMGR.as_mut() {
                tm.wait();
            }
        }
    }

    /*pub*/ fn curr_tid() -> Option<usize>
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

fn wrap_inc(x: usize, bound: usize) -> usize
{
    let y = x + 1;
    if y < bound {y} else {0}
}

fn wrap_diff(x: usize, y: usize, bound: usize) -> usize
{
    if x >= y {
        x - y
    }
    else {
        x + (bound - y)
    }
}

//

pub struct MTMsgQueue<L>
{
    mem_head: *mut Option<L>,
    mem_len: usize,
    wr_idx: usize,
    rd_idx: usize,
    wr_tid: Option<usize>,
    rd_tid: Option<usize>
}

impl<L> MTMsgQueue<L>
{
    fn new(mem_head: *mut Option<L>, mem_len: usize) -> MTMsgQueue<L>
    {
        MTMsgQueue {
            mem_head,
            mem_len,
            wr_idx: 0,
            rd_idx: 0,
            wr_tid: None,
            rd_tid: None
        }
    }

    fn index(&mut self, idx: usize) -> *mut Option<L>
    {
        let ptr = self.mem_head as usize;
        let ptr = ptr + size_of::<Option<L>>() * idx;
        let ptr = ptr as *mut Option<L>;
        ptr
    }
}

//

pub struct MTMsgSender<L>(*mut MTMsgQueue<L>);

unsafe impl<L> Send for MTMsgSender<L> {}

impl<L> MTMsgSender<L>
{
    pub fn vacant(&self) -> usize
    {
        let q = unsafe { self.0.as_mut().unwrap() };

        wrap_diff(q.rd_idx, wrap_inc(q.wr_idx, q.mem_len), q.mem_len)
    }

    pub fn send(&mut self, v: L)
    {
        let q = unsafe { self.0.as_mut().unwrap() };

        q.wr_tid = Minimult::curr_tid();

        let curr_wr_idx = q.wr_idx;
        let next_wr_idx = wrap_inc(curr_wr_idx, q.mem_len);

        loop {
            if next_wr_idx == q.rd_idx {
                Minimult::wait();
            }
            else {
                break;
            }
        }

        let ptr = q.index(curr_wr_idx);

        unsafe {
            *ptr = Some(v);
        }

        q.wr_idx = next_wr_idx;

        if let Some(rd_tid) = q.rd_tid {
            Minimult::signal(rd_tid);
        }
    }
}

//

pub struct MTMsgReceiver<L>(*mut MTMsgQueue<L>);

unsafe impl<L> Send for MTMsgReceiver<L> {}

impl<L> MTMsgReceiver<L>
{
    pub fn available(&self) -> usize
    {
        let q = unsafe { self.0.as_mut().unwrap() };

        wrap_diff(q.wr_idx, q.rd_idx, q.mem_len)
    }

    pub fn receive<F>(&mut self, f: F)
    where F: FnOnce(&L)
    {
        let q = unsafe { self.0.as_mut().unwrap() };

        q.rd_tid = Minimult::curr_tid();

        let curr_rd_idx = q.rd_idx;
        let next_rd_idx = wrap_inc(curr_rd_idx, q.mem_len);

        loop {
            if curr_rd_idx == q.wr_idx {
                Minimult::wait();
            }
            else {
                break;
            }
        }

        let ptr = q.index(curr_rd_idx);
        let ptr = unsafe { ptr.as_mut().unwrap() };

        f(ptr.as_ref().unwrap());
        ptr.take().unwrap();

        q.rd_idx = next_rd_idx;

        if let Some(wr_tid) = q.wr_tid {
            Minimult::signal(wr_tid);
        }
    }
} 
