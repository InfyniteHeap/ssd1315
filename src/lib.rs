//! # SSD1315
//!
//! SSD1315 driver.
//!
//! ## Usage
//!
//! Import the ssd1315:
//!
//! ```rust
//! use ssd1315::*;
//! ```
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
//! Now we can create an SSD1315 instance:
//!
//! ```rust
//! let mut display = Ssd1315::new(interface);
//! ```
//!
//! We created an SSD1315 instance! But the OLED screen cannot work now.
//! That's because we need to initialize OLED screen first before using it:
//!
//! ```rust
//! display.init();
//! ```
//!
//! Let's draw a circle on our OLED screen!
//! Now we add `embedded-graphics` dependency in `Cargo.toml` and import these items:
//!
//! ```rust
//! use embedded_graphics::{
//!     pixelcolor::BinaryColor,
//!     prelude::*,
//!     primitives::{Circle, PrimitiveStyle},
//! };
//! ```
//!
//! Then create a circle instance:
//!
//! ```rust
//! Circle::new(Point::new(16, 15), 10)
//!         .into_styled(PrimitiveStyle::with_fill(BinaryColor::On))
//!         .draw(&mut display)
//!         .unwrap();
//! ```
//!
//! Last we must flush the `display` to let OLED screen actually display what we want:
//! ```rust
//! display.flush();
//! ```
//!
//! Congratulations! Now you can see a little circle that is displaying on your OLED screen!

#![no_std]

mod command;
pub mod interface;

use command::{oled_clear_screen, oled_init, oled_set_cursor};

use display_interface::{DataFormat, WriteOnlyDataCommand};
use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::geometry::{OriginDimensions, Size};
use embedded_graphics::pixelcolor::raw::ToBytes;
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::Pixel;

/// A virtal SSD1315 device that holds an interface data
/// and a buffer that maps to the actual buffer in the SSD1315.
pub struct Ssd1315<DI> {
    pub interface: DI,
    pub buffer: [[u8; 128]; 8],
}

impl<DI: WriteOnlyDataCommand> Ssd1315<DI> {
    /// Create a new instance of SSD1315.
    ///
    /// The `interface` can be either an `I2c` interface or an `Spi` interface.
    pub fn new(interface: DI) -> Self {
        Self {
            interface,
            buffer: [[0; 128]; 8],
        }
    }
}

impl<DI: WriteOnlyDataCommand> OriginDimensions for Ssd1315<DI> {
    fn size(&self) -> Size {
        Size::new(128, 64)
    }
}

impl<DI: WriteOnlyDataCommand> DrawTarget for Ssd1315<DI> {
    type Color = BinaryColor;
    type Error = core::convert::Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for Pixel(coord, color) in pixels.into_iter() {
            match coord.into() {
                // As controling a single pixel is a little hard and annoy to SSD1315,
                // this look-like-stupid method is adopted.
                (x @ 1..=128, y @ 1..=8) => {
                    self.buffer[0][x as usize - 1] |= color.to_ne_bytes()[0] << (y - 1)
                }
                (x @ 1..=128, y @ 9..=16) => {
                    self.buffer[1][x as usize - 1] |= color.to_ne_bytes()[0] << (y - 8 - 1)
                }
                (x @ 1..=128, y @ 17..=24) => {
                    self.buffer[2][x as usize - 1] |= color.to_ne_bytes()[0] << (y - 16 - 1)
                }
                (x @ 1..=128, y @ 25..=32) => {
                    self.buffer[3][x as usize - 1] |= color.to_ne_bytes()[0] << (y - 24 - 1)
                }
                (x @ 1..=128, y @ 33..=40) => {
                    self.buffer[4][x as usize - 1] |= color.to_ne_bytes()[0] << (y - 32 - 1)
                }
                (x @ 1..=128, y @ 41..=48) => {
                    self.buffer[5][x as usize - 1] |= color.to_ne_bytes()[0] << (y - 40 - 1)
                }
                (x @ 1..=128, y @ 49..=56) => {
                    self.buffer[6][x as usize - 1] |= color.to_ne_bytes()[0] << (y - 48 - 1)
                }
                (x @ 1..=128, y @ 57..=64) => {
                    self.buffer[7][x as usize - 1] |= color.to_ne_bytes()[0] << (y - 56 - 1)
                }
                _ => unreachable!(
                    "`x` coordinate or `page` coordinate indexed out of bound of its corresponding size of OLED screen!"
                ),
            }
        }

        Ok(())
    }
}

impl<DI: WriteOnlyDataCommand> Ssd1315<DI> {
    /// Initialize SSD1315.
    pub fn init(&mut self) {
        oled_init(&mut self.interface);
        oled_clear_screen(&mut self.interface);
    }

    /// Flush SSD1315 buffer to make contents actually displays on the OLED screen.
    pub fn flush(&mut self) {
        for (page, data) in self.buffer.iter().enumerate() {
            oled_set_cursor(&mut self.interface, page as u8);
            self.interface.send_data(DataFormat::U8(data)).unwrap();
        }
    }
}
