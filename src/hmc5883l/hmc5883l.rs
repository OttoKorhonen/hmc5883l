use crate::hmc5883l::CompassPoint;
use core::fmt;
use embedded_hal::i2c::SevenBitAddress;
use libm::atan2f;
use crate::config::{RegisterA, RegisterB, ModeRegister, Gain, OperatingMode};
use crate::errors::hmc8553lerror::Hmc8553lError;

const PI: f32 = 3.14159265358979323846264338327950288_f32;

pub struct Hmc5883l<I2c, E> {
    i2c: I2c,
    device_address: SevenBitAddress
}

impl<I2c, E> Hmc5883l<I2c, E>
where
    I2c: embedded_hal::i2c::I2c<Error = E>,
    E: fmt::Debug
{
    //read 0x3D
    //write 0x3C

    pub fn new(i2c: I2c, device_address: SevenBitAddress) -> Self {
        Self {
            i2c,
            device_address,
        }
    }
    
    pub fn set_register_a(&mut self, register: RegisterA) -> Result<(), Hmc8553lError<I2c::Error>> {
        self.i2c.write(self.device_address, &[register.get_value()])?;
        Ok(())
    }

    pub fn set_register_b(&mut self, register: RegisterB) -> Result<(), Hmc8553lError<I2c::Error>> {
        self.i2c.write(self.device_address, &[register.get_value()])?;
        Ok(())
    }
    
    pub fn set_mode_register(&mut self, mode_register: ModeRegister) -> Result<(), Hmc8553lError<I2c::Error>> {
        self.i2c.write(self.device_address, &[mode_register.get_value()])?;
        Ok(())
    }
    
    pub fn set_gain(&mut self, gain: Gain) -> Result<(), Hmc8553lError<I2c::Error>> {
        self.i2c.write(self.device_address, &[gain.get_value()])?;
        Ok(())
    }
    
    pub fn set_operating_mode(&mut self, operating_mode: OperatingMode) -> Result<(), Hmc8553lError<I2c::Error>> {
        self.i2c.write(self.device_address, &[operating_mode.get_value()])?;
        Ok(())
    }

    // fn get_x_register(&mut self) -> [u8; 2] {
    //     let cmd = [0x03];
    //     let mut response = [0u8; 6];
    //     self.i2c
    //         .write_read(self.device_address, &cmd, &mut response)
    //         .unwrap();
    // 
    //     [response[0], response[1]] // return msb & lsb
    // }
    // 
    // fn get_y_register(&mut self) -> [u8; 2] {
    //     let cmd = [0x07];
    //     let mut response = [0u8; 6];
    //     self.i2c
    //         .write_read(self.device_address, &cmd, &mut response)
    //         .unwrap();
    // 
    //     [response[0], response[1]] // return msb & lsb
    // }
    // 
    // pub fn get_compasspoint(&mut self, angle: f32) -> CompassPoint {
    //     match angle {
    //         0.0..22.0 => CompassPoint::North,
    //         23.0..66.0 => CompassPoint::NorthEast,
    //         67.0..112.0 => CompassPoint::East,
    //         112.0..156.0 => CompassPoint::SouthEast,
    //         157.0..201.0 => CompassPoint::South,
    //         202.0..247.0 => CompassPoint::SouthWest,
    //         248.0..292.0 => CompassPoint::West,
    //         293.0..337.0 => CompassPoint::NorthWest,
    //         338.0..360.0 => CompassPoint::North,
    //         _ => CompassPoint::North,
    //     }
    // }
    // 
    // ///take single measure
    // pub fn single_measurement(&mut self, gain: Gain) -> f32 {
    //     self.cra();
    //     self.crb(gain);
    //     self.mode_register(Mode::Single);
    //     self.delay.delay_ms(6);
    // 
    //     let x = self.get_x_register();
    //     let y = self.get_y_register();
    // 
    //     let response = [x[0], x[1], y[0], y[1]];
    // 
    //     let gauss_values = self.measure_gauss(&response);
    // 
    //     self.atan2_custom(gauss_values[1], gauss_values[0])
    // }
    // 
    // ///take continuous measurement
    // pub fn continuous_measure(&mut self, gain: Gain) -> f32 {
    //     self.cra();
    //     self.crb(gain);
    //     self.mode_register(Mode::Continuous);
    //     self.delay.delay_ms(6);
    // 
    //     loop {
    //         let x = self.get_x_register();
    //         let y = self.get_y_register();
    // 
    //         let response = [x[0], x[1], y[0], y[1]];
    //         let gauss_values = self.measure_gauss(&response);
    // 
    //         self.delay.delay_ms(67);
    // 
    //         self.atan2_custom(gauss_values[1], gauss_values[0]);
    //     }
    // }
    // 
    // //East-Counterclockwise Convention y,x
    // fn atan2_custom(&self, y: f32, x: f32) -> f32 {
    //     let mut angle = atan2f(y, x) * (180.0 / PI);
    // 
    //     if angle < 0.0 {
    //         angle += 360.0;
    //     }
    //     println!("Angle: {:.2}°", angle);
    //     angle
    // }
    // 
    // fn measure_gauss(&self, raw_data: &[u8; 4]) -> Vec<f32, 2> {
    //     let mut gauss_values: Vec<f32, 2> = Vec::new();
    // 
    //     for chunk in raw_data.chunks(2) {
    //         let mut direction = ((chunk[0] as i32) << 8) | (chunk[1] as i32);
    // 
    //         //handle negative values
    //         if direction > 0x7FFF {
    //             direction -= 0x10000 as i32;
    //         }
    // 
    //         //value converted into gauss
    //         let gauss = direction as f32 / 230.0; // 1090 on muutettava vastaamaan gainia
    // 
    //         gauss_values.push(gauss).unwrap();
    //     }
    //     gauss_values
    // }
    // 
    // fn cra(&mut self) {
    //     let cmd = [0x00, 0b0001_1100]; // Configuration Register A (0x00)
    //     self.i2c.write(self.device_address, &cmd).unwrap();
    // }
    // 
    // fn crb(&mut self, gain: Gain) {
    //     let cmd = [0x01, gain as u8]; // Configuration Register B (0x01)
    //     self.i2c.write(self.device_address, &cmd).unwrap();
    // }
    // 
    // fn mode_register(&mut self, mode: Mode) {
    //     let cmd = [0x02, mode as u8]; // Mode Register (0x02)
    //     self.i2c.write(self.device_address, &cmd).unwrap();
    // }
}
