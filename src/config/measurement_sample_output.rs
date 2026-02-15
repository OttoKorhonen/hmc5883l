
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MeasurementSampleRate {
    One = 0b00 << 5,
    Two = 0b01 << 5,
    Four = 0b10 << 5,
    Eight = 0b11 << 5
}

impl MeasurementSampleRate {
    pub const fn get_value(&self) -> u8 {*self as u8}
}