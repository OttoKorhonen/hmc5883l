use crate::config::{data_output_rate::DataOutputRate, MeasurementMode, measurement_sample_output::MeasurementSampleRate};
// Maskit rekisterille A
const MASK_MEASUREMENT_MODE: u8 = 0b0000_0011; // Bitit 0-1
const MASK_DATA_OUTPUT_RATE: u8 = 0b0001_1100; // Bitit 2-4
const MASK_SAMPLES_AVERAGED: u8 = 0b0110_0000; // Bitit 5-6

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RegisterA {
    bits: u8
}
impl RegisterA {
    // Oletusarvo: 0x10 (Data Output Rate 15Hz, Normal Mode, 1 Sample)
    // 0x10 = 0001 0000 -> DOR=100 (15Hz), Samples=00 (1), Mode=00 (Normal)
    pub const fn new() -> Self {
        RegisterA { bits: 0x10 } 
    }
    pub const  fn get_value(&self) -> u8 {
        self.bits
    }
    /// Sets Measurement Mode (bits 0-1)
    pub const fn set_measurement_mode(&mut self, mode: MeasurementMode) {
        self.bits = (self.bits & !MASK_MEASUREMENT_MODE) | (mode.get_value() & MASK_MEASUREMENT_MODE);
    }
    /// Sets Data Output Rate (bits 2-4)
    pub const fn set_data_output_rate(&mut self, rate: DataOutputRate) {
        self.bits = (self.bits & !MASK_DATA_OUTPUT_RATE) | (rate.get_value() & MASK_DATA_OUTPUT_RATE);
    }
    /// Select number of samples averaged (1 to 8) per measurement output (bits 5-6).
    pub const fn set_number_of_samples(&mut self, output: MeasurementSampleRate) {
        self.bits = (self.bits & !MASK_SAMPLES_AVERAGED) | (output.get_value() & MASK_SAMPLES_AVERAGED);
    }
}
