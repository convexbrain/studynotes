use cortex_m::peripheral::SCB;

//

extern crate alloc;
use alloc::boxed::Box;

struct BoxedFnOnce(usize, usize);

fn task_start(a0: usize, a1: usize) -> !
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
    // TODO: task state

    loop {}
}

fn align(x: usize) -> usize
{
    let y = (x + core::mem::size_of::<usize>() - 1) / core::mem::size_of::<usize>();
    let y = y * core::mem::size_of::<usize>();
    y
}

//

struct TaskMgr
{
    sp0: usize,
    sp1: usize,
    sp2: usize,
    sp3: usize,
    tid: Option<usize>,
    num_tasks: usize,
}

impl TaskMgr
{
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
        let fn_task_strart = task_start as *const fn(usize, usize) -> !;
        unsafe { *ret_addr = fn_task_strart as usize }

        let xpsr = sp + (8 + 7) * 4;
        let xpsr = xpsr as *mut usize;
        unsafe { *xpsr = 0x01000000 } // xPSR: set T-bit since Cortex-M has only Thumb instructions
    }

    fn register_box<T>(&mut self, tid: usize, sp: usize, t: T)
    where T: FnOnce() + Send + 'static
    {
        let bf: Box<dyn FnOnce()> = Box::new(t);
        let bfo = unsafe { core::mem::transmute::<Box<dyn FnOnce()>, BoxedFnOnce>(bf) };

        let sp = sp - align(128); // TODO: magic number
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

    fn setup_task(sp: usize, data: usize, call_once: usize)
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

    fn register<T>(&mut self, tid: usize, sp: usize, t: T)
    where T: FnOnce() + Send + 'static
    {
        let sz = core::mem::size_of::<T>();
        let rfo = unsafe { core::mem::transmute::<&dyn FnOnce(), RefFnOnce>(&t) };

        let sp = sp - align(sz);
        let data = sp;
        unsafe {
            core::intrinsics::copy(rfo.data as *const u8, data as *mut u8, sz)
        }

        let call_once_addr = (rfo.vtbl + core::mem::size_of::<usize>() * 3) as *const usize; // TODO: magic number
        let call_once = unsafe {*call_once_addr};

        let sp = sp - align(128); // TODO: magic number
        TaskMgr::setup_task(sp, data, call_once);

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
}

//

static mut O_TASKMGR: Option<TaskMgr> = None;

#[no_mangle]
pub extern fn task_switch(curr_sp: usize) -> usize
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


pub struct MTStack<S>
{
    mem: S
}

impl<S> MTStack<S>
{
    pub fn new() -> MTStack<S>
    {
        MTStack {
            mem: unsafe { core::mem::MaybeUninit::<S>::uninit().assume_init() }
        }
    }

    fn size(&self) -> usize
    {
        core::mem::size_of::<S>()
    }

    fn head(&self) -> usize
    {
        let ptr = &self.mem;
        let ptr = ptr as *const S;
        let ptr = ptr as usize;
        ptr
    }
}

#[macro_export]
macro_rules! minimult_stack {
    ($size:expr) => {
        MTStack::<[usize; ($size + core::mem::size_of::<usize>() - 1) / core::mem::size_of::<usize>()]>::new()
    };
}


pub struct Minimult<'a>
{
    tm: TaskMgr,
    phantom: core::marker::PhantomData<&'a ()>
}

impl<'a> Minimult<'a>
{
    pub fn create(cmperi: &mut cortex_m::Peripherals) -> Self
    {
        let tm = TaskMgr {
            sp0: 0,
            sp1: 0,
            sp2: 0,
            sp3: 0,
            tid: None,
            num_tasks: 4,
        };

        let control = cortex_m::register::control::read();
        assert!(control.spsel().is_msp()); // CONTROL.SPSEL: SP_main

        unsafe {
            cmperi.SCB.set_priority(cortex_m::peripheral::scb::SystemHandler::PendSV, 255) // PendSV: lowest priority
        }

        Minimult {
            tm,
            phantom: core::marker::PhantomData
        }
    }

    pub fn register_box<T, S>(mut self, tid: usize, stack: &'a mut MTStack<S>, t: T) -> Minimult<'a>
    where T: FnOnce() + Send + 'static
    {
        let sp = stack.head() + stack.size();
        self.tm.register_box(tid, sp, t);

        self
    }

    pub fn register<T, S>(mut self, tid: usize, stack: &'a mut MTStack<S>, t: T) -> Minimult<'a>
    where T: FnOnce() + Send + 'static
    {
        let sp = stack.head() + stack.size();
        self.tm.register(tid, sp, t);

        self
    }

    pub fn start(self) -> !
    {
        unsafe {
            O_TASKMGR = Some(self.tm);
        }
        
        Minimult::req_task_switch();

        loop {}
    }

    pub fn req_task_switch()
    {
        unsafe {
            if O_TASKMGR.is_none() {
                return;
            }
        }

        SCB::set_pendsv();
    }
}
