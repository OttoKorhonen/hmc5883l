use crate::hmc5883l::{CompassPoint, Gain, Mode};
use core::marker::PhantomData;
use core::{fmt, i32};
use embedded_hal::i2c::I2c as HalI2c;
use esp_hal::delay::Delay;
use heapless::Vec;
use libm::atan2f;

const PI: f32 = 3.14159265358979323846264338327950288_f32;

pub struct Hmc5883l<I2c, E> {
    i2c: I2c,
    device_address: u8,
    _error: PhantomData<E>,
}

impl<I2c, E> Hmc5883l<I2c, E>
where
    I2c: HalI2c<Error = E>,
    E: fmt::Debug,
{
    //read 0x3D
    //write 0x3C

    pub fn new(i2c: I2c, device_address: u8) -> Self {
        Self {
            i2c,
            device_address,
            _error: PhantomData,
        }
    }

    fn get_x_register(&mut self) -> [u8; 2] {
        let cmd = [0x03];  // X-akselin MSB
        let mut response = [0u8; 6]; // Lue kaikki akselidatat samalla
        self.i2c.write_read(self.device_address, &cmd, &mut response).unwrap();
    
        [response[0], response[1]] // Palauta vain X:n MSB ja LSB
    }

    fn get_y_register(&mut self) -> [u8; 2] {
        let cmd = [0x07];  // X-akselin MSB
        let mut response = [0u8; 6]; // Lue kaikki akselidatat samalla
        self.i2c.write_read(self.device_address, &cmd, &mut response).unwrap();
    
        [response[0], response[1]] // Palauta vain X:n MSB ja LSB
    }

    ///take single measure
    pub fn single_measurement(&mut self, gain: Gain) -> f32 {
        let delay = Delay::new();
        self.cra();
        self.crb(gain);
        self.mode_register(Mode::Single);
        delay.delay_millis(6);

        let x = self.get_x_register();
        let y = self.get_y_register();

        let response = [x[0], x[1], y[0], y[1]];

        let gauss_values = self.measure_gauss(&response);

        let angle = self.atan2_custom(gauss_values[1], gauss_values[2]);

        angle
    }


    ///take continuous measurement
    pub fn continuous_measure(&mut self, gain: Gain) -> f32 {
        let delay = Delay::new();
        self.cra();
        self.crb(gain);
        self.mode_register(Mode::Continuous);
        delay.delay_millis(6);

        loop {

            let x = self.get_x_register();
            let y = self.get_y_register();

            let response = [x[0], x[1], y[0], y[1]];
            let gauss_values = self.measure_gauss(&response);

            delay.delay_millis(67);

            return self.atan2_custom(gauss_values[0], gauss_values[1]);
        }
    }

    //East-Counterclockwise Convention y,x
    fn atan2_custom(&self, y: f32, x: f32) -> f32 {
        let mut angle = atan2f(y, x) * (180.0 / PI);

        if angle < 0.0 {
            angle += 360.0;
        }
        // println!("Angle: {:.2}°", angle);
        angle
    }

    fn measure_gauss(&self, raw_data: &[u8; 4]) -> Vec<f32, 2> {
        let mut gauss_values: Vec<f32, 2> = Vec::new();

        for chunk in raw_data.chunks(2) {
            let mut direction = ((chunk[0] as i32) << 8) | (chunk[1] as i32);

            // Käsittele negatiiviset luvut kahden komplementin mukaisesti
            if direction > 0x7FFF {
                direction -= 0x10000 as i32;
            }

            //value converted into gauss
            let gauss = direction as f32 / 230.0; // 1090 on muutettava vastaamaan gainia

            gauss_values.push(gauss).unwrap();
        }
        gauss_values
    }

    fn cra(&mut self) {
        let cmd = [0x00, 0b0011_0000]; // Configuration Register A (0x00)
        self.i2c.write(self.device_address, &cmd).unwrap();
    }

    fn crb(&mut self, gain: Gain) {
        let cmd = [0x01, gain as u8]; // Configuration Register B (0x01)
        self.i2c.write(self.device_address, &cmd).unwrap();
    }

    fn mode_register(&mut self, mode: Mode) {
        let cmd = [0x02, mode as u8]; // Mode Register (0x02)
        self.i2c.write(self.device_address, &cmd).unwrap();
    }
}
