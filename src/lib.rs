//! # SSD1315 (WIP)
//!
//! SSD1315 driver.
//!
//! ## Usage
//!
//! We assume you have already created an I2C or SPI instance.
//!
//! Create an I2C interface:
//!
//! ```rust
//! let interface = interface::I2cDisplayInterface::new(i2c);
//! ```
//!
//! or the SPI version (you must additionally create a GPIO instance in advanced):
//!
//! ```rust
//! let interface = interface::SpiDisplayInterface::new(spi, dc);
//! ```
//!
//! Before creating an SSD1315 instance, we must create a `DisplayConfig` instance:
//!
//! ```rust
//! let config = config::DisplayConfig::new();
//! ```
//!
//! Now we can create an SSD1315 instance:
//!
//! ```rust
//! let mut display = Ssd1315::new(interface, config).into_non_buffered_mode();
//! ```
//!
//! We created an SSD1315 instance! But the OLED screen cannot work now.
//! That's because we need to initialize OLED screen first before using it:
//!
//! ```rust
//! display.init();
//! ```
//!
//! To let OLED screen displays anything on it, we can call a function and pass something to it:
//!
//! Because we chose non-buffered mode, when we compile our project, program it in our MCU and power on the MCU,
//! we'll see the contents would be displayed on the OLED screen.

#![no_std]

pub mod command;
pub mod config;
pub mod interface;
pub mod mode;

use config::DisplayConfig;
use display_interface::WriteOnlyDataCommand;
use mode::Mode;

pub struct Ssd1315<DI> {
    pub interface: DI,
    pub config: DisplayConfig,
    pub mode: Mode,
}

impl<DI: WriteOnlyDataCommand> Ssd1315<DI> {
    /// Create new instance of SSD1315.
    ///
    /// The `interface` can be either an `I2c` instance or an `Spi` instance.
    pub fn new(interface: DI, config: DisplayConfig) -> Self {
        Self {
            interface,
            config,
            mode: Mode::Basic,
        }
    }

    pub fn into_non_buffered_mode(mut self) -> Self {
        self.mode = Mode::NonBuffered;
        self
    }

    pub fn into_buffered_mode(mut self) -> Self {
        self.mode = Mode::Buffered;
        self
    }
}

#[cfg(test)]
pub fn test_init<DI: WriteOnlyDataCommand>(mut d: Ssd1315<DI>) {
    command::oled_init(&mut d.interface, 0x00)
}
