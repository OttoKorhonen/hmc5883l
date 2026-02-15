
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Gain{
    Gain1370 = 0b000 << 7,
    Gain1090 = 0b001 << 7,
    Gain820 = 0b010 << 7,
    Gain660 = 0b011 << 7,
    Gain440 = 0b100 << 7,
    Gain390 = 0b101 << 7,
    Gain330 = 0b110 << 7,
    Gain230 = 0b111 << 7
}

impl Gain {
    pub const fn get_value(self) -> u8 {self as u8}
}