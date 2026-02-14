
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Registers {
    ConfigurationRegistersA = 0x00,
    ConfigurationRegistersB = 0x01,
    ModeRegister = 0x02,
    DataOutputXMsB = 0x03,
    DataOutputXLsB = 0x04,
    DataOutputZMsB = 0x05,
    DataOutputZLsB = 0x06,
    DataOutputYMsB = 0x07,
    DataOutputYLsB = 0x08,
    StatusRegister = 0x09,
    IdentificationRegisterA = 0x10,
    IdentificationRegisterB = 0x11,
    IdentificationRegisterC = 0x12,
}

impl Registers {
    pub fn get_register_value(&self) -> Self {*self}
}