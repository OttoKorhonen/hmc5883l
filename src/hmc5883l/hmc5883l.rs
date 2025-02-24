use core::fmt;
use core::marker::PhantomData;
use embedded_hal::i2c::I2c as HalI2c;
use esp_hal::delay::Delay;
use esp_println::println;
use crate::hmc5883l::mode::Mode;

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

    ///measure once
    pub fn single_measurement(&mut self) -> [u8; 6] {
        let delay = Delay::new();
        self.cra();
        self.crb();
        self.mode_register(Mode::Single);
        delay.delay_millis(6);

        let cmd = [0x06];
        let mut response = [0u8; 6];
        self.i2c.write_read(self.device_address, &cmd, &mut response).unwrap();

        println!("response {:?}", response);

        response

    }
    ///set measure in idle state
    pub fn set_measurement_idle(&mut self) {
        todo!()
    }

    ///set measure in continuous mode
    pub fn continuous_measurement(&mut self) {
        // let delay = Delay::new();
        // self.cra();
        // self.crb();
        // self.mode_register();
        // delay.delay_millis(6);

        // let cmd = [0x06];
        // let mut response = [0u8; 6];
        // self.i2c.write_read(self.device_address, &cmd, &mut response).unwrap();

        // println!("response {:?}", response);

        // response
        todo!()
    }

    fn cra(&mut self) {
        //CRA default is 0x10
        let cmd =[0x3C, 0b0111_0000];
        self.i2c.write(self.device_address, &cmd).unwrap();
    }

    fn crb(&mut self) {
        //CRB default is 0x20
        let cmd = [0x3C, 0b001_0000];
        self.i2c.write(self.device_address, &cmd).unwrap();
    }

    fn measure_effective_resolution(&mut self) {
        //Effective Resolution = Max (Digital Resolution, Noise Floor)
        todo!()
    }

    fn mode_register(&mut self, mode: Mode) {
        //Mode register default is 0x01
        let cmd = [0x3C, mode as u8];
        self.i2c.write(self.device_address, &cmd).unwrap();
    }

}
