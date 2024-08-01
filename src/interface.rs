use display_interface_i2c::I2CInterface;
use display_interface_spi::SPIInterface;
use embedded_hal::{digital::OutputPin, i2c::I2c, spi::SpiDevice};

pub struct I2cDisplayInterface;

impl I2cDisplayInterface {
    /// Create new I2C interface.
    ///
    /// `i2c` is the instance of I2C device.
    pub fn new_interface<I: I2c>(i2c: I) -> I2CInterface<I> {
        Self::new_custom_interface(i2c, 0x3c)
    }

    /// Create new I2C interface with an alternative screen address set.
    pub fn new_alternative_interface<I: I2c>(i2c: I) -> I2CInterface<I> {
        Self::new_custom_interface(i2c, 0x3d)
    }

    /// Create new I2C interface for which user must specify a screen address.
    pub fn new_custom_interface<I: I2c>(i2c: I, addr: u8) -> I2CInterface<I> {
        I2CInterface::new(i2c, addr, 0x40)
    }
}

pub struct SpiDisplayInterface;

impl SpiDisplayInterface {
    /// Create new SPI interface.
    ///
    /// `spi` is the instance of SPI device while `dc` is the instance of GPIO that controls the data/command pin.
    pub fn new_interface<S: SpiDevice, DC: OutputPin>(spi: S, dc: DC) -> SPIInterface<S, DC> {
        SPIInterface::new(spi, dc)
    }
}
