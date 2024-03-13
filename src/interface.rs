use display_interface_i2c::I2CInterface;
use display_interface_spi::SPIInterface;
use embedded_hal::i2c::*;
use embedded_hal::spi::*;

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

pub struct SpiDisplayInterface;

pub enum DataCommand {
    Data,
    Command,
}

impl SpiDisplayInterface {
    /// Create a new SPI interface.
    pub fn new_interface<S: SpiDevice>(spi: S, dc: DataCommand) -> SPIInterface<S, DataCommand> {
        SPIInterface::new(spi, dc)
    }
}
