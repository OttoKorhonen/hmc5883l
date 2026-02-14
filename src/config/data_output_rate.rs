
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataOutputRate {
    Hz075 = 0b000 << 2,
    Hz1Dot5 = 0b001 << 2,
    Hz3 = 0b010 << 2,
    Hz7Dot5 = 0b011 << 2,
    Hz15 = 0b100 << 2,
    Hz30 = 0b101 << 2,
    Hz75 = 0b110 << 2,
    NotUsed = 0b111 << 2
}

impl DataOutputRate {
    pub fn get_value(&self) -> u8 {*self as u8}
}