use core::fmt;

use bitflags::bitflags;

bitflags! {
    #[derive(Default)]
    pub struct ProcessorStatus: u8 {
        // Negative
        const N = 0b10000000;
        // Overflow
        const V = 0b01000000;
        // Break
        const B = 0b00010000;
        // Decimal
        const D = 0b00001000;
        // Interupt Disable
        const I = 0b00000100;
        // Zero
        const Z = 0b00000010;
        // Carry
        const C = 0b00000001;
    }
}

impl ProcessorStatus {
    pub fn clear(&mut self) -> &mut Self {
        self.bits = 0;
        self
    }
}

impl fmt::Display for ProcessorStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:08b}", self.bits)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default() {
        let bits = ProcessorStatus::default();
        assert_eq!(format!("{bits}"), "00000000");
    }

    #[test]
    fn clear() {
        let mut bits = ProcessorStatus::N;
        bits.clear();
        assert_eq!(format!("{bits}"), "00000000");
    }

    #[test]
    fn bitwise_or() {
        let negative_flag = ProcessorStatus::N;
        let no_flags = ProcessorStatus::default();
        assert_eq!(negative_flag | no_flags, ProcessorStatus::N);
    }
}
