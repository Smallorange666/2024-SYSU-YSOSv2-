use crate::{input, serial::get_serial_for_sure};

use super::consts::*;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

pub unsafe fn register_idt(idt: &mut InterruptDescriptorTable) {
    idt[Interrupts::IrqBase as usize + Irq::Serial0 as usize].set_handler_fn(serial_handler);
}

pub extern "x86-interrupt" fn serial_handler(_st: InterruptStackFrame) {
    receive();
    super::ack();
}

/// Receive character from uart 16550
/// Should be called on every interrupt
fn receive() {
    // receive character from uart 16550, put it into INPUT_BUFFER
    let mut serial = get_serial_for_sure();
    let data = serial.receive();
    drop(serial);

    if let Some(data) = data {
        input::push_key(data);
    }
}
