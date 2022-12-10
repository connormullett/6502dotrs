use crate::{
    memory::{self, Memory},
    op_codes::*,
    processor_status::ProcessorStatus,
};

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

    pub fn execute(&mut self) {
        loop {
            let instruction = self.fetch_and_increment_pc();
            match instruction {
                LDA_IM => {
                    let value = self.fetch_and_increment_pc();
                    self.a = value;

                    // set the zero flag
                    if self.a == 0 {
                        self.ps = self.ps | ProcessorStatus::Z;
                    }

                    // set negative flag
                    if self.a & 0b10000000 > 0 {
                        self.ps = self.ps | ProcessorStatus::N;
                    }
                }
                NOP => break,
                _ => {
                    panic!("unrecognized instruction")
                }
            }
        }
    }

    fn fetch_and_increment_pc(&mut self) -> u8 {
        if self.pc as usize > memory::MAX_MEM {
            panic!("PC exceeds max memory allocated {}", memory::MAX_MEM);
        }

        let data = self.memory.data[self.pc as usize];
        self.pc += 1;
        data
    }
}
