
use core::fmt::{Debug, Display, Formatter};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum Hmc5883lError<E> {
    I2cError(E),
}

impl<E> Display for Hmc5883lError<E>
where
    E: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::I2cError(E) => write!(f, "I2c error: {}", E),
        }
    }
}
impl<E> core::error::Error for Hmc5883lError<E> where E: Debug {}

impl<E> From<E> for Hmc5883lError<E> {
    fn from(e: E) -> Self {
        Hmc5883lError::I2cError(e)
    }
}
