// TODO: module

use cortex_m::peripheral::SCB;

use core::mem::{MaybeUninit, size_of, align_of, transmute};
use core::marker::PhantomData;

type MTTaskId = u16;
type MTTaskPri = u8;

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
    data: *const u8,
    vtbl: *const usize
}

fn inf_loop() -> !
{
    let tm = unsafe { O_TASKMGR.as_mut().unwrap() };
    tm.none();
    
    loop {}
}

//

#[derive(Clone, Copy, PartialEq, Debug)]
enum MTState
{
    None,
    Idle,
    Ready,
    Waiting
}

struct MTTask
{
    sp_start: *mut usize,
    sp_end: *mut usize,
    priority: MTTaskPri,
    //
    sp: *mut usize,
    kick_excnt: usize,
    wakeup_cnt: usize,
    signal_excnt: usize,
    wait_cnt: usize,
    state: MTState,
    //
    bh_array: Option<MTTaskId>
}

struct MTTaskMgr
{
    tasks: *mut MTTask,
    num_tasks: MTTaskId,
    bh_ready: MTTaskId,
    bh_idlewait: MTTaskId,
    //
    sp_loops: *mut usize,
    tid: Option<MTTaskId>
}

impl MTTaskMgr
{
    fn task_index(&mut self, tid: MTTaskId) -> &mut MTTask
    {
        assert!(tid < self.num_tasks); // TODO: better message
        unsafe { self.tasks.add(tid as usize).as_mut() }.unwrap()
    }

    fn task_current(&mut self) -> Option<&mut MTTask>
    {
        if let Some(curr_tid) = self.tid {
            Some(self.task_index(curr_tid))
        }
        else {
            None
        }
    }

    //

    fn bh_add_idlewait_tail(&mut self, tid: MTTaskId)
    {
        self.task_index(self.bh_ready + self.bh_idlewait).bh_array = Some(tid);

        self.bh_idlewait += 1;
    }

    fn bh_remove_idlewait_tail(&mut self)
    {
        self.bh_idlewait -= 1;

        self.task_index(self.bh_ready + self.bh_idlewait).bh_array = None;
    }

    fn bh_replace(&mut self, pos0: MTTaskId, pos1: MTTaskId)
    {
    }

    fn bh_upheap(&mut self)
    {
    }

    fn bh_downheap(&mut self)
    {
    }

    fn bh_none_to_ready(&mut self, tid: MTTaskId)
    {
        self.bh_add_idlewait_tail(tid);

        self.bh_idlewait_to_ready(self.bh_ready + self.bh_idlewait - 1);
    }

    fn bh_idlewait_to_ready(&mut self, pos: MTTaskId)
    {
        // replace pos <=> idlewait head
        self.bh_replace(pos, self.bh_ready);

        // idlewait head <=> ready tail
        self.bh_idlewait -= 1;
        self.bh_ready += 1;

        // upheap correction
        self.bh_upheap();
    }

    fn bh_ready_to_idlewait(&mut self)
    {
        // replace ready head <=> ready tail
        self.bh_replace(0, self.bh_ready - 1);

        // ready tail <=> idlewait head
        self.bh_idlewait += 1;
        self.bh_ready -= 1;

        // downheap correction
        self.bh_downheap();
    }

    fn bh_ready_round(&mut self)
    {
        self.bh_ready_to_idlewait();

        self.bh_idlewait_to_ready(self.bh_ready);
    }

    fn bh_ready_to_none(&mut self)
    {
        self.bh_ready_to_idlewait();

        // replace idlewait head <=> idlewait tail
        self.bh_replace(self.bh_ready, self.bh_ready + self.bh_idlewait - 1);
        
        // remove idlewait tail
        self.bh_remove_idlewait_tail();
    }

    // Main context

    fn new(tasks: *mut MTTask, num_tasks: MTTaskId) -> MTTaskMgr
    {
        for i in 0..num_tasks {
            unsafe {
                tasks.add(i as usize).write(
                    MTTask {
                        sp_start: core::ptr::null_mut(),
                        sp_end: core::ptr::null_mut(),
                        priority: 0,
                        sp: core::ptr::null_mut(),
                        kick_excnt: 0,
                        wakeup_cnt: 0,
                        signal_excnt: 0,
                        wait_cnt: 0,
                        state: MTState::None,
                        bh_array: None
                    }
                );
            }
        }

        MTTaskMgr {
            tasks,
            num_tasks,
            bh_ready: 0,
            bh_idlewait: 0,
            sp_loops: core::ptr::null_mut(),
            tid: None
        }
    }

    fn setup_task_once(sp: *mut usize, data: *mut u8, call_once: usize)
    {
        // TODO: magic number

        unsafe {
            // r0
            sp.add(8 + 0).write_volatile(data as usize);
            
            // lr
            sp.add(8 + 5).write_volatile(inf_loop as usize);

            // ReturnAddress
            sp.add(8 + 6).write_volatile(call_once);

            // xPSR: set T-bit since Cortex-M has only Thumb instructions
            sp.add(8 + 7).write_volatile(0x01000000);
        }
    }

    fn register_once<T>(&mut self, tid: MTTaskId, stack: &mut [usize], t: T)
    where T: FnOnce() + Send // TODO: appropriate lifetime
    {
        let task = self.task_index(tid);

        assert_eq!(task.state, MTState::None); // TODO: better message

        let sp_start = &mut stack[0] as *mut usize;
        let sp_end = &mut stack[stack.len() - 1] as *mut usize;

        let sz = size_of::<T>();
        let rfo = unsafe { transmute::<&dyn FnOnce(), RefFnOnce>(&t) };

        let sp = sp_end as usize;
        let sp = align_down::<T>(sp - sz);
        let data = sp as *mut u8;
        let sp = align_down::<usize>(sp);
        let sp = sp as *mut usize;
        let sp = unsafe { sp.sub(32) }; // TODO: magic number

        unsafe {
            core::intrinsics::copy(rfo.data, data, sz)
        }

        let vtbl = rfo.vtbl;
        let call_once = unsafe { vtbl.add(3).read() }; // TODO: magic number

        MTTaskMgr::setup_task_once(sp, data, call_once);

        task.sp_start = sp_start;
        task.sp_end = sp_end;
        task.priority = tid as u8; // TODO: task priority
        task.sp = sp;
        task.state = MTState::Ready;
    }

    // Task context

    fn idle(&mut self)
    {
        let task = self.task_current().unwrap();

        task.state = MTState::Idle;
        
        Minimult::schedule();
    }

    fn none(&mut self)
    {
        let task = self.task_current().unwrap();

        task.state = MTState::None;
        
        Minimult::schedule();
    }

    fn wait(&mut self)
    {
        let task = self.task_current().unwrap();

        task.state = MTState::Waiting;
        
        Minimult::schedule();
    }

    fn signal(&mut self, tid: MTTaskId)
    {
        let task = self.task_index(tid);

        unsafe {
            ex_countup(&mut task.signal_excnt);
        }

        if let Some(curr_tid) = self.tid {
            if curr_tid <= tid { // TODO: task priority
                return;
            }
        }

        Minimult::schedule();
    }

    // Interrupt context

    fn task_switch(&mut self, curr_sp: *mut usize) -> *mut usize
    {
        if let Some(task) = self.task_current() {
            assert!(curr_sp >= task.sp_start); // TODO: better message
            assert!(curr_sp <= task.sp_end); // TODO: better message

            task.sp = curr_sp;
        }
        else {
            self.sp_loops = curr_sp;
        }

        SCB::clear_pendsv();

        for i in 0.. self.num_tasks {
            let task = self.task_index(i);

            match task.state {
                MTState::Idle => {
                    if task.kick_excnt != task.wakeup_cnt {
                        task.wakeup_cnt = task.wakeup_cnt.wrapping_add(1);
                        task.state = MTState::Ready;
                    }
                },
                MTState::Waiting => {
                    if task.signal_excnt != task.wait_cnt {
                        task.wait_cnt = task.signal_excnt;
                        task.state = MTState::Ready;
                    }
                },
                _ => {}
            }
        }

        let mut next_tid = None;
        let mut next_sp = self.sp_loops;
        for i in 0.. self.num_tasks { // TODO: task priority
            let task = self.task_index(i);

            if let MTState::Ready = task.state {
                next_tid = Some(i);
                next_sp = task.sp;
                break;
            }
        }

        self.tid = next_tid;

        next_sp
    }

    // Task and Interrupt context

    fn kick(&mut self, tid: MTTaskId)
    {
        let task = self.task_index(tid);

        unsafe {
            ex_countup(&mut task.kick_excnt);
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
pub extern fn task_switch(curr_sp: *mut usize) -> *mut usize
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

    fn get<T, U>(&mut self, len: U) -> *mut T
    where U: Into<usize>
    {
        let len = len.into();
        let size = size_of::<T>() * len;

        let p = align_up::<T>(self.cur_pos);
        let e = p + size;

        assert!(e <= self.end_cap); // TODO: better message

        self.cur_pos = e;

        p as *mut T
    }

    fn get_mut<T>(&mut self) -> &'a mut T
    {
        let p = self.get::<T, _>(1_usize);

        unsafe { p.as_mut().unwrap() }
    }

    fn get_slice_mut<T, U>(&mut self, len: U) -> &'a mut [T]
    where U: Into<usize>
    {
        let len = len.into();

        let p = self.get::<T, _>(len);

        unsafe { core::slice::from_raw_parts_mut(p, len) }
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

    pub fn create<M>(mem: &'a mut MTMemory<M>, num_tasks: MTTaskId) -> Minimult<'a>
    {
        let mut alloc = MTAlloc::new(mem);

        unsafe {
            O_TASKMGR = Some(MTTaskMgr::new(alloc.get(num_tasks), num_tasks));
        }

        Minimult {
            alloc
        }
    }

    pub fn msg_queue<L>(&mut self, len: usize) -> (MTMsgSender<'a, L>, MTMsgReceiver<'a, L>)
    {
        let q = self.alloc.get_mut();
        let mem = self.alloc.get_slice_mut(len);

        *q = MTMsgQueue::new(mem);

        (
            MTMsgSender(q),
            MTMsgReceiver(q)
        )
    }

    pub fn register<T>(&mut self, tid: MTTaskId, stack_len: usize, t: T)
    where T: FnOnce() + Send + 'a
    {
        let tm = unsafe { O_TASKMGR.as_mut().unwrap() };

        let stack = self.alloc.get_slice_mut(stack_len);
        
        tm.register_once(tid, stack, t);
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

    pub fn kick(tid: MTTaskId)
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

    /*pub*/ fn signal(tid: MTTaskId)
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

    /*pub*/ fn curr_tid() -> Option<MTTaskId>
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

impl Drop for Minimult<'_>
{
    fn drop(&mut self)
    {
        panic!(); // better message
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

pub struct MTMsgQueue<'a, L>
{
    mem: &'a mut [Option<L>],
    wr_idx: usize,
    rd_idx: usize,
    wr_tid: Option<MTTaskId>,
    rd_tid: Option<MTTaskId>
}

impl<'a, L> MTMsgQueue<'a, L>
{
    fn new(mem: &'a mut [Option<L>]) -> MTMsgQueue<'a, L>
    {
        MTMsgQueue {
            mem,
            wr_idx: 0,
            rd_idx: 0,
            wr_tid: None,
            rd_tid: None
        }
    }
}

//

pub struct MTMsgSender<'a, L>(*mut MTMsgQueue<'a, L>);

unsafe impl<L> Send for MTMsgSender<'_, L> {}

impl<L> MTMsgSender<'_, L>
{
    pub fn vacant(&self) -> usize
    {
        let q = unsafe { self.0.as_ref().unwrap() };

        wrap_diff(q.rd_idx, wrap_inc(q.wr_idx, q.mem.len()), q.mem.len())
    }

    pub fn send(&mut self, v: L)
    {
        let q = unsafe { self.0.as_mut().unwrap() };

        q.wr_tid = Minimult::curr_tid();

        let curr_wr_idx = q.wr_idx;
        let next_wr_idx = wrap_inc(curr_wr_idx, q.mem.len());

        loop {
            if next_wr_idx == q.rd_idx {
                Minimult::wait();
            }
            else {
                break;
            }
        }

        q.mem[curr_wr_idx] = Some(v);

        q.wr_idx = next_wr_idx;

        if let Some(rd_tid) = q.rd_tid {
            Minimult::signal(rd_tid);
        }
    }
}

//

pub struct MTMsgReceiver<'a, L>(*mut MTMsgQueue<'a, L>);

unsafe impl<L> Send for MTMsgReceiver<'_, L> {}

impl<L> MTMsgReceiver<'_, L>
{
    pub fn available(&self) -> usize
    {
        let q = unsafe { self.0.as_ref().unwrap() };

        wrap_diff(q.wr_idx, q.rd_idx, q.mem.len())
    }

    pub fn receive<F>(&mut self, f: F)
    where F: FnOnce(&L)
    {
        let q = unsafe { self.0.as_mut().unwrap() };

        q.rd_tid = Minimult::curr_tid();

        let curr_rd_idx = q.rd_idx;
        let next_rd_idx = wrap_inc(curr_rd_idx, q.mem.len());

        loop {
            if curr_rd_idx == q.wr_idx {
                Minimult::wait();
            }
            else {
                break;
            }
        }

        f(q.mem[curr_rd_idx].as_ref().unwrap());
        q.mem[curr_rd_idx].take().unwrap();

        q.rd_idx = next_rd_idx;

        if let Some(wr_tid) = q.wr_tid {
            Minimult::signal(wr_tid);
        }
    }
} 
