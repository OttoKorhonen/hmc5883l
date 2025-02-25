use crate::hmc5883l::{CompassPoint, Gain, Mode};
use core::marker::PhantomData;
use core::{fmt, i32};
use embedded_hal::i2c::I2c as HalI2c;
use esp_hal::delay::Delay;
use esp_println::println;
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

    ///take single measure
    pub fn single_measurement(&mut self, gain: Gain) -> [u8; 6] {
        let delay = Delay::new();
        self.cra();
        self.crb(gain);
        self.mode_register(Mode::Single);
        delay.delay_millis(6);

        let cmd = [0x06];
        let mut response = [0u8; 6];

        self.i2c
        .write_read(self.device_address, &cmd, &mut response)
        .unwrap();

        println!("res {:?}", response);

        let gauss_values = self.measure_gauss(&response);

        self.atan2_custom(gauss_values[1], gauss_values[2]);

        self.i2c.write(self.device_address, &[0x3C, 0x03]).unwrap();


        response
    }

    //take continuous measurement
    pub fn continuous_measure(&mut self, gain: Gain) {
        let delay = Delay::new();
        self.cra();
        self.crb(gain);
        self.mode_register(Mode::Continuous);
        delay.delay_millis(6);

        let cmd = [0x06];
        let mut response = [0u8; 6];

        loop {
            self.i2c
                .write_read(self.device_address, &cmd, &mut response)
                .unwrap();

            let gauss_values = self.measure_gauss(&response);

            self.atan2_custom(gauss_values[1], gauss_values[2]);

            self.i2c.write(self.device_address, &[0x3C, 0x03]).unwrap();

            delay.delay_millis(70);
        }
    }

    //East-Counterclockwise Convention y,x
    fn atan2_custom(&self, y: f32, x: f32) -> f32 {
        let mut angle = atan2f(y, x) * (360.0 / PI);

        if angle < 0.0 {
            angle += 360.0;
        }
        //println!("Angle: {:.2}°", angle);
        angle
    }

    fn measure_gauss(&self, raw_data: &[u8; 6]) -> Vec<f32, 3> {
        let mut gauss_values: Vec<f32, 3> = Vec::new();

        for chunk in raw_data.chunks(2) {
            let mut direction = ((chunk[0] as i32) << 8) | (chunk[1] as i32);

            // Käsittele negatiiviset luvut kahden komplementin mukaisesti
            if direction > 0x7FFF {
                direction -= 0x10000 as i32;
            }

            //value converted into gauss
            let gauss = direction as f32 / 1090.0; // 1090 on muutettava vastaamaan gainia

            gauss_values.push(gauss).unwrap();
        }
        gauss_values
    }

fn cra(&mut self) {
    let cmd = [0x00, 0b0111_0000]; // Configuration Register A (0x00)
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
