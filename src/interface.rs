use display_interface_i2c::I2CInterface;
use display_interface_spi::SPIInterface;
use embedded_hal::{digital::OutputPin, i2c::I2c, spi::SpiDevice};

pub struct I2cDisplayInterface;

impl I2cDisplayInterface {
    pub fn new_interface<I: I2c>(i2c: I) -> I2CInterface<I> {
        Self::new_custom_interface(i2c, 0x78)
    }

    pub fn new_alternative_interface<I: I2c>(i2c: I) -> I2CInterface<I> {
        Self::new_custom_interface(i2c, 0x3C)
    }

    pub fn new_custom_interface<I: I2c>(i2c: I, addr: u8) -> I2CInterface<I> {
        I2CInterface::new(i2c, addr, 0x40)
    }
}

pub struct SpiDisplayInterface;

impl SpiDisplayInterface {
    pub fn new_interface<S: SpiDevice, DC: OutputPin>(spi: S, dc: DC) -> SPIInterface<S, DC> {
        SPIInterface::new(spi, dc)
    }
}
