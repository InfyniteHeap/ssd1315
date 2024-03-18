use display_interface_i2c::I2CInterface;
use display_interface_spi::SPIInterface;
use embedded_hal::digital::{ErrorKind as DEK, ErrorType as DET, OutputPin, PinState};
use embedded_hal::i2c::*;
use embedded_hal::spi::*;

const DATA_TYPE: u8 = 0x40;

/// An empty struct that helps to organize the different interfaces.
pub struct I2cDisplayInterface;

impl I2cDisplayInterface {
    /// Create a new I2C interface.
    pub fn new_interface<I: I2c>(i2c: I) -> I2CInterface<I> {
        Self::new_custom_interface(i2c, 0x00)
    }

    pub fn new_alternative_interface<I: I2c>(i2c: I) -> I2CInterface<I> {
        Self::new_custom_interface(i2c, 0x00)
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

impl DET for DataCommand {
    type Error = DEK;
}

impl OutputPin for DataCommand {
    fn set_low(&mut self) -> Result<(), Self::Error> {
        self.set_state(PinState::Low)
    }

    fn set_high(&mut self) -> Result<(), Self::Error> {
        self.set_state(PinState::High)
    }
}

impl SpiDisplayInterface {
    /// Create a new SPI interface with command mode.
    pub fn new_command_interface<S: SpiDevice>(spi: S) -> SPIInterface<S, DataCommand> {
        Self::new_interface(spi, DataCommand::Command)
    }

    /// Create a new SPI interface with data mode.
    pub fn new_data_interface<S: SpiDevice>(spi: S) -> SPIInterface<S, DataCommand> {
        Self::new_interface(spi, DataCommand::Data)
    }

    fn new_interface<S: SpiDevice>(spi: S, dc: DataCommand) -> SPIInterface<S, DataCommand> {
        SPIInterface::new(spi, dc)
    }
}
