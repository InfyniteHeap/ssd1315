//! # SSD1315 (WIP)
//!
//! SSD1315 OLED driver.
//!
//! ## Usage
//!
//! We assume that you have already created an `I2c` or `Spi` instance.
//! Before using OLED screen driven by SSD1315/SSD1306, you need to create an `I2CInterface` or `SPIInterface`:
//!
//! ```rust
//! let i2c_interface = I2cDisplayInterface::new_interface(i2c);
//! ```
//!
//! or
//!
//! ```rust
//! let spi_interface = SpiDisplayInterface::new_interface(spi, DataCommand::Data);
//! ```
//!
//! Then, create a display config:
//! ```rust
//! let display_config = DisplayConfig::new();
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

use display_interface_i2c::I2CInterface;
use display_interface_spi::SPIInterface;
use embedded_hal::{i2c::I2c, spi::SpiDevice};

use config::DisplayConfig;
use interface::DataCommand;

pub trait DisplayOperation {
    fn update(&mut self);
}

pub struct I2cSsd1315<I: I2c> {
    pub interface: I2CInterface<I>,
    pub display_config: DisplayConfig,
}

pub struct SpiSsd1315<S: SpiDevice> {
    pub interface: SPIInterface<S, DataCommand>,
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

impl<S: SpiDevice> SpiSsd1315<S> {
    pub fn new(spi: SPIInterface<S, DataCommand>, config: DisplayConfig) -> Self {
        Self {
            interface: spi,
            display_config: config,
        }
    }
}

impl<I: I2c> DisplayOperation for I2cSsd1315<I> {
    fn update(&mut self) {
        todo!()
    }
}
