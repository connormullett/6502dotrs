#![allow(unused)]
use std::ops::Shr;

use crate::{
    memory::{self, Memory},
    op_codes::*,
    processor_status::ProcessorStatus,
};

#[derive(Debug, Default, Clone)]
pub struct Cpu {
    /// program counter
    pc: u16,
    /// stack pointer
    sp: u16,
    /// accumulator
    a: u8,
    /// x index register
    x: u8,
    /// y index register
    y: u8,
    /// processor status (bitfield)
    ps: ProcessorStatus,

    /// Memory module
    pub memory: Memory,
}

impl Cpu {
    /// construct a new cpu
    pub fn new() -> Self {
        Self::default()
    }

    /// reset the cpu to initial state
    /// an optional address can be given to give the
    /// cpu a location to fetch instructions from after
    /// the reset has finished
    pub fn reset(&mut self, address: Option<u16>) -> Self {
        self.pc = 0xFFFC;
        self.sp = 0x0100;
        self.a = 0;
        self.x = 0;
        self.y = 0;
        self.ps.clear();

        // read 0xFFFC and 0xFFFD and
        // jump to that address for instructions
        if let Some(address) = address {
            self.memory.write_word(self.pc as usize, address);
            self.pc = self.memory.read_word(0xFFFC);
        }

        self.to_owned()
    }

    /// load a program into the cpu's memory at a given address
    pub fn load_program(&mut self, address: usize, program: Vec<u8>) {
        todo!()
    }

    /// execute the program loaded in memory
    pub fn execute(&mut self) {
        loop {
            let instruction = self.fetch_byte();
            match instruction {
                LDA_IM => self.lda_immediate(),
                LDA_ABS => self.lda_absolute(),
                LDA_ABS_X => self.lda_absolute_x_indexed(),
                LDA_ABS_Y => self.lda_absolute_y_indexed(),
                LDA_ZP => self.lda_zp(),
                LDA_ZP_X => self.lda_zp_x(),
                LDA_ZP_XI => self.lda_x_indexed_zero_page_indirect(),
                LDA_ZP_IY => self.lda_y_zero_page_indirect_indexed(),
                LDX_IM => self.ldx_immediate(),
                LDX_ABS => self.ldx_absolute(),
                LDX_ZP => self.ldx_zp(),
                LDX_ZP_Y => self.ldx_y_indexed_zero_page(),
                LDX_ABS_Y => self.ldx_absolute_y_indexed(),
                LDY_IM => self.ldy_immediate(),
                LDY_ABS => self.ldy_absolute(),
                LDY_ZP => self.ldy_zp(),
                LDY_ZP_X => self.ldy_x_indexed_zero_page(),
                LDY_ABS_X => self.ldy_absolute_x_indexed(),
                LSR_ACC => self.lsr_acc(),
                LSR_ABS => self.lsr_abs(),
                LSR_ZP => self.lsr_zp(),
                LSR_ABS_X => self.lsr_abs_x(),
                LSR_ZP_X => self.lsr_zp_x(),
                PHA => self.pha(),
                PHP => self.php(),
                PLA => self.pla(),
                PLP => self.plp(),
                JMP_ABS => self.jump_absolute(),
                JMP_ABS_IND => self.jump_absolute_indirect(),
                JSR => self.jump_subroutine(),
                RTS => self.return_subroutine(),
                ANDA_IM => self.anda_im(),
                ANDA_ABS => self.anda_abs(),
                ANDA_X_ABS => self.anda_abs_x(),
                ANDA_Y_ABS => self.anda_abs_y(),
                ANDA_ZP => self.anda_zp(),
                ANDA_ZP_X => self.anda_zp_x(),
                ANDA_ZP_IY => self.anda_zp_iy(),
                ANDA_ZP_XI => self.anda_zp_xi(),
                ORA_IM => self.ora_im(),
                ORA_ABS => self.ora_abs(),
                ORA_X_ABS => self.ora_abs_x(),
                ORA_Y_ABS => self.ora_abs_y(),
                ORA_ZP => self.ora_zp(),
                ORA_ZP_X => self.ora_zp_x(),
                ORA_ZP_IY => self.ora_zp_iy(),
                ORA_ZP_XI => self.ora_zp_xi(),
                TAX => self.transfer_a_to_x(),
                TAY => self.transfer_a_to_y(),
                TSX => self.transfer_sp_to_x(),
                TXA => self.transfer_x_to_a(),
                TXS => self.transfer_x_to_sp(),
                TYA => self.transfer_y_to_a(),
                SEC => self.set_carry_flag(true),
                SED => self.set_decimal_mode(),
                SEI => self.set_interrupt_disable(),
                NOP => break,
                _ => {
                    self.debug_print();
                    panic!("reason: unrecognized instruction");
                }
            }
        }
    }

    /// print contents of registers, pc, sp, and status flags and current instruction
    /// useful when the emulator crashes, you can get a state of the machine
    pub fn debug_print(&self) {
        println!("pc: 0x{:04x}", self.pc);
        println!("sp: 0x{:04x}", self.sp);
        println!("a : 0x{:04x}", self.a);
        println!("x : 0x{:04x}", self.x);
        println!("y : 0x{:04x}", self.y);
        println!("ps: {}", self.ps);
        println!(
            "current instruction: 0x{:02X}",
            self.memory.read_byte(self.pc as usize)
        );
    }

    /// fetch a word from memory while incrememting the pc each read (2 cycles)
    fn fetch_word(&mut self) -> u16 {
        let mut data = self.memory.data[self.pc as usize] as u16;
        self.pc += 1;

        data |= u16::from(self.memory.data[self.pc as usize]) << 8;
        self.pc += 1;

        data
    }

    /// fetch a byte and increment the pc
    fn fetch_byte(&mut self) -> u8 {
        if self.pc as usize > memory::MAX_MEM {
            panic!("PC exceeds max memory allocated {}", memory::MAX_MEM);
        }

        let data = self.memory.data[self.pc as usize];
        self.pc += 1;
        data
    }

    /* LOAD A INSTRUCTIONS */
    /// load accumulator immediate mode
    fn lda_immediate(&mut self) {
        self.a = self.fetch_byte();
        self.set_negative_and_zero_flags();
    }

    /// load accumulator absolute
    fn lda_absolute(&mut self) {
        let abs_address = self.fetch_word();
        self.a = self.memory.read_byte(abs_address as usize);
        self.set_negative_and_zero_flags();
    }

    /// load accumulator absolute x indexed
    fn lda_absolute_x_indexed(&mut self) {
        let abs_address = self.fetch_word() + self.x as u16;
        self.a = self.memory.read_byte(abs_address as usize);
        self.set_negative_and_zero_flags();
    }

    /// load accumulator absolute y indexed
    fn lda_absolute_y_indexed(&mut self) {
        let abs_address = self.fetch_word() + self.y as u16;
        self.a = self.memory.read_byte(abs_address as usize);
        self.set_negative_and_zero_flags();
    }

    /// load accumulator zero page
    fn lda_zp(&mut self) {
        let zero_page_address = self.fetch_byte();
        self.a = self.memory.read_byte(zero_page_address as usize);
        self.set_negative_and_zero_flags();
    }

    /// load accumulator zero page x indexed
    fn lda_zp_x(&mut self) {
        let zero_page_address = self.fetch_byte();
        self.a = self.memory.read_byte((zero_page_address) as usize) + self.x;
        self.set_negative_and_zero_flags();
    }

    /// load accumulator indexed zero page indirect
    fn lda_x_indexed_zero_page_indirect(&mut self) {
        let indirect_address = self.fetch_byte() + self.x;
        self.a = self.memory.read_byte(indirect_address as usize);
        self.set_negative_and_zero_flags();
    }

    /// load accumulator zero page indirect y indexed
    fn lda_y_zero_page_indirect_indexed(&mut self) {
        let zero_page_address = self.fetch_byte();
        let effective_address = self.memory.read_word(zero_page_address as usize);
        let effective_address_y = effective_address + self.y as u16;
        self.a = self.memory.read_byte(effective_address_y as usize);
        self.set_negative_and_zero_flags();
    }

    /// set zero and negative flags whenever an LDA instruction is executed
    fn set_negative_and_zero_flags(&mut self) {
        // set zero flag
        self.ps.set(ProcessorStatus::Z, self.a == 0);
        self.ps.set(ProcessorStatus::N, (self.a & 0b10000000) > 0);
    }

    /* LOAD X INSTRUCTIONS */
    /// load x index immediate mode
    fn ldx_immediate(&mut self) {
        self.x = self.fetch_byte();
        self.set_negative_and_zero_flags();
    }

    /// load x index absolute mode
    fn ldx_absolute(&mut self) {
        let abs_address = self.fetch_word();
        self.x = self.memory.read_byte(abs_address as usize);
        self.set_negative_and_zero_flags();
    }

    /// load x index from zero page
    fn ldx_zp(&mut self) {
        let zero_page_address = self.fetch_byte();
        self.x = self.memory.read_byte(zero_page_address as usize);
        self.set_negative_and_zero_flags();
    }

    /// load x index y indexed absolute
    fn ldx_absolute_y_indexed(&mut self) {
        let abs_address = self.fetch_word() + self.y as u16;
        self.x = self.memory.read_byte(abs_address as usize);
        self.set_negative_and_zero_flags();
    }

    /// load x index y indexed zero page
    fn ldx_y_indexed_zero_page(&mut self) {
        let zero_page_address = self.fetch_byte();
        self.x = self.memory.read_byte((zero_page_address) as usize) + self.y;
        self.set_negative_and_zero_flags();
    }

    /* LOAD Y INSTRUCTIONS */
    /// load y index immediate mode
    fn ldy_immediate(&mut self) {
        self.y = self.fetch_byte();
        self.set_negative_and_zero_flags();
    }

    /// load y index absolute mode
    fn ldy_absolute(&mut self) {
        let abs_address = self.fetch_word();
        self.y = self.memory.read_byte(abs_address as usize);
        self.set_negative_and_zero_flags();
    }

    /// load y index from zero page
    fn ldy_zp(&mut self) {
        let zero_page_address = self.fetch_byte();
        self.y = self.memory.read_byte(zero_page_address as usize);
        self.set_negative_and_zero_flags();
    }

    /// load y index x indexed absolute
    fn ldy_absolute_x_indexed(&mut self) {
        let abs_address = self.fetch_word() + self.x as u16;
        self.y = self.memory.read_byte(abs_address as usize);
        self.set_negative_and_zero_flags();
    }

    /// load x index y indexed zero page
    fn ldy_x_indexed_zero_page(&mut self) {
        let zero_page_address = self.fetch_byte();
        self.y = self.memory.read_byte((zero_page_address) as usize) + self.x;
        self.set_negative_and_zero_flags();
    }

    fn jump_absolute(&mut self) {
        self.pc = self.fetch_word();
    }

    fn jump_absolute_indirect(&mut self) {
        let indirect_address = self.fetch_word() as usize;
        let low_byte = self.memory.read_byte(indirect_address);

        // do not cross page boundary
        let hi_byte_address = if indirect_address as u8 == 0xFF {
            indirect_address & 0xFF00
        } else {
            indirect_address + 1
        };

        let hi_byte = self.memory.read_byte(hi_byte_address);

        self.pc = u16::from_le_bytes([low_byte, hi_byte as u8]);
    }

    /// jump to a subroutine by pushing the pc onto the stack and modifying the pc
    fn jump_subroutine(&mut self) {
        let sub_address = self.fetch_word();
        self.memory.write_word(self.sp as usize, (self.pc - 1));
        self.sp -= 2;
        self.pc = sub_address;
    }

    /// return from subroutine, taking PC from stack and continuing before the jump
    fn return_subroutine(&mut self) {
        self.sp += 1;
        let pch = self.memory.read_byte(self.sp as usize);
        self.sp += 1;
        let pcl = self.memory.read_byte(self.sp as usize);
        self.pc = (((pch as u16) << 8) | pcl as u16) + 1;
    }

    /* AND Accumulator logical instructions */
    /// AND accumulator immediate mode
    fn anda_im(&mut self) {
        self.a &= self.fetch_byte();
        self.set_negative_and_zero_flags();
    }

    /// AND accumulator absolute mode
    fn anda_abs(&mut self) {
        let absolute_address = self.fetch_word();
        let value = self.memory.read_byte(absolute_address as usize);
        self.a &= value;
        self.set_negative_and_zero_flags();
    }

    /// AND accumulator absolute x indexed
    fn anda_abs_x(&mut self) {
        let absolute_address = self.fetch_word();
        let effective_address = absolute_address + self.x as u16;
        let value = self.memory.read_byte(effective_address as usize);
        self.a &= value;
        self.set_negative_and_zero_flags();
    }

    /// AND accumulator absolute y indexed
    fn anda_abs_y(&mut self) {
        let absolute_address = self.fetch_word();
        let effective_address = absolute_address + self.y as u16;
        let value = self.memory.read_byte(effective_address as usize);
        self.a &= value;
        self.set_negative_and_zero_flags();
    }

    /// AND accumulator zero page
    fn anda_zp(&mut self) {
        let address = self.fetch_byte();
        let value = self.memory.read_byte(address as usize);
        self.a &= value;
        self.set_negative_and_zero_flags();
    }

    /// AND accumulator zero page x indexed
    fn anda_zp_x(&mut self) {
        let address = self.fetch_byte();
        let effective_address = address + self.x;
        let value = self.memory.read_byte(effective_address as usize);
        self.a &= value;
        self.set_negative_and_zero_flags();
    }

    /// AND accumulator zero page indirect y indexed
    fn anda_zp_iy(&mut self) {
        let zero_page_address = self.fetch_byte();
        let indirect_address = self.memory.read_word(zero_page_address as usize) + self.y as u16;
        let value = self.memory.read_byte(indirect_address as usize);
        self.a &= value;
        self.set_negative_and_zero_flags();
    }

    /// AND accumulator zero page x indexed indirect
    fn anda_zp_xi(&mut self) {
        let address = self.fetch_byte();
        let indirect_address = address + self.x;
        let effective_address = self.memory.read_word(indirect_address as usize);
        let value = self.memory.read_byte(effective_address as usize);
        self.a &= value;
        self.set_negative_and_zero_flags();
    }

    /* OR Accumulator logical instructions */
    /// OR accumulator immediate mode
    fn ora_im(&mut self) {
        self.a |= self.fetch_byte();
        self.set_negative_and_zero_flags();
    }

    /// OR accumulator absolute mode
    fn ora_abs(&mut self) {
        let absolute_address = self.fetch_word();
        let value = self.memory.read_byte(absolute_address as usize);
        self.a |= value;
        self.set_negative_and_zero_flags();
    }

    /// OR accumulator absolute x indexed
    fn ora_abs_x(&mut self) {
        let absolute_address = self.fetch_word();
        let effective_address = absolute_address + self.x as u16;
        let value = self.memory.read_byte(effective_address as usize);
        self.a |= value;
        self.set_negative_and_zero_flags();
    }

    /// OR accumulator absolute y indexed
    fn ora_abs_y(&mut self) {
        let absolute_address = self.fetch_word();
        let effective_address = absolute_address + self.y as u16;
        let value = self.memory.read_byte(effective_address as usize);
        self.a |= value;
        self.set_negative_and_zero_flags();
    }

    /// OR accumulator zero page
    fn ora_zp(&mut self) {
        let address = self.fetch_byte();
        let value = self.memory.read_byte(address as usize);
        self.a |= value;
        self.set_negative_and_zero_flags();
    }

    /// OR accumulator zero page x indexed
    fn ora_zp_x(&mut self) {
        let address = self.fetch_byte();
        let effective_address = address + self.x;
        let value = self.memory.read_byte(effective_address as usize);
        self.a |= value;
        self.set_negative_and_zero_flags();
    }

    /// OR accumulator zero page indirect y indexed
    fn ora_zp_iy(&mut self) {
        let zero_page_address = self.fetch_byte();
        let indirect_address = self.memory.read_word(zero_page_address as usize) + self.y as u16;
        let value = self.memory.read_byte(indirect_address as usize);
        self.a |= value;
        self.set_negative_and_zero_flags();
    }

    /// OR accumulator zero page x indexed indirect
    fn ora_zp_xi(&mut self) {
        let address = self.fetch_byte();
        let indirect_address = address + self.x;
        let effective_address = self.memory.read_word(indirect_address as usize);
        let value = self.memory.read_byte(effective_address as usize);
        self.a |= value;
        self.set_negative_and_zero_flags();
    }

    /* logical shift right instructions */
    /// logical shift right accumulator mode
    fn lsr_acc(&mut self) {
        let carry = self.a & 1;
        self.a >>= 1;
        self.set_negative_and_zero_flags();
        self.set_carry_flag(carry > 0);
    }

    /// logical shift right absolute mode
    fn lsr_abs(&mut self) {
        let abs_address = self.fetch_word() as usize;
        let mut data = self.memory.read_byte(abs_address);

        let carry = data & 1;
        data >>= 1;

        self.memory.write_byte(abs_address, data);

        // set flags
        self.ps.set(ProcessorStatus::N, false);
        self.ps.set(ProcessorStatus::Z, data == 0);
        self.set_carry_flag(carry > 0);
    }

    /// logical shift right zero page
    fn lsr_zp(&mut self) {
        let zero_page_address = self.fetch_byte() as usize;
        let mut data = self.memory.read_byte(zero_page_address);

        let carry = data & 1;
        data >>= 1;
        self.memory.write_byte(zero_page_address, data);

        // set flags
        self.ps.set(ProcessorStatus::N, false);
        self.ps.set(ProcessorStatus::Z, data == 0);
        self.set_carry_flag(carry > 0);
    }

    /// logical shift right absolute x indexed
    fn lsr_abs_x(&mut self) {
        let abs_address = self.fetch_word() as usize;
        let effective_address = abs_address + self.x as usize;

        let mut data = self.memory.read_byte(effective_address);
        let carry = data & 1;
        data >>= 1;

        self.memory.write_byte(effective_address, data);

        // set flags
        self.ps.set(ProcessorStatus::N, false);
        self.ps.set(ProcessorStatus::Z, data == 0);
        self.set_carry_flag(carry > 0);
    }

    /// logical shift right zero page x indexed
    fn lsr_zp_x(&mut self) {
        let zero_page_address = self.fetch_byte() as usize;
        let effective_address = zero_page_address + self.x as usize;
        let data = self.memory.read_byte(effective_address);

        self.memory.write_byte(effective_address, data >> 1);

        self.set_negative_and_zero_flags();
        self.set_carry_flag((data & 1) > 0);
    }

    /// sets the carry bit if flag is true in processor status register
    fn set_carry_flag(&mut self, flag: bool) {
        self.ps.set(ProcessorStatus::C, flag);
    }

    /// set decimal mode
    /// This is a no-op and is not supported but is here for completeness
    fn set_decimal_mode(&self) {}

    /// sets the interupt disable flag to true
    fn set_interrupt_disable(&mut self) {
        self.ps.set(ProcessorStatus::I, true);
    }

    /// push accumulator on the stack
    fn pha(&mut self) {
        self.memory.write_byte(self.sp as usize, self.a);
        self.sp -= 1;
    }

    /// push processor status on the stack
    fn php(&mut self) {
        self.memory.write_byte(self.sp as usize, self.ps.bits());
        self.sp -= 1;
    }

    /// pop accumulator from stack
    fn pla(&mut self) {
        self.sp += 1;
        self.a = self.memory.read_byte(self.sp as usize);
        self.set_negative_and_zero_flags();
    }

    /// pop processor status from stack
    fn plp(&mut self) {
        self.sp += 1;
        let ps = self.memory.read_byte(self.sp as usize);
        self.ps = ProcessorStatus::from_bits_truncate(ps);
    }

    /* Implied transfer instructions */
    /// transfer accumulator to index x
    fn transfer_a_to_x(&mut self) {
        self.x = self.a;

        self.ps.set(ProcessorStatus::Z, self.x == 0);
        self.ps.set(ProcessorStatus::N, (self.x & 0x80) > 0);
    }

    /// transfer accumulator to index y
    fn transfer_a_to_y(&mut self) {
        self.y = self.a;

        self.ps.set(ProcessorStatus::Z, self.y == 0);
        self.ps.set(ProcessorStatus::N, (self.y & 0x80) > 0);
    }

    /// transfer stack pointer to x
    fn transfer_sp_to_x(&mut self) {
        // TODO: stack is a fixed area of memory at 0x0100 to 0x01FF
        // but is represented as 16 bits. sp should be u8 and
        // compensate for the high byte when pushing/pulling
        self.x = self.sp as u8;

        self.ps.set(ProcessorStatus::Z, self.x == 0);
        self.ps.set(ProcessorStatus::N, (self.x & 0x80) > 0);
    }

    /// transfer index x to accumulator
    fn transfer_x_to_a(&mut self) {
        self.a = self.x;
        self.set_negative_and_zero_flags();
    }

    /// transfer index x to stack pointer
    fn transfer_x_to_sp(&mut self) {
        self.sp = 0x0100 | (self.x as u16);
    }

    /// transfer index y to accumulator
    fn transfer_y_to_a(&mut self) {
        self.a = self.y;
        self.set_negative_and_zero_flags();
    }

    /// no-op (do nothing)
    fn nop(&mut self) {}
}

#[cfg(test)]
mod tests {
    use super::Cpu;
    use crate::op_codes::*;
    use crate::processor_status::ProcessorStatus;

    #[test]
    fn new_cpu_should_initialize_defaults() {
        let cpu = Cpu::new().reset(None);
        assert_eq!(cpu.pc, 0xFFFC);
    }

    #[test]
    fn reset_cpu_with_address_should_fetch_from_correct_address() {
        let cpu = Cpu::new().reset(0x0010.into());
        assert_eq!(cpu.pc, 0x0010);
    }

    #[test]
    fn set_carry_flag_should_set_correct_bit() {
        let mut cpu = Cpu::new().reset(None);
        cpu.set_carry_flag(true);
        assert_eq!(cpu.ps, ProcessorStatus::C)
    }

    #[test]
    fn write_word_should_write_correct_data_to_memory() {
        let data: u16 = 0b1111111100000000;
        let mut cpu = Cpu::new().reset(None);
        cpu.memory.write_word(0xFFFC, data);
        let word = cpu.memory.read_word(0xFFFC);
        assert_eq!(word, data);
    }

    #[test]
    fn jump_absolute_should_set_pc_to_correct_address() {
        let mut cpu = Cpu::new().reset(0x0001.into());

        cpu.memory.data[0x0001] = JMP_ABS;
        cpu.memory.data[0x0002] = 0xBB;
        cpu.memory.data[0x0003] = 0xBB;
        cpu.memory.data[0xBBBB] = LDA_IM;
        cpu.memory.data[0xBBBC] = 0xFF;
        cpu.memory.data[0xBBBD] = NOP;

        cpu.execute();
        assert_eq!(cpu.a, 0xFF);
    }

    #[test]
    fn jump_absolute_indirect_should_set_pc_correctly() {
        let mut cpu = Cpu::new().reset(0x0001.into());

        cpu.memory.data[0x0001] = JMP_ABS_IND;
        cpu.memory.data[0x0002] = 0xBB;
        cpu.memory.data[0x0003] = 0xBB; // JMP ($BBBB)

        cpu.memory.data[0xBBBB] = 0xDD;
        cpu.memory.data[0xBBBC] = 0xDD;
        cpu.memory.data[0xDDDD] = LDA_IM;
        cpu.memory.data[0xDDDE] = 0xFF;
        cpu.memory.data[0xDDDF] = NOP;

        cpu.execute();
        assert_eq!(cpu.a, 0xFF);
    }

    #[test]
    fn jump_absolute_indirect_should_not_cross_page_boundary() {
        let mut cpu = Cpu::new().reset(0x0001.into());

        cpu.memory.data[0x0001] = JMP_ABS_IND;
        cpu.memory.data[0x0002] = 0xFF;
        cpu.memory.data[0x0003] = 0xAA; // JMP ($AAFF)

        cpu.memory.data[0xAAFF] = 0xBB;
        cpu.memory.data[0xAA00] = 0xBB; // shouldn't cross page boundary

        cpu.memory.data[0xBBBB] = LDA_IM;
        cpu.memory.data[0xBBBC] = 0xFF;
        cpu.memory.data[0xBBBD] = NOP;

        cpu.execute();
        assert_eq!(cpu.a, 0xFF);
    }

    #[test]
    fn transfer_a_to_x() {
        let mut cpu = Cpu::new().reset(0x0001.into());
        cpu.a = 0xFF;

        cpu.memory.data[0x0001] = TAX;
        cpu.memory.data[0x0002] = NOP;

        cpu.execute();
        assert_eq!(cpu.x, 0xFF);
    }

    #[test]
    fn transfer_a_to_y() {
        let mut cpu = Cpu::new().reset(0x0001.into());
        cpu.a = 0xFF;

        cpu.memory.data[0x0001] = TAY;
        cpu.memory.data[0x0002] = NOP;

        cpu.execute();
        assert_eq!(cpu.y, 0xFF);
    }

    #[test]
    fn transfer_sp_to_x() {
        let mut cpu = Cpu::new().reset(0x0001.into());
        cpu.sp = 0x0101;

        cpu.memory.data[0x0001] = TSX;
        cpu.memory.data[0x0002] = NOP;

        cpu.execute();
        assert_eq!(cpu.x, 0x01);
    }

    #[test]
    fn transfer_x_to_a() {
        let mut cpu = Cpu::new().reset(0x0001.into());
        cpu.x = 0xFF;

        cpu.memory.data[0x0001] = TXA;
        cpu.memory.data[0x0002] = NOP;

        cpu.execute();
        assert_eq!(cpu.a, 0xFF);
    }

    #[test]
    fn transfer_y_to_a() {
        let mut cpu = Cpu::new().reset(0x0001.into());
        cpu.y = 0xFF;

        cpu.memory.data[0x0001] = TYA;
        cpu.memory.data[0x0002] = NOP;

        cpu.execute();
        assert_eq!(cpu.a, 0xFF);
    }

    #[test]
    fn transfer_x_to_sp() {
        let mut cpu = Cpu::new().reset(0x0001.into());
        cpu.x = 0xAA;

        cpu.memory.data[0x0001] = TXS;
        cpu.memory.data[0x0002] = NOP;

        cpu.execute();
        assert_eq!(cpu.sp, 0x01AA);
    }

    #[test]
    fn set_carry_flag_should_set_carry_flag() {
        let mut cpu = Cpu::new().reset(0x0001.into());

        cpu.memory.data[0x0001] = SEC;
        cpu.memory.data[0x0002] = NOP;

        cpu.execute();
        assert_eq!(cpu.ps, ProcessorStatus::C);
    }

    #[test]
    fn set_decimal_mode_should_do_nothing() {
        let mut cpu = Cpu::new().reset(0x0001.into());

        cpu.memory.data[0x0001] = SED;
        cpu.memory.data[0x0002] = NOP;

        cpu.execute();
        assert_eq!(cpu.ps, ProcessorStatus::empty());
    }

    #[test]
    fn set_interrupt_disable_should_set_interrupt_flag() {
        let mut cpu = Cpu::new().reset(0x0001.into());

        cpu.memory.data[0x0001] = SEI;
        cpu.memory.data[0x0002] = NOP;

        cpu.execute();
        assert_eq!(cpu.ps, ProcessorStatus::I);
    }

    #[test]
    fn anda_immediate_should_perform_bitwise_or_correctly() {
        let mut cpu = Cpu::new().reset(0x0001.into());

        cpu.memory.data[0x0001] = LDA_IM;
        cpu.memory.data[0x0002] = 0xFF;
        cpu.memory.data[0x0003] = ANDA_IM;
        cpu.memory.data[0x0004] = 0xFF;
        cpu.memory.data[0x0005] = NOP;

        cpu.execute();

        assert_eq!(cpu.a, 0xFF);
    }

    #[test]
    fn anda_absolute_should_perform_bitwise_or_correctly() {
        let mut cpu = Cpu::new().reset(0x0001.into());

        cpu.memory.data[0x0020] = 0xFF;
        cpu.memory.data[0x0001] = LDA_IM;
        cpu.memory.data[0x0002] = 0xFF;
        cpu.memory.data[0x0003] = ORA_ABS;
        cpu.memory.data[0x0004] = 0x20;
        cpu.memory.data[0x0005] = 0x00;
        cpu.memory.data[0x0006] = NOP;

        cpu.execute();

        assert_eq!(cpu.a, 0xFF);
    }

    #[test]
    fn anda_absolute_x_indexed_should_perform_bitwise_or_correctly() {
        let mut cpu = Cpu::new().reset(0x0001.into());
        cpu.x = 0x01;

        cpu.memory.data[0x0021] = 0xFF;
        cpu.memory.data[0x0001] = LDA_IM;
        cpu.memory.data[0x0002] = 0xFF;
        cpu.memory.data[0x0003] = ORA_X_ABS;
        cpu.memory.data[0x0004] = 0x20;
        cpu.memory.data[0x0005] = 0x00;
        cpu.memory.data[0x0006] = NOP;

        cpu.execute();

        assert_eq!(cpu.a, 0xFF);
    }

    #[test]
    fn anda_absolute_y_indexed_should_perform_bitwise_or_correctly() {
        let mut cpu = Cpu::new().reset(0x0001.into());
        cpu.y = 0x01;

        cpu.memory.data[0x0021] = 0xFF;
        cpu.memory.data[0x0001] = LDA_IM;
        cpu.memory.data[0x0002] = 0xFF;
        cpu.memory.data[0x0003] = ORA_Y_ABS;
        cpu.memory.data[0x0004] = 0x20;
        cpu.memory.data[0x0005] = 0x00;
        cpu.memory.data[0x0006] = NOP;

        cpu.execute();

        assert_eq!(cpu.a, 0xFF);
    }

    #[test]
    fn anda_zero_page_should_perform_bitwise_or_correctly() {
        let mut cpu = Cpu::new().reset(0x0001.into());

        cpu.memory.data[0x0001] = LDA_IM;
        cpu.memory.data[0x0002] = 0xFF;
        cpu.memory.data[0x0003] = ORA_ZP;
        cpu.memory.data[0x0004] = 0xF0;
        cpu.memory.data[0x00F0] = 0xFF;
        cpu.memory.data[0x0005] = NOP;

        cpu.execute();

        assert_eq!(cpu.a, 0xFF);
    }

    #[test]
    fn anda_zero_page_x_indexed_should_perform_bitwise_or_correctly() {
        let mut cpu = Cpu::new().reset(0x0001.into());
        cpu.x = 0x01;

        cpu.memory.data[0x00F1] = 0xFF;
        cpu.memory.data[0x0001] = LDA_IM;
        cpu.memory.data[0x0002] = 0xFF;
        cpu.memory.data[0x0003] = ORA_ZP_X;
        cpu.memory.data[0x0004] = 0xF0;
        cpu.memory.data[0x0005] = NOP;

        cpu.execute();
        assert_eq!(cpu.a, 0xFF);
    }

    #[test]
    fn anda_zero_page_indirect_y_indexed_should_perform_bitwise_or_correctly() {
        let mut cpu = Cpu::new().reset(0x0001.into());
        cpu.a = 0xFF;
        cpu.y = 0x01;

        cpu.memory.data[0x0011] = 0x00;
        cpu.memory.data[0x0012] = 0xFF;
        cpu.memory.data[0xFF01] = 0xFF;

        cpu.memory.data[0x0001] = ORA_ZP_IY;
        cpu.memory.data[0x0002] = 0x11;
        cpu.memory.data[0x0003] = NOP;

        cpu.execute();
        assert_eq!(cpu.a, 0xFF);
    }

    #[test]
    fn anda_zero_page_x_indexed_indirect_should_perform_bitwise_or_correctly() {
        let mut cpu = Cpu::new().reset(0x0001.into());
        cpu.a = 0xFF;
        cpu.x = 0x01;

        cpu.memory.data[0x0012] = 0x00;
        cpu.memory.data[0x0013] = 0xFF;
        cpu.memory.data[0xFF00] = 0xFF;

        cpu.memory.data[0x0001] = ORA_ZP_XI;
        cpu.memory.data[0x0002] = 0x11;
        cpu.memory.data[0x0003] = NOP;

        cpu.execute();
        assert_eq!(cpu.a, 0xFF);
    }

    #[test]
    fn ora_immediate_should_perform_bitwise_or_correctly() {
        let mut cpu = Cpu::new().reset(0x0001.into());

        cpu.memory.data[0x0001] = LDA_IM;
        cpu.memory.data[0x0002] = 0b0101_0101;
        cpu.memory.data[0x0003] = ORA_IM;
        cpu.memory.data[0x0004] = 0b1010_1010;
        cpu.memory.data[0x0005] = NOP;

        cpu.execute();

        assert_eq!(cpu.a, 0xFF);
    }

    #[test]
    fn ora_absolute_should_perform_bitwise_or_correctly() {
        let mut cpu = Cpu::new().reset(0x0001.into());

        cpu.memory.data[0x0020] = 0x42;
        cpu.memory.data[0x0001] = LDA_IM;
        cpu.memory.data[0x0002] = 0x55;
        cpu.memory.data[0x0003] = ORA_ABS;
        cpu.memory.data[0x0004] = 0x20;
        cpu.memory.data[0x0005] = 0x00;
        cpu.memory.data[0x0006] = NOP;

        cpu.execute();

        assert_eq!(cpu.a, 0x57); // 0x42 | 0x55 = 0x57
    }

    #[test]
    fn ora_absolute_x_indexed_should_perform_bitwise_or_correctly() {
        let mut cpu = Cpu::new().reset(0x0001.into());
        cpu.x = 0x01;

        cpu.memory.data[0x0021] = 0x42;
        cpu.memory.data[0x0001] = LDA_IM;
        cpu.memory.data[0x0002] = 0x55;
        cpu.memory.data[0x0003] = ORA_X_ABS;
        cpu.memory.data[0x0004] = 0x20;
        cpu.memory.data[0x0005] = 0x00;
        cpu.memory.data[0x0006] = NOP;

        cpu.execute();

        assert_eq!(cpu.a, 0x57); // 0x42 | 0x55 = 0x57
    }

    #[test]
    fn ora_absolute_y_indexed_should_perform_bitwise_or_correctly() {
        let mut cpu = Cpu::new().reset(0x0001.into());
        cpu.y = 0x01;

        cpu.memory.data[0x0021] = 0x42;
        cpu.memory.data[0x0001] = LDA_IM;
        cpu.memory.data[0x0002] = 0x55;
        cpu.memory.data[0x0003] = ORA_Y_ABS;
        cpu.memory.data[0x0004] = 0x20;
        cpu.memory.data[0x0005] = 0x00;
        cpu.memory.data[0x0006] = NOP;

        cpu.execute();

        assert_eq!(cpu.a, 0x57); // 0x42 | 0x55 = 0x57
    }

    #[test]
    fn ora_zero_page_should_perform_bitwise_or_correctly() {
        let mut cpu = Cpu::new().reset(0x0001.into());

        cpu.memory.data[0x0001] = LDA_IM;
        cpu.memory.data[0x0002] = 0x00;
        cpu.memory.data[0x0003] = ORA_ZP;
        cpu.memory.data[0x0004] = 0xF0;
        cpu.memory.data[0x00F0] = 0xFF;
        cpu.memory.data[0x0005] = NOP;

        cpu.execute();

        assert_eq!(cpu.a, 0xFF);
    }

    #[test]
    fn ora_zero_page_x_indexed_should_perform_bitwise_or_correctly() {
        let mut cpu = Cpu::new().reset(0x0001.into());
        cpu.x = 0x01;

        cpu.memory.data[0x00F1] = 0xFF;
        cpu.memory.data[0x0001] = LDA_IM;
        cpu.memory.data[0x0002] = 0x00;
        cpu.memory.data[0x0003] = ORA_ZP_X;
        cpu.memory.data[0x0004] = 0xF0;
        cpu.memory.data[0x0005] = NOP;

        cpu.execute();
        let address = cpu.memory.data[0xF1];
        assert_eq!(cpu.a, 0xFF);
    }

    #[test]
    fn ora_zero_page_indirect_y_indexed_should_perform_bitwise_or_correctly() {
        let mut cpu = Cpu::new().reset(0x0001.into());
        cpu.a = 0x00;
        cpu.y = 0x01;

        cpu.memory.data[0x0011] = 0x00;
        cpu.memory.data[0x0012] = 0xFF;
        cpu.memory.data[0xFF01] = 0xFF;

        cpu.memory.data[0x0001] = ORA_ZP_IY;
        cpu.memory.data[0x0002] = 0x11;
        cpu.memory.data[0x0003] = NOP;

        cpu.execute();
        assert_eq!(cpu.a, 0xFF);
    }

    #[test]
    fn ora_zero_page_x_indexed_indirect_should_perform_bitwise_or_correctly() {
        let mut cpu = Cpu::new().reset(0x0001.into());
        cpu.a = 0x00;
        cpu.x = 0x01;

        cpu.memory.data[0x0012] = 0x00;
        cpu.memory.data[0x0013] = 0xFF;
        cpu.memory.data[0xFF00] = 0xFF;

        cpu.memory.data[0x0001] = ORA_ZP_XI;
        cpu.memory.data[0x0002] = 0x11;
        cpu.memory.data[0x0003] = NOP;

        cpu.execute();
        assert_eq!(cpu.a, 0xFF);
    }

    #[test]
    fn pop_accumulator_should_push_a_register_onto_stack() {
        let mut cpu = Cpu::new().reset(0x0001.into());

        cpu.memory.data[0x0001] = LDA_IM;
        cpu.memory.data[0x0002] = 0xFF;
        cpu.memory.data[0x0003] = PHA;
        cpu.memory.data[0x0004] = LDA_IM;
        cpu.memory.data[0x0005] = 0x00;
        cpu.memory.data[0x0006] = PLA;
        cpu.memory.data[0x0007] = NOP;

        cpu.execute();

        assert_eq!(cpu.a, 0xFF);
    }

    #[test]
    fn pop_processor_status_should_push_ps_register_onto_stack() {
        let mut cpu = Cpu::new().reset(0x0001.into());
        cpu.memory.write_byte(cpu.sp as usize, 0b1101_1111);
        cpu.sp -= 1;
        cpu.memory.data[0x0001] = PLP;
        cpu.memory.data[0x0002] = NOP;

        cpu.execute();

        assert_eq!(cpu.ps.bits(), ProcessorStatus::all().bits());
    }

    #[test]
    fn push_accumulator_should_push_a_register_onto_stack() {
        let mut cpu = Cpu::new().reset(0x0001.into());

        cpu.memory.data[0x0001] = LDA_IM;
        cpu.memory.data[0x0002] = 0xFF;
        cpu.memory.data[0x0003] = PHA;
        cpu.memory.data[0x0004] = NOP;

        cpu.execute();
        let accumulator = cpu.memory.read_byte((cpu.sp + 1) as usize);

        assert_eq!(accumulator, 0xFF);
    }

    #[test]
    fn push_processor_status_should_push_ps_register_onto_stack() {
        let mut cpu = Cpu::new().reset(0x0001.into());
        cpu.ps = ProcessorStatus::all();
        cpu.memory.data[0x0001] = PHP;
        cpu.memory.data[0x0002] = NOP;

        cpu.execute();
        let ps = cpu.memory.read_byte((cpu.sp + 1) as usize);

        assert_eq!(ps, ProcessorStatus::all().bits());
    }

    #[test]
    fn logical_shift_right_absolute_x_indexed_should_shift_value_correctly() {
        let mut cpu = Cpu::new().reset(0x0001.into());

        cpu.memory.data[0x0011] = 0x02;
        cpu.memory.data[0x0001] = LDX_IM;
        cpu.memory.data[0x0002] = 0x01;
        cpu.memory.data[0x0003] = LSR_ABS_X;
        cpu.memory.data[0x0004] = 0x10;
        cpu.memory.data[0x0005] = 0x00; // 0x0010
        cpu.memory.data[0x0006] = NOP;

        cpu.execute();
        let address = cpu.memory.read_byte(0x011); //0x10 + 1
        assert_eq!(address, 0x01);
    }

    #[test]
    fn logical_shift_right_zero_page_x_indexed_should_shift_value_correctly() {
        let mut cpu = Cpu::new().reset(0x0001.into());

        cpu.memory.data[0x0011] = 0x02;
        cpu.memory.data[0x0001] = LDX_IM;
        cpu.memory.data[0x0002] = 0x01;
        cpu.memory.data[0x0003] = LSR_ZP_X;
        cpu.memory.data[0x0004] = 0x10;
        cpu.memory.data[0x0005] = NOP;

        cpu.execute();
        let address = cpu.memory.read_byte(0x011); //0x10 + 1
        assert_eq!(address, 0x01);
    }

    #[test]
    fn logical_shift_right_zero_page_should_shift_value_correctly() {
        let mut cpu = Cpu::new().reset(0x0001.into());

        cpu.memory.data[0x0010] = 0x02;
        cpu.memory.data[0x0001] = LSR_ZP;
        cpu.memory.data[0x0002] = 0x10;
        cpu.memory.data[0x0003] = NOP;

        cpu.execute();
        let address = cpu.memory.read_byte(0x010);
        assert_eq!(address, 0x01);
    }

    #[test]
    fn logical_shift_right_zero_page_should_set_correct_flags() {
        let mut cpu = Cpu::new().reset(0x0001.into());

        cpu.memory.data[0x0010] = 0x01;
        cpu.memory.data[0x0001] = LSR_ZP;
        cpu.memory.data[0x0002] = 0x10;
        cpu.memory.data[0x0003] = NOP;

        cpu.execute();
        assert_eq!(cpu.ps, ProcessorStatus::Z | ProcessorStatus::C);
    }

    #[test]
    fn logical_shift_right_absolute_should_shift_value_at_address_correctly() {
        let mut cpu = Cpu::new().reset(0x0001.into());

        cpu.memory.data[0x0100] = 0x02;
        cpu.memory.data[0x0001] = LSR_ABS;
        cpu.memory.data[0x0002] = 0x00;
        cpu.memory.data[0x0003] = 0x01; // 0x0100
        cpu.memory.data[0x0004] = NOP;

        cpu.execute();
        let address = cpu.memory.read_byte(0x0100);
        assert_eq!(address, 0x01);
    }

    #[test]
    fn logical_shift_right_accumulator_should_shift_value_correctly() {
        let mut cpu = Cpu::new().reset(0x0001.into());
        cpu.memory.data[0x0001] = LDA_IM;
        cpu.memory.data[0x0002] = 0x02;
        cpu.memory.data[0x0003] = LSR_ACC;
        cpu.memory.data[0x0004] = NOP;

        cpu.execute();
        assert_eq!(cpu.a, 0x01);
    }

    #[test]
    fn logical_shift_right_should_set_carry_flag() {
        let mut cpu = Cpu::new().reset(0x0001.into());
        cpu.memory.data[0x0001] = LDA_IM;
        cpu.memory.data[0x0002] = 0x02;
        cpu.memory.data[0x0003] = LSR_ACC;
        cpu.memory.data[0x0004] = NOP;

        cpu.execute();
        assert_eq!(format!("{}", cpu.ps), "00000000");
    }

    #[test]
    fn logical_shift_right_should_reset_negative_flag() {
        let mut cpu = Cpu::new().reset(0x0001.into());
        cpu.memory.data[0x0001] = LDA_IM;
        cpu.memory.data[0x0002] = 0b1000;
        cpu.memory.data[0x0003] = LSR_ACC;
        cpu.memory.data[0x0004] = NOP;

        cpu.execute();
        assert_eq!(format!("{}", cpu.ps), "00000000");
    }

    #[test]
    fn logical_shift_right_should_set_carry_and_zero_flags() {
        let mut cpu = Cpu::new().reset(0x0001.into());
        cpu.memory.data[0x0001] = LDA_IM;
        cpu.memory.data[0x0002] = 0b0001;
        cpu.memory.data[0x0003] = LSR_ACC;
        cpu.memory.data[0x0004] = NOP;

        cpu.execute();
        assert_eq!(format!("{}", cpu.ps), "00000011");
    }

    #[test]
    fn jump_subroutine_should_jump_to_new_address() {
        let mut cpu = Cpu::new().reset(None);

        // load a dummy program into memory
        cpu.memory.data[0xFFFC] = JSR;
        cpu.memory.data[0xFFFD] = 0x10;
        cpu.memory.data[0xFFFE] = 0x00; // JSR 0x0010
        cpu.memory.data[0x0010] = NOP;

        cpu.execute();
        // stack pointer should be 0xFF 0xFD (high byte first)
        let expected_return_address = (cpu.sp + 2) as usize;
        let stack_address = cpu.memory.read_word(expected_return_address);
        // should get to no-op
        assert_eq!(cpu.pc, 0x0011);
        // return to last byte of last instruction
        assert_eq!(stack_address, 0xFFFE);
    }

    #[test]
    fn return_subroutine_should_grab_instructions_from_where_pc_was_left_on_stack() {
        let mut cpu = Cpu::new().reset(0x0001.into());

        cpu.memory.data[0x0001] = JSR;
        cpu.memory.data[0x0002] = 0x00;
        cpu.memory.data[0x0003] = 0x10; // 0x0100
        cpu.memory.data[0x0004] = NOP;
        cpu.memory.data[0x1000] = LDA_IM;
        cpu.memory.data[0x1001] = 0x01;
        cpu.memory.data[0x1002] = RTS;

        cpu.execute();

        assert_eq!(cpu.pc, 0x05);
    }

    #[test]
    fn ldy_immediate_should_load_y_register() {
        let mut cpu = Cpu::new().reset(None);
        // Load a dummy program into memory
        cpu.memory.data[0xFFFC] = LDY_IM;
        cpu.memory.data[0xFFFD] = 0x42;
        cpu.memory.data[0xFFFE] = NOP;

        cpu.execute();
        assert_eq!(cpu.y, 0x42);
    }

    #[test]
    fn ldy_absolute_should_load_y_register() {
        let mut cpu = Cpu::new().reset(None);
        // would overflow if ran from reset vector
        // set PC to lower address
        cpu.pc = 0xFFF0;
        // Load a dummy program into memory
        cpu.memory.data[0xFFF0] = LDY_ABS;
        cpu.memory.data[0xFFF1] = 0x80;
        cpu.memory.data[0xFFF2] = 0x44; // 0x4480
        cpu.memory.data[0x4480] = 0x37;
        cpu.memory.data[0xFFF3] = NOP;

        cpu.execute();
        assert_eq!(cpu.y, 0x37);
    }

    #[test]
    fn ldy_absolute_x_indexed_should_load_y_register_with_correct_value() {
        let mut cpu = Cpu::new().reset(None);
        // would overflow if ran from reset vector
        // set PC to lower address
        cpu.pc = 0xFFF0;
        // set y register
        cpu.x = 0x01;
        // Load a dummy program into memory
        cpu.memory.data[0xFFF0] = LDY_ABS_X;
        cpu.memory.data[0xFFF1] = 0x80;
        cpu.memory.data[0xFFF2] = 0x44; // 0x4480
        cpu.memory.data[0x4481] = 0x37;
        cpu.memory.data[0xFFF3] = NOP;

        cpu.execute();
        assert_eq!(cpu.y, 0x37);
    }

    #[test]
    fn ldy_zero_page_should_load_y_register() {
        let mut cpu = Cpu::new().reset(None);
        // Load a dummy program into memory
        cpu.memory.data[0xFFFC] = LDY_ZP;
        cpu.memory.data[0xFFFD] = 0x42;
        cpu.memory.data[0x0042] = 0x84;
        cpu.memory.data[0xFFFE] = NOP;

        cpu.execute();
        assert_eq!(cpu.y, 0x84);
    }

    #[test]
    fn ldy_zero_page_x_indexed_should_load_y_register() {
        let mut cpu = Cpu::new().reset(None);
        // set the X register to 1
        cpu.x = 0x01;
        // Load a dummy program into memory
        cpu.memory.data[0xFFFC] = LDY_ZP_X;
        cpu.memory.data[0xFFFD] = 0x42;
        cpu.memory.data[0x0042] = 0x84;
        cpu.memory.data[0xFFFE] = NOP;

        cpu.execute();
        assert_eq!(cpu.y, 0x85);
    }

    #[test]
    fn ldx_immediate_should_load_x_register() {
        let mut cpu = Cpu::new().reset(None);
        // Load a dummy program into memory
        cpu.memory.data[0xFFFC] = LDX_IM;
        cpu.memory.data[0xFFFD] = 0x42;
        cpu.memory.data[0xFFFE] = NOP;

        cpu.execute();
        assert_eq!(cpu.x, 0x42);
    }

    #[test]
    fn ldx_absolute_should_load_x_register() {
        let mut cpu = Cpu::new().reset(None);
        // would overflow if ran from reset vector
        // set PC to lower address
        cpu.pc = 0xFFF0;
        // Load a dummy program into memory
        cpu.memory.data[0xFFF0] = LDX_ABS;
        cpu.memory.data[0xFFF1] = 0x80;
        cpu.memory.data[0xFFF2] = 0x44; // 0x4480
        cpu.memory.data[0x4480] = 0x37;
        cpu.memory.data[0xFFF3] = NOP;

        cpu.execute();
        assert_eq!(cpu.x, 0x37);
    }

    #[test]
    fn ldx_absolute_y_indexed_should_load_x_register_with_correct_value() {
        let mut cpu = Cpu::new().reset(None);
        // would overflow if ran from reset vector
        // set PC to lower address
        cpu.pc = 0xFFF0;
        // set y register
        cpu.y = 0x01;
        // Load a dummy program into memory
        cpu.memory.data[0xFFF0] = LDX_ABS_Y;
        cpu.memory.data[0xFFF1] = 0x80;
        cpu.memory.data[0xFFF2] = 0x44; // 0x4480
        cpu.memory.data[0x4481] = 0x37;
        cpu.memory.data[0xFFF3] = NOP;

        cpu.execute();
        assert_eq!(cpu.x, 0x37);
    }

    #[test]
    fn ldx_zero_page_should_load_x_register() {
        let mut cpu = Cpu::new().reset(None);
        // Load a dummy program into memory
        cpu.memory.data[0xFFFC] = LDX_ZP;
        cpu.memory.data[0xFFFD] = 0x42;
        cpu.memory.data[0x0042] = 0x84;
        cpu.memory.data[0xFFFE] = NOP;

        cpu.execute();
        assert_eq!(cpu.x, 0x84);
    }

    #[test]
    fn ldx_zero_page_y_indexed_should_load_x_register() {
        let mut cpu = Cpu::new().reset(None);
        // set the X register to 1
        cpu.y = 0x01;
        // Load a dummy program into memory
        cpu.memory.data[0xFFFC] = LDX_ZP_Y;
        cpu.memory.data[0xFFFD] = 0x42;
        cpu.memory.data[0x0042] = 0x84;
        cpu.memory.data[0xFFFE] = NOP;

        cpu.execute();
        assert_eq!(cpu.x, 0x85);
    }

    #[test]
    fn lda_immediate_should_load_accumulator_register() {
        let mut cpu = Cpu::new().reset(None);
        // Load a dummy program into memory
        cpu.memory.data[0xFFFC] = LDA_IM;
        cpu.memory.data[0xFFFD] = 0x42;
        cpu.memory.data[0xFFFE] = NOP;

        cpu.execute();
        assert_eq!(cpu.a, 0x42);
    }

    #[test]
    fn lda_absolute_should_load_accumulator_register() {
        let mut cpu = Cpu::new().reset(None);
        // would overflow if ran from reset vector
        // set PC to lower address
        cpu.pc = 0xFFF0;
        // Load a dummy program into memory
        cpu.memory.data[0xFFF0] = LDA_ABS;
        cpu.memory.data[0xFFF1] = 0x80;
        cpu.memory.data[0xFFF2] = 0x44; // 0x4480
        cpu.memory.data[0x4480] = 0x37;
        cpu.memory.data[0xFFF3] = NOP;

        cpu.execute();
        assert_eq!(cpu.a, 0x37);
    }

    #[test]
    fn lda_absolute_x_indexed_should_load_accumulator_with_correct_value() {
        let mut cpu = Cpu::new().reset(None);
        // would overflow if ran from reset vector
        // set PC to lower address
        cpu.pc = 0xFFF0;
        // set x register
        cpu.x = 0x01;
        // Load a dummy program into memory
        cpu.memory.data[0xFFF0] = LDA_ABS_X;
        cpu.memory.data[0xFFF1] = 0x80;
        cpu.memory.data[0xFFF2] = 0x44; // 0x4480
        cpu.memory.data[0x4481] = 0x37;
        cpu.memory.data[0xFFF3] = NOP;

        cpu.execute();
        assert_eq!(cpu.a, 0x37);
    }

    #[test]
    fn lda_absolute_y_indexed_should_load_accumulator_with_correct_value() {
        let mut cpu = Cpu::new().reset(None);
        // would overflow if ran from reset vector
        // set PC to lower address
        cpu.pc = 0xFFF0;
        // set y register
        cpu.y = 0x01;
        // Load a dummy program into memory
        cpu.memory.data[0xFFF0] = LDA_ABS_Y;
        cpu.memory.data[0xFFF1] = 0x80;
        cpu.memory.data[0xFFF2] = 0x44; // 0x4480
        cpu.memory.data[0x4481] = 0x37;
        cpu.memory.data[0xFFF3] = NOP;

        cpu.execute();
        assert_eq!(cpu.a, 0x37);
    }

    #[test]
    fn lda_zero_should_set_zero_flag() {
        let mut cpu = Cpu::new().reset(None);
        // Load a dummy program into memory
        cpu.memory.data[0xFFFC] = LDA_IM;
        cpu.memory.data[0xFFFD] = 0x00;
        cpu.memory.data[0xFFFE] = NOP;

        cpu.execute();
        assert_eq!(format!("{}", cpu.ps), "00000010");
    }

    #[test]
    fn lda_seventh_bit_set_should_raise_negative_flag() {
        let mut cpu = Cpu::new().reset(None);
        // Load a dummy program into memory
        cpu.memory.data[0xFFFC] = LDA_IM;
        cpu.memory.data[0xFFFD] = 0b10000001;
        cpu.memory.data[0xFFFE] = NOP;

        cpu.execute();
        assert_eq!(format!("{}", cpu.ps), "10000000");
    }

    #[test]
    fn lda_zero_page_should_load_accumulator_register() {
        let mut cpu = Cpu::new().reset(None);
        // Load a dummy program into memory
        cpu.memory.data[0xFFFC] = LDA_ZP;
        cpu.memory.data[0xFFFD] = 0x42;
        cpu.memory.data[0x0042] = 0x84;
        cpu.memory.data[0xFFFE] = NOP;

        cpu.execute();
        assert_eq!(cpu.a, 0x84);
    }

    #[test]
    fn lda_zero_page_x_indexed_should_load_accumulator_register() {
        let mut cpu = Cpu::new().reset(None);
        // set the X register to 1
        cpu.x = 0x01;
        // Load a dummy program into memory
        cpu.memory.data[0xFFFC] = LDA_ZP_X;
        cpu.memory.data[0xFFFD] = 0x42;
        cpu.memory.data[0x0042] = 0x84;
        cpu.memory.data[0xFFFE] = NOP;

        cpu.execute();
        assert_eq!(cpu.a, 0x85);
    }

    #[test]
    fn lda_zero_page_x_indexed_indirect_should_load_accumulator_register() {
        let mut cpu = Cpu::new().reset(None);
        // set the X register to 1
        cpu.x = 0x04;
        // Load a dummy program into memory
        cpu.memory.data[0xFFFC] = LDA_ZP_XI;
        cpu.memory.data[0xFFFD] = 0x20;
        cpu.memory.data[0x0024] = 0x20;
        cpu.memory.data[0xFFFE] = NOP;

        cpu.execute();
        assert_eq!(cpu.a, 0x20);
    }

    #[test]
    fn lda_zero_page_indirect_y_indexed_should_load_accumulator_register() {
        let mut cpu = Cpu::new().reset(None);
        // set the Y register to 10
        cpu.y = 0x04;
        // Load a dummy program into memory
        cpu.memory.data[0xFFFC] = LDA_ZP_IY;
        cpu.memory.data[0xFFFD] = 0x02;
        cpu.memory.data[0x0002] = 0x00;
        cpu.memory.data[0x0003] = 0x80;
        cpu.memory.data[0x8004] = 0x37;
        cpu.memory.data[0xFFFE] = NOP;

        cpu.execute();
        assert_eq!(cpu.a, 0x37);
    }

    #[test]
    fn read_word() {
        let mut cpu = Cpu::new().reset(None);
        cpu.memory.data[0x44] = 0x20;
        cpu.memory.data[0x45] = 0x20;

        let word = cpu.memory.read_word(0x44);

        assert_eq!(word, 0x2020);
    }
}
