use crate::println;

use super::*;
use alloc::collections::BTreeMap;
use core::arch::asm;
use crossbeam_queue::ArrayQueue;
use x86_64::{
    structures::idt::{InterruptStackFrame, InterruptStackFrameValue},
    VirtAddr,
};

pub struct RoundRobinScheduler {
    current_task_id: TaskId,
    tasks: BTreeMap<TaskId, Task>,
    tasks_queue: ArrayQueue<TaskId>,
}

impl RoundRobinScheduler {
    pub fn init() -> Self {
        let mut tasks = BTreeMap::new();
        let id = TaskId::new();
        tasks.insert(
            id,
            Task {
                context: TaskContext::default(),
            },
        );
        Self {
            current_task_id: id,
            tasks,
            tasks_queue: ArrayQueue::new(100),
        }
    }

    pub fn schedule(&mut self) {
        if let Some(task_id) = self.tasks_queue.pop() {
            // println!("{:?} -> {:?}", self.current_task_id, task_id);
            self.tasks_queue.push(self.current_task_id).unwrap();
            if self.tasks.get(&task_id).unwrap().context.rax == 0 {
                let rip = self.tasks.get(&task_id).unwrap().context.rip;
                self.tasks.get_mut(&task_id).unwrap().context = self
                    .tasks
                    .get(&self.current_task_id)
                    .unwrap()
                    .context
                    .clone();
                self.tasks.get_mut(&task_id).unwrap().context.rip = rip;
            }
            self.current_task_id = task_id;
        }
    }

    pub fn spawn(&mut self, entry_point: fn()) {
        let id = TaskId::new();
        let mut context = TaskContext::default();
        context.rip = entry_point as u64;
        context.cs = 0x08;
        context.rflags = 0x202;
        // context.rsp = stack_addr;
        // context.ss = 0x10;
        let task = Task { context };

        self.tasks.insert(id, task);
        if let Err(_) = self.tasks_queue.push(id) {
            panic!("task queue full");
        }
    }

    pub unsafe fn save_context(&mut self, stack_frame: &InterruptStackFrame) {
        let current_task = self.tasks.get_mut(&self.current_task_id).unwrap();
        asm!(
            "mov [{0} + 0], rax",
            "mov [{0} + 8], rbx",
            "mov [{0} + 16], rcx",
            "mov [{0} + 24], rdx",
            "mov [{0} + 40], rsi",
            "mov [{0} + 32], rdi",
            "mov [{0} + 48], rbp",
            "mov [{0} + 56], r8",
            "mov [{0} + 64], r9",
            "mov [{0} + 72], r10",
            "mov [{0} + 80], r11",
            "mov [{0} + 88], r12",
            "mov [{0} + 96], r13",
            "mov [{0} + 104], r14",
            "mov [{0} + 112], r15",
            // "mov rax, {1}",
            // "mov [{0} + 120], rax",
            // "mov rax, {2}",
            // "mov [{0} + 128], rax",
            // "mov rax, {3}",
            // "mov [{0} + 136], rax",
            // "mov rax, {4}",
            // "mov [{0} + 144], rax",
            // "mov rax, {5}",
            // "mov [{0} + 148], rax",
            in(reg) &mut current_task.context as *mut _ as usize,
            // in(reg) stack_frame.instruction_pointer.as_u64(),
            // in(reg) stack_frame.code_segment,
            // in(reg) stack_frame.cpu_flags,
            // in(reg) stack_frame.stack_pointer.as_u64(),
            // in(reg) stack_frame.stack_segment,
            // out("rax") _
        );
        current_task.context.rip = stack_frame.instruction_pointer.as_u64();
        current_task.context.cs = stack_frame.code_segment;
        current_task.context.rflags = stack_frame.cpu_flags;
        current_task.context.rsp = stack_frame.stack_pointer.as_u64();
        current_task.context.ss = stack_frame.stack_segment;
        // println!(
        //     "{:?}",
        //     self.tasks.get(&self.current_task_id).unwrap().context
        // );
    }

    pub unsafe fn load_context(&self, stack_frame: &mut InterruptStackFrame) {
        let current_task = self.tasks.get(&self.current_task_id).unwrap();
        // Needs to happen before otherwise the registers are already changed
        stack_frame.as_mut().write(InterruptStackFrameValue {
            stack_segment: current_task.context.ss,
            stack_pointer: VirtAddr::new(current_task.context.rsp),
            cpu_flags: current_task.context.rflags,
            code_segment: current_task.context.cs,
            instruction_pointer: VirtAddr::new(current_task.context.rip),
        });
        // println!("{:?}", current_task.context);
        asm!(
            "mov rax, [{0} + 0]",
            "mov rbx, [{0} + 8]",
            "mov rcx, [{0} + 16]",
            "mov rdx, [{0} + 24]",
            "mov rdi, [{0} + 32]",
            "mov rsi, [{0} + 40]",
            "mov rbp, [{0} + 48]",
            "mov r8, [{0} + 56]",
            "mov r9, [{0} + 64]",
            "mov r10, [{0} + 72]",
            "mov r11, [{0} + 80]",
            "mov r12, [{0} + 88]",
            "mov r13, [{0} + 96]",
            "mov r14, [{0} + 104]",
            "mov r15, [{0} + 112]",
            in(reg) &current_task.context as *const _ as usize,
        );
        // println!("still standing");
    }
}
