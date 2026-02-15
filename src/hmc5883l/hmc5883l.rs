use crate::address::Registers::{
    ConfigurationRegistersA, ConfigurationRegistersB, DataOutputXMsB, ModeRegister,
};
use crate::config::{Gain, OperatingMode, RegisterA, RegisterB};
use crate::errors::hmc5883lerror::Hmc5883lError;
use crate::hmc5883l::CompassPoint;
use core::f32::consts::PI;
use embedded_hal::i2c::SevenBitAddress;
use libm::atan2f;

pub struct Hmc5883l<I2c> {
    i2c: I2c,
    device_address: SevenBitAddress,
}

impl<I2c> Hmc5883l<I2c>
where
    I2c: embedded_hal::i2c::I2c,
{
    pub const fn new(i2c: I2c, device_address: SevenBitAddress) -> Self {
        Self {
            i2c,
            device_address,
        }
    }

    pub fn set_register_a(&mut self, register: RegisterA) -> Result<(), Hmc5883lError<I2c::Error>> {
        self.i2c.write(
            self.device_address,
            &[
                ConfigurationRegistersA.get_register_value(),
                register.get_value(),
            ],
        )?;
        Ok(())
    }

    pub fn set_register_b(&mut self, register: RegisterB) -> Result<(), Hmc5883lError<I2c::Error>> {
        self.i2c.write(
            self.device_address,
            &[
                ConfigurationRegistersB.get_register_value(),
                register.get_value(),
            ],
        )?;
        Ok(())
    }

    pub fn set_mode_register(
        &mut self,
        mode_register: ModeRegister,
    ) -> Result<(), Hmc5883lError<I2c::Error>> {
        self.i2c.write(
            self.device_address,
            &[ModeRegister.get_register_value(), mode_register.get_value()],
        )?;
        Ok(())
    }

    pub fn set_gain(&mut self, gain: Gain) -> Result<(), Hmc5883lError<I2c::Error>> {
        self.i2c.write(
            self.device_address,
            &[
                ConfigurationRegistersB.get_register_value(),
                gain.get_value(),
            ],
        )?;
        Ok(())
    }

    pub fn set_operating_mode(
        &mut self,
        operating_mode: OperatingMode,
    ) -> Result<(), Hmc5883lError<I2c::Error>> {
        // Sama huomio: Operating Mode on osa Mode Registeriä (0x02).
        self.i2c.write(
            self.device_address,
            &[
                ModeRegister.get_register_value(),
                operating_mode.get_value(),
            ],
        )?;
        Ok(())
    }

    fn read_data_output(&mut self) -> Result<[u8; 6], Hmc5883lError<I2c::Error>> {
        let mut buffer = [0u8; 6];
        //DataOutputXMsb jne
        self.i2c.write_read(
            self.device_address,
            &[DataOutputXMsB.get_register_value()],
            &mut buffer,
        )?;

        Ok(buffer)
    }

    pub fn get_angle(&mut self) -> Result<f32, Hmc5883lError<I2c::Error>> {
        let data_register_output = self.read_data_output()?;
        let high_byte_x = data_register_output[0] as i16;
        let low_byte_x = data_register_output[1] as i16;

        let high_byte_y = data_register_output[4] as i16;
        let low_byte_y = data_register_output[5] as i16;

        let output_y = (high_byte_y << 8) | low_byte_y;
        let output_x = (high_byte_x << 8) | low_byte_x;

        let gauss = self.measure_gauss(output_x, output_y);
        let angle = self.atan2_custom(gauss);
        Ok(angle)
    }

    fn atan2_custom(&self, gauss: (f32, f32)) -> f32 {
        let mut angle = atan2f(gauss.1, gauss.0) * (180.0 / PI);

        if angle < 0.0 {
            angle += 360.0;
        }

        angle
    }

    fn measure_gauss(&self, x: i16, y: i16) -> (f32, f32) {
        let gauss_x = (x as f32) / 230.0;
        let gauss_y = (y as f32) / 230.0;
        (gauss_x, gauss_y)
    }

    pub fn get_compass_point(&mut self, angle: f32) -> CompassPoint {
        let angle = if angle < 0.0 { angle + 360.0 } else { angle };
        match angle {
            x if x >= 337.5 || x < 22.5 => CompassPoint::North,
            x if x >= 22.5 && x < 67.5 => CompassPoint::NorthEast,
            x if x >= 67.5 && x < 112.5 => CompassPoint::East,
            x if x >= 112.5 && x < 157.5 => CompassPoint::SouthEast,
            x if x >= 157.5 && x < 202.5 => CompassPoint::South,
            x if x >= 202.5 && x < 247.5 => CompassPoint::SouthWest,
            x if x >= 247.5 && x < 292.5 => CompassPoint::West,
            x if x >= 292.5 && x < 337.5 => CompassPoint::NorthWest,
            _ => CompassPoint::North,
        }
    }
}
