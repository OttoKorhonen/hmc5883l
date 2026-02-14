
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MeasurementMode {
    ///Normal measurement configuration (Default)
    NormalMode = 0b00,
    ///Positive bias configuration for X, Y, and Z axes. In this configuration, a positive
    /// current is forced across the resistive load for all three axes.
    PositiveBiasConfig = 0b01,
    ///Negative bias configuration for X, Y and Z axes. In this configuration, a negative
    /// current is forced across the resistive load for all three axes..
    NegativeBiasConfig = 0b10
}

impl MeasurementMode {
    
    pub fn get_value(&self) -> u8 {*self as u8}
}