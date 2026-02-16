#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum Gain{
    Gain1370 = 0b000 << 5,
    Gain1090 = 0b001 << 5,
    Gain820 = 0b010 << 5,
    Gain660 = 0b011 << 5,
    Gain440 = 0b100 << 5,
    Gain390 = 0b101 << 5,
    Gain330 = 0b110 << 5,
    Gain230 = 0b111 << 5
}

impl Gain {
    pub const fn get_value(self) -> u8 {self as u8}

    pub fn lsb_per_gauss(self) -> f32 {
        match self {
            Gain::Gain1370 => 1370.0,
            Gain::Gain1090 => 1090.0,
            Gain::Gain820 => 820.0,
            Gain::Gain660 => 660.0,
            Gain::Gain440 => 440.0,
            Gain::Gain390 => 390.0,
            Gain::Gain330 => 330.0,
            Gain::Gain230 => 230.0,
        }
    }
}