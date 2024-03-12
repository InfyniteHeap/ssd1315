use display_interface_spi::SPIInterface;
use embedded_hal::spi::*;

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
