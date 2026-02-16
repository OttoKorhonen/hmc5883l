#[derive(Debug)]
pub enum Hmc5883lError<E>{
    I2cError(E)
}

impl<E> From<E> for Hmc5883lError<E> {
    fn from(e: E) -> Self {
        Hmc5883lError::I2cError(e)
    }
}
