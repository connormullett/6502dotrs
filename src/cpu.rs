use crate::{memory::Memory, processor_status::ProcessorStatus};

#[derive(Debug, Default, Clone)]
pub struct Cpu {
    // program counter
    pc: u16,
    // stack pointer
    sp: u16,
    // accumulator
    a: u8,
    // x index register
    x: u8,
    // y index register
    y: u8,
    // processor status (bitfield)
    ps: ProcessorStatus,

    // Memory module
    memory: Memory,
}

impl Cpu {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn reset(&mut self) -> Self {
        self.pc = 0xFFFC;
        self.sp = 0x0100;
        self.a = 0;
        self.x = 0;
        self.y = 0;
        self.ps.clear();

        self.to_owned()
    }
}
