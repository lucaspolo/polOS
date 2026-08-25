#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

use core::panic::PanicInfo;
use pol_os::serial_print;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    serial_print!("stack_overflow::stack_overflow...\t");

    pol_os::gdt::init();
    init_test_idt();

    stack_overflow();

    panic!("Execution continued after stack overflow");
}

#[allow(unconditional_recursion)]
fn stack_overflow() {
    stack_overflow();
    // Leitura volátil de um valor descartado: impede o compilador de transformar a
    // recursão acima em laço (tail call) e nunca estourar a pilha. Antes isto era
    // `volatile::Volatile::new(0).read()`, tipo que o volatile 0.5+ removeu.
    let probe = 0u8;
    unsafe { core::ptr::read_volatile(&raw const probe) };
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    pol_os::test_panic_handler(info)
}

use spin::LazyLock;
use x86_64::structures::idt::InterruptDescriptorTable;

static TEST_IDT: LazyLock<InterruptDescriptorTable> = LazyLock::new(|| {
    let mut idt = InterruptDescriptorTable::new();
    unsafe {
        idt.double_fault
            .set_handler_fn(test_double_fault_handler)
            .set_stack_index(pol_os::gdt::DOUBLE_FAULT_IST_INDEX);
    }

    idt
});

pub fn init_test_idt() {
    TEST_IDT.load();
}

use pol_os::{exit_qemu, serial_println, QemuExitCode};
use x86_64::structures::idt::InterruptStackFrame;

extern "x86-interrupt" fn test_double_fault_handler(
    _stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    serial_println!("[ok]");
    exit_qemu(QemuExitCode::Success);
    loop {}
}
