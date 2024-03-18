//! # SSD1315 (WIP)
//!
//! SSD1315 OLED driver.
//!
//! ## Usage
//!
//! We assume that you have already created an `I2c` or `Spi` instance.
//! Before using OLED screen driven by SSD1315/SSD1306, an `I2CInterface` or `SPIInterface` must be created:
//!
//! ```rust
//! let i2c_interface = I2cDisplayInterface::new_interface(i2c);
//! ```
//!
//! or
//!
//! ```rust
//! let spi_interface = SpiDisplayInterface::new_data_interface(spi);
//! ```
//!
//! Then, create a display config:
//! ```rust
//! let display_config = DisplayConfig::new(100, rotation::DisplayRotation::Rotate0, );
//! ```
//!
//! Now you can create an SSD1315 instance:
//!
//! ```rust
//! let mut display = I2cSsd1315::new(i2c_interface, display_config);
//! ```
//!
//! or
//!
//! ```rust
//! let mut display = SpiSsd1315::new(spi_interface, display_config);
//! ```

#![no_std]

pub mod command;
pub mod config;
pub mod interface;
pub mod prelude;

pub static mut DISPLAY_BUFFER: [[u8; 128]; 8] = [[0; 128]; 8];

use config::DisplayConfig;
use interface::DataCommand;

use display_interface::{DataFormat, DisplayError, WriteOnlyDataCommand};
use display_interface_i2c::I2CInterface;
use display_interface_spi::SPIInterface;
use embedded_hal::{i2c::I2c, spi::SpiDevice};

pub trait DisplayOperation: WriteOnlyDataCommand {
    fn init(&mut self);
    fn clear(&mut self);
    fn update(&mut self);
}

pub struct I2cSsd1315<I> {
    pub interface: I2CInterface<I>,
    pub display_config: DisplayConfig,
}

impl<I: I2c> I2cSsd1315<I> {
    pub fn new(i2c: I2CInterface<I>, config: DisplayConfig) -> Self {
        Self {
            interface: i2c,
            display_config: config,
        }
    }
}

impl<I: I2c> WriteOnlyDataCommand for I2cSsd1315<I> {
    fn send_commands(&mut self, cmd: DataFormat<'_>) -> Result<(), DisplayError> {
        self.interface.send_commands(cmd)
    }

    fn send_data(&mut self, buf: DataFormat<'_>) -> Result<(), DisplayError> {
        self.interface.send_data(buf)
    }
}

impl<I: I2c> DisplayOperation for I2cSsd1315<I> {
    fn init(&mut self) {
        todo!()
    }

    fn clear(&mut self) {
        todo!()
    }

    fn update(&mut self) {}
}

pub struct SpiSsd1315<S> {
    pub interface: SPIInterface<S, DataCommand>,
    pub display_config: DisplayConfig,
}

impl<S: SpiDevice> SpiSsd1315<S> {
    pub fn new(spi: SPIInterface<S, DataCommand>, config: DisplayConfig) -> Self {
        Self {
            interface: spi,
            display_config: config,
        }
    }
}

impl<S: SpiDevice> WriteOnlyDataCommand for SpiSsd1315<S> {
    fn send_commands(&mut self, cmd: DataFormat<'_>) -> Result<(), DisplayError> {
        self.interface.send_commands(cmd)
    }

    fn send_data(&mut self, buf: DataFormat<'_>) -> Result<(), DisplayError> {
        self.interface.send_data(buf)
    }
}

impl<S: SpiDevice> DisplayOperation for SpiSsd1315<S> {
    fn init(&mut self) {
        todo!()
    }

    fn clear(&mut self) {
        todo!()
    }

    fn update(&mut self) {
        todo!()
    }
}
