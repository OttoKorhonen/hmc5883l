use crate::config::operating_mode_register_config::OperatingMode;
const MASK_MODE_REGISTER: u8 = 0b0000_0011;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OperatingModeRegister {
    bits: u8,
}

impl OperatingModeRegister {
    pub const fn new() -> Self {
        Self { bits: 0 }
    }
    pub const fn get_value(&self) -> u8 {
        self.bits
    }
    pub const fn set_mode(&mut self, mode: OperatingMode) {
        self.bits = (self.bits & !MASK_MODE_REGISTER) | (mode.get_value() & MASK_MODE_REGISTER);
    }
}
