use display_interface_i2c::I2CInterface;
use embedded_hal::i2c::*;

const DATA_TYPE: u8 = 0x40;

pub struct I2cDisplayInterface;

impl I2cDisplayInterface {
    pub fn new_interface<I: I2c>(i2c: I) -> I2CInterface<I> {
        I2CInterface::new(i2c, 0x00, DATA_TYPE)
    }

    pub fn new_alternative_interface<I: I2c>(i2c: I) -> I2CInterface<I> {
        I2CInterface::new(i2c, 0x00, DATA_TYPE)
    }

    pub fn new_custom_interface<I: I2c>(i2c: I, addr: u8) -> I2CInterface<I> {
        I2CInterface::new(i2c, addr, DATA_TYPE)
    }
}
