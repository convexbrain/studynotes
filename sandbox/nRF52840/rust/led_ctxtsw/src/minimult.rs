// TODO: module
// TODO: clarify TaskMgr/Minimult role separation

use cortex_m::peripheral::SCB;

use core::mem::{MaybeUninit, size_of, align_of, transmute};
use core::marker::PhantomData;
use core::convert::TryInto;

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

struct MTBHeapList<I, K>
{
    array: MTRawArray<Option<(I, K)>>,
    n_bheap: I,
    n_list: I
}

impl<I, K> MTBHeapList<I, K>
where I: Into<usize> + Copy, usize: TryInto<I>, K: Ord
{
    pub fn new(array: MTRawArray<Option<(I, K)>>) -> MTBHeapList<I, K>
    {
        MTBHeapList {
            array,
            n_bheap: 0.try_into().ok().unwrap(),
            n_list: 0.try_into().ok().unwrap()
        }
    }

    fn push_list(&mut self, id: I, key: K)
    {
        let pos = self.n_bheap.into() + self.n_list.into();
        self.array.write(pos, Some((id, key)));

        self.n_list = (self.n_list.into() + 1).try_into().ok().unwrap();
    }

    fn pop_list(&mut self)
    {
        self.n_list = (self.n_list.into() - 1).try_into().ok().unwrap();

        let pos = self.n_bheap.into() + self.n_list.into();
        self.array.write(pos, None);
    }

    fn replace<U1, U2>(&mut self, pos0: U1, pos1: U2)
    where U1: TryInto<I>, U2: TryInto<I>
    {
        let pos0 = pos0.try_into().ok().unwrap();
        let pos1 = pos1.try_into().ok().unwrap();
        let tmp0 = self.array.refer(pos0).take();
        let tmp1 = self.array.refer(pos1).take();
        self.array.write(pos0, tmp1);
        self.array.write(pos1, tmp0);
    }

    fn up_bheap(&mut self)
    {
        let mut pos = self.n_bheap.into() - 1;

        while pos > 0 {
            let parent = (pos - 1) / 2;

            let key_pos = &self.array.refer(pos).as_ref().unwrap().1;
            let key_parent = &self.array.refer(parent).as_ref().unwrap().1;

            if key_pos >= key_parent {
                break;
            }

            self.replace(pos, parent);
            pos = parent;
        }
    }

    fn down_bheap(&mut self)
    {
        let mut pos = 0_usize;

        while pos < self.n_bheap.into() / 2 {
            let child0 = pos * 2 + 1;
            let child1 = pos * 2 + 2;

            let key_pos = &self.array.refer(pos).as_ref().unwrap().1;
            let key_child0 = &self.array.refer(child0).as_ref().unwrap().1;

            let (child, key_child) = if child1 < self.n_bheap.into() {
                let key_child1 = &self.array.refer(child1).as_ref().unwrap().1;

                if key_child0 <= key_child1 {
                    (child0, key_child0)
                }
                else {
                    (child1, key_child1)
                }
            }
            else {
                (child0, key_child0)
            };

            if key_pos < key_child {
                break;
            }

            self.replace(pos, child);
            pos = child;
        }
    }

    pub fn add_bheap(&mut self, id: I, key: K)
    {
        self.push_list(id, key);

        let pos = self.n_bheap.into() + self.n_list.into() - 1;
        self.list_to_bheap(pos.try_into().ok().unwrap());
    }

    pub fn list_to_bheap(&mut self, pos: I)
    {
        assert!(pos.into() >= self.n_bheap.into());

        // replace pos <=> list head
        self.replace(pos, self.n_bheap.into());

        // list head <=> bheap tail
        self.n_list = (self.n_list.into() - 1).try_into().ok().unwrap();
        self.n_bheap = (self.n_bheap.into() + 1).try_into().ok().unwrap();

        // upheap correction
        self.up_bheap();
    }

    pub fn bheap_to_list(&mut self)
    {
        // replace bheap head <=> bheap tail
        let pos1 = self.n_bheap.into() - 1;
        self.replace(0, pos1);

        // bheap tail <=> list head
        self.n_list = (self.n_list.into() + 1).try_into().ok().unwrap();
        self.n_bheap = (self.n_bheap.into() - 1).try_into().ok().unwrap();

        // downheap correction
        self.down_bheap();
    }

    pub fn bheap_round(&mut self)
    {
        self.bheap_to_list();

        self.list_to_bheap(self.n_bheap);
    }

    pub fn remove_bheap(&mut self)
    {
        self.bheap_to_list();

        // replace list head <=> list tail
        let pos1 = self.n_bheap.into() + self.n_list.into() - 1;
        self.replace(self.n_bheap.into(), pos1);
        
        // remove list tail
        self.pop_list();
    }
}

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
    state: MTState
}

struct MTTaskMgr
{
    tasks: MTRawArray<MTTask>,
    task_tree: MTBHeapList<MTTaskId, MTTaskPri>,
    //
    sp_loops: *mut usize,
    tid: Option<MTTaskId>
}

impl MTTaskMgr
{
    fn task_current(&mut self) -> Option<&mut MTTask>
    {
        if let Some(curr_tid) = self.tid {
            Some(self.tasks.refer(curr_tid))
        }
        else {
            None
        }
    }

    // Main context

    fn new(tasks: MTRawArray<MTTask>, task_tree_array: MTRawArray<Option<(MTTaskId, MTTaskPri)>>) -> MTTaskMgr
    {
        for i in 0..tasks.len() {
            tasks.write(i,
                MTTask {
                    sp_start: core::ptr::null_mut(),
                    sp_end: core::ptr::null_mut(),
                    priority: 0,
                    sp: core::ptr::null_mut(),
                    kick_excnt: 0,
                    wakeup_cnt: 0,
                    signal_excnt: 0,
                    wait_cnt: 0,
                    state: MTState::None
                }
            );
        }

        MTTaskMgr {
            tasks,
            task_tree: MTBHeapList::new(task_tree_array),
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

    fn register_once<T>(&mut self, tid: MTTaskId, stack: MTRawArray<usize>, t: T)
    where T: FnOnce() + Send // unsafe lifetime
    {
        let task = self.tasks.refer(tid);

        assert_eq!(task.state, MTState::None); // TODO: better message

        let sp_start = stack.head();
        let sp_end = stack.tail();

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
        let task = self.tasks.refer(tid);

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

        for i in 0.. self.tasks.len() { // TODO: iterator
            let task = self.tasks.refer(i);

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
        for i in 0.. self.tasks.len() { // TODO: task priority
            let task = self.tasks.refer(i);

            if let MTState::Ready = task.state {
                next_tid = Some(i as MTTaskId);
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
        let task = self.tasks.refer(tid);

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

struct MTRawArray<T>
{
    head: *mut T,
    len: usize
}

impl<T> MTRawArray<T>
{
    fn refer<U>(&self, i: U) -> &mut T
    where U: Into<usize>
    {
        let i = i.into();
        assert!(i < self.len); // TODO: better message

        let ptr = self.head;
        let ptr = unsafe { ptr.add(i) };

        unsafe { ptr.as_mut().unwrap() }
    }

    fn write<U>(&self, i: U, v: T)
    where U: Into<usize>
    {
        let i = i.into();
        assert!(i < self.len); // TODO: better message

        let ptr = self.head;
        let ptr = unsafe { ptr.add(i) };

        unsafe { ptr.write(v); }
    }

    fn write_volatile<U>(&self, i: U, v: T)
    where U: Into<usize>
    {
        let i = i.into();
        assert!(i < self.len); // TODO: better message

        let ptr = self.head;
        let ptr = unsafe { ptr.add(i) };

        unsafe { ptr.write_volatile(v); }
    }

    fn head(&self) -> *mut T
    {
        self.head
    }

    fn len(&self) -> usize
    {
        self.len
    }

    fn tail(&self) -> *mut T
    {
        let ptr = self.head;
        let ptr = unsafe { ptr.add(self.len) };
        ptr
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
    fn new<'b, M>(mem: &'b mut MTMemory<M>) -> MTAlloc<'b>
    {
        MTAlloc {
            cur_pos: mem.head(),
            end_cap: mem.head() + mem.size(),
            phantom: PhantomData
        }
    }

    fn array<T, U>(&mut self, len: U) -> MTRawArray<T>
    where U: Into<usize>
    {
        let len = len.into();
        let size = size_of::<T>() * len;

        let p = align_up::<T>(self.cur_pos);
        let e = p + size;

        assert!(e <= self.end_cap); // TODO: better message

        self.cur_pos = e;

        MTRawArray {
            head: p as *mut T,
            len
        }
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

    pub fn create<M>(mem: &mut MTMemory<M>, num_tasks: MTTaskId) -> Minimult
    {
        let mut alloc = MTAlloc::new(mem);

        unsafe {
            O_TASKMGR = Some(MTTaskMgr::new(alloc.array(num_tasks), alloc.array(num_tasks)));
        }

        Minimult {
            alloc
        }
    }

    pub fn msgq<L>(&mut self, len: usize) -> MTMsgQueue<'a, L> // TODO: lifetime is correct?
    {
        let mem = self.alloc.array(len);

        MTMsgQueue::new(mem)
    }

    pub fn register<T>(&mut self, tid: MTTaskId, stack_len: usize, t: T)
    where T: FnOnce() + Send + 'a  // TODO: lifetime is correct?
    {
        let tm = unsafe { O_TASKMGR.as_mut().unwrap() };

        let stack = self.alloc.array(stack_len);
        
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

    pub fn signal(tid: MTTaskId)
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

    pub fn curr_tid() -> Option<MTTaskId>
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
    mem: MTRawArray<Option<L>>,
    wr_idx: usize,
    rd_idx: usize,
    wr_tid: Option<MTTaskId>,
    rd_tid: Option<MTTaskId>,
    phantom: PhantomData<&'a ()>
}

impl<'a, L> MTMsgQueue<'a, L>
{
    fn new(mem: MTRawArray<Option<L>>) -> MTMsgQueue<'a, L> // TODO: lifetime is correct?
    {
        MTMsgQueue {
            mem,
            wr_idx: 0,
            rd_idx: 0,
            wr_tid: None,
            rd_tid: None,
            phantom: PhantomData
        }
    }

    pub fn ch<'q>(&'q mut self) -> (MTMsgSender<'a, 'q, L>, MTMsgReceiver<'a, 'q, L>)
    {
        (
            MTMsgSender {
                q: self,
                phantom: PhantomData
            },
            MTMsgReceiver {
                q: self,
                phantom: PhantomData
            }
        )
    }
}

//

pub struct MTMsgSender<'a, 'q, L>
{
    q: *mut MTMsgQueue<'a, L>,
    phantom: PhantomData<&'q ()>
}

unsafe impl<L> Send for MTMsgSender<'_, '_, L> {}

impl<L> MTMsgSender<'_, '_, L>
{
    pub fn vacant(&self) -> usize
    {
        let q = unsafe { self.q.as_mut().unwrap() };

        q.wr_tid = Minimult::curr_tid();

        wrap_diff(q.rd_idx, wrap_inc(q.wr_idx, q.mem.len()), q.mem.len())
    }

    pub fn send(&self, v: L)
    {
        let q = unsafe { self.q.as_mut().unwrap() };

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

        q.mem.write_volatile(curr_wr_idx, Some(v));

        q.wr_idx = next_wr_idx;

        if let Some(rd_tid) = q.rd_tid {
            Minimult::signal(rd_tid);
        }
    }
}

//

pub struct MTMsgReceiver<'a, 'q, L>
{
    q: *mut MTMsgQueue<'a, L>,
    phantom: PhantomData<&'q ()>
}

unsafe impl<L> Send for MTMsgReceiver<'_, '_, L> {}

impl<L> MTMsgReceiver<'_, '_, L>
{
    pub fn available(&self) -> usize
    {
        let q = unsafe { self.q.as_mut().unwrap() };

        q.rd_tid = Minimult::curr_tid();

        wrap_diff(q.wr_idx, q.rd_idx, q.mem.len())
    }

    pub fn receive<F>(&self, f: F)
    where F: FnOnce(&L)
    {
        let q = unsafe { self.q.as_mut().unwrap() };

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

        let ptr = q.mem.refer(curr_rd_idx);

        f(ptr.as_ref().unwrap());
        ptr.take().unwrap();

        q.rd_idx = next_rd_idx;

        if let Some(wr_tid) = q.wr_tid {
            Minimult::signal(wr_tid);
        }
    }
} 
