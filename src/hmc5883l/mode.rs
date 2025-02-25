#[derive(PartialEq, Clone, Copy)]
pub enum Mode {
    Single = 0b0000_0001,
    Continuous = 0b0000_0000,
    Idle = 0b0000_0010
}