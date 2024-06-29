use core::sync::atomic::{AtomicU64, Ordering};
use lazy_static::lazy_static;
use round_robin_scheduler::RoundRobinScheduler;

pub mod round_robin_scheduler;

#[repr(C)]
#[derive(Default, Debug, Clone, Copy)]
struct TaskContext {
    rax: u64,
    rbx: u64,
    rcx: u64,
    rdx: u64,
    rsi: u64,
    rdi: u64,
    rbp: u64,
    r8: u64,
    r9: u64,
    r10: u64,
    r11: u64,
    r12: u64,
    r13: u64,
    r14: u64,
    r15: u64,
    rip: u64,
    cs: u64,
    rflags: u64,
    rsp: u64,
    ss: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct TaskId(u64);

impl TaskId {
    fn new() -> Self {
        static NEXT_ID: AtomicU64 = AtomicU64::new(0);
        TaskId(NEXT_ID.fetch_add(1, Ordering::Relaxed))
    }
}

struct Task {
    context: TaskContext,
}

lazy_static! {
    pub static ref SCHEDULER: spin::Mutex<RoundRobinScheduler> =
        spin::Mutex::new(RoundRobinScheduler::init());
}