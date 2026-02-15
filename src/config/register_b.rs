use crate::config::gain_config::Gain;

const MASK_GAIN_CONFIGURATION: u8 = 0b1110_0000;

pub struct RegisterB {
    bits: u8
}

impl RegisterB {

    pub const fn new() -> Self {Self {bits: 0}}
    pub const fn get_value(&self) -> u8 { self.bits}

    pub const fn set_gain(&mut self, gain: Gain)  {
        self.bits = (self.bits & !MASK_GAIN_CONFIGURATION) | (gain.get_value() & MASK_GAIN_CONFIGURATION);
    }
}