#![allow(unused)]
/// load accumulator immediate
pub const LDA_IM: u8 = 0xA9;
/// load accumulator absolute
pub const LDA_ABS: u8 = 0xAD;
/// load accumulator absolute x indexed
pub const LDA_ABS_X: u8 = 0xBD;
/// load accumulator absolute y indexed
pub const LDA_ABS_Y: u8 = 0xB9;
/// load accumulator zero page
pub const LDA_ZP: u8 = 0xA5;
/// load accumulator zero page x indexed
pub const LDA_ZP_X: u8 = 0xB5;
/// load accumulator zero page x indexed indirect
pub const LDA_ZP_XI: u8 = 0xA1;
/// load accumulator zero page y indirect indexed
pub const LDA_ZP_IY: u8 = 0xB1;

/// load x index immediate
pub const LDX_IM: u8 = 0xA2;
/// load x index absolute
pub const LDX_ABS: u8 = 0xA3;
/// load x index y indexed absolute
pub const LDX_ABS_Y: u8 = 0xBE;
/// load x index zero page
pub const LDX_ZP: u8 = 0xA6;
/// load x index y indexed zero page
pub const LDX_ZP_Y: u8 = 0xB6;

/// load y immediate
pub const LDY_IM: u8 = 0xA0;
/// load y index absolute
pub const LDY_ABS: u8 = 0xAC;
/// load y index x indexed absolute
pub const LDY_ABS_X: u8 = 0xBC;
/// load y index zero page
pub const LDY_ZP: u8 = 0xA4;
/// load y index x indexed zero page
pub const LDY_ZP_X: u8 = 0xB4;

/// no-op
pub const NOP: u8 = 0xEA;
/// jump subroutine
pub const JSR: u8 = 0x20;
/// jump absolute
pub const JMP_ABS: u8 = 0x4C;
/// jump absolute indirect
pub const JMP_ABS_IND: u8 = 0x6C;
/// return from subroutine
pub const RTS: u8 = 0x60;

/// logical shift right accumulator
pub const LSR_ACC: u8 = 0x4A;
/// logical shift right absolute
pub const LSR_ABS: u8 = 0x4E;
/// logical shift right zero page
pub const LSR_ZP: u8 = 0x46;
/// logical shift right absolute x indexed
pub const LSR_ABS_X: u8 = 0x5E;
/// logical shift right zero page x indexed
pub const LSR_ZP_X: u8 = 0x56;

/// push accumulator on the stack
pub const PHA: u8 = 0x48;
/// push processor status on the stack
pub const PHP: u8 = 0x08;
/// pop accumulator on the stack
pub const PLA: u8 = 0x68;
/// pop processor status on the stack
pub const PLP: u8 = 0x28;

/// or accumulator immediate
pub const ANDA_IM: u8 = 0x29;
/// or accumulator absolute
pub const ANDA_ABS: u8 = 0x2D;
/// or accumulator x indexed absolute
pub const ANDA_X_ABS: u8 = 0x3D;
/// or accumulator y indexed absolute
pub const ANDA_Y_ABS: u8 = 0x39;
/// or accumulator zero page
pub const ANDA_ZP: u8 = 0x25;
/// or accumulator x indexed zero page
pub const ANDA_ZP_X: u8 = 0x35;
/// or accumulator x indexed zero page indirect
pub const ANDA_ZP_XI: u8 = 0x21;
/// or accumulator zero page indirect y indexed
pub const ANDA_ZP_IY: u8 = 0x31;

/// and accumulator immediate
pub const ORA_IM: u8 = 0x09;
/// and accumulator absolute
pub const ORA_ABS: u8 = 0x0D;
/// and accumulator x indexed absolute
pub const ORA_X_ABS: u8 = 0x1D;
/// and accumulator y indexed absolute
pub const ORA_Y_ABS: u8 = 0x19;
/// and accumulator zero page
pub const ORA_ZP: u8 = 0x05;
/// and accumulator x indexed zero page
pub const ORA_ZP_X: u8 = 0x15;
/// and accumulator x indexed zero page indirect
pub const ORA_ZP_XI: u8 = 0x01;
/// and accumulator zero page indirect y indexed
pub const ORA_ZP_IY: u8 = 0x11;

/// transfer accumulator to index x
pub const TAX: u8 = 0xAA;
/// transfer accumulator to index y
pub const TAY: u8 = 0xA8;
/// transfer stack pointer to index x
pub const TSX: u8 = 0xBA;
/// transfer stack pointer to index x
pub const TXA: u8 = 0x8A;
/// transfer stack pointer to index x
pub const TXS: u8 = 0x9A;
/// transfer stack pointer to index x
pub const TYA: u8 = 0x98;

/// set carry flag
pub const SEC: u8 = 0x38;
/// set decimal mode
pub const SED: u8 = 0xF8;
/// set interrupt disable
pub const SEI: u8 = 0x78;
