#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum OperatingMode {
    ContinuousMeasurementMode = 0b00 << 0,
    SingleMeasurementMode = 0b01 << 0,
    IdleMode = 0b10 << 0,
    IdleMode2 = 0b11 << 0,
}

impl OperatingMode {
    pub const fn get_value(&self) -> u8 {
        *self as u8
    }
}
