use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

use crate::{gdt, println};

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);

        // workaround due to a regression in nightly
        // https://github.com/rust-lang/rust/issues/143072
        let double_fault_handler_ptr =
            double_fault_handler as extern "x86-interrupt" fn(InterruptStackFrame, u64);
        unsafe {
            idt.double_fault
                .set_handler_fn(core::mem::transmute(double_fault_handler_ptr))
                .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        }

        idt
    };
}

pub fn init_idt() {
    IDT.load();
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn double_fault_handler(stack_frame: InterruptStackFrame, _error_code: u64) {
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
}
