//! SSD1315
//!
//! Usage
//!
//! We assume that you have already created an `I2c` instance.
//! Before using OLED screen driven by SSD1315/SSD1306, you need to create an `I2CInterface` or `SPIInterface`:
//!
//! ```rust
//! let i2c_interface = I2cDisplayInterface::new_interface(i2c);
//! ```
//!
//! or
//!
//! ```rust
//! let spi_interface = SpiDisplayInterface::new_interface(spi);
//! ```

#![no_std]

mod command;
mod error;
mod function;
mod interface;
mod prelude;

use display_interface_i2c::I2CInterface;
use display_interface_spi::SPIInterface;
use embedded_hal::{i2c::I2c, spi::SpiDevice};

use function::{display::DisplayMode, rotation::DisplayRotation};
use interface::spi::DataCommand;

pub struct I2cSsd1315<I: I2c> {
    pub interface: I2CInterface<I>,
    pub display_mode: DisplayMode,
    pub rotation: DisplayRotation,
}

pub struct SpiSsd1315<S: SpiDevice> {
    pub interface: SPIInterface<S, DataCommand>,
}

impl<I: I2c> I2cSsd1315<I> {
    pub fn new(i2c: I2CInterface<I>, display_mode: DisplayMode, rotation: DisplayRotation) -> Self {
        Self {
            interface: i2c,
            display_mode,
            rotation,
        }
    }
}

impl<S: SpiDevice> SpiSsd1315<S> {
    pub fn new(spi: SPIInterface<S, DataCommand>) -> Self {
        Self { interface: spi }
    }
}
