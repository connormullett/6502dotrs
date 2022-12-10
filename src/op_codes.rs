#![allow(unused)]
// load accumulator immediate
pub const LDA_IM: u8 = 0xA9;
// load accumulator zero page
pub const LDA_ZP: u8 = 0xA5;
// load accumulator zero page x indexed
pub const LDA_ZP_X: u8 = 0xB5;
// no-op
pub const NOP: u8 = 0xEA;
