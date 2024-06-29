use crate::{hlt_loop, println};

use super::*;
use core::arch::asm;
use x86_64::{
    structures::idt::{InterruptStackFrame, InterruptStackFrameValue},
    VirtAddr,
};

pub struct RoundRobinScheduler {
    current_task: Task,
}

impl RoundRobinScheduler {
    pub fn init() -> Self {
        Self {
            current_task: Task {
                context: TaskContext::default(),
                id: TaskId::new(),
            },
        }
    }

    pub fn schedule(&mut self) {}

    pub unsafe fn save_context(&mut self, stack_frame: &InterruptStackFrame) {
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
            in(reg) &mut self.current_task.context as *mut _ as usize,
            // in(reg) stack_frame.instruction_pointer.as_u64(),
            // in(reg) stack_frame.code_segment,
            // in(reg) stack_frame.cpu_flags,
            // in(reg) stack_frame.stack_pointer.as_u64(),
            // in(reg) stack_frame.stack_segment,
            // out("rax") _
        );
        self.current_task.context.rip = stack_frame.instruction_pointer.as_u64();
        self.current_task.context.cs = stack_frame.code_segment;
        self.current_task.context.rflags = stack_frame.cpu_flags;
        self.current_task.context.rsp = stack_frame.stack_pointer.as_u64();
        self.current_task.context.ss = stack_frame.stack_segment;
        // println!("{:?}", self.current_task.context);
    }

    pub unsafe fn load_context(&self) -> InterruptStackFrameValue {
        // Needs to happen before otherwise the registers are already changed
        let ret_value = InterruptStackFrameValue {
            stack_segment: self.current_task.context.ss,
            stack_pointer: VirtAddr::new(self.current_task.context.rsp),
            cpu_flags: self.current_task.context.rflags,
            code_segment: self.current_task.context.cs,
            instruction_pointer: VirtAddr::new(self.current_task.context.rip),
        };
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
            in(reg) &self.current_task.context as *const _ as usize,
        );
        ret_value
    }
}
