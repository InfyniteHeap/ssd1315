//! # SSD1315
//!
//! SSD1315 driver.
//!
//! ## Usage
//!
//! Here is an example about how to use `ssd1315`:
//!
//! ```rust
//! use ssd1315::*;
//! use embedded_graphics::{
//!     pixelcolor::BinaryColor,
//!     prelude::*,
//!     primitives::{Circle, PrimitiveStyle},
//! };
//!
//! let interface = interface::I2cDisplayInterface::new(i2c);
//! // let interface = interface::SpiDisplayInterface::new(spi, dc);
//!
//! let mut display = Ssd1315::new(interface);
//!
//! Circle::new(Point::new(1, 1), 40)
//!         .into_styled(PrimitiveStyle::with_fill(BinaryColor::On))
//!         .draw(&mut display)
//!         .unwrap();
//!
//! display.init_screen();
//! display.flush_screen();
//! ```
//!
//! Congratulations! Now you can see a little circle that is displaying on your OLED screen!
//!
//! If you want to apply your own config for SSD1315 when it is initializing,
//! follow this example (we assume that you want to change the contrast of the OLED screen):
//!
//! ```rust
//! use ssd1315::*;
//! use embedded_graphics::{
//!     pixelcolor::BinaryColor,
//!     prelude::*,
//!     primitives::{Circle, PrimitiveStyle},
//! };
//!
//! let interface = interface::I2cDisplayInterface::new(i2c);
//! // let interface = interface::SpiDisplayInterface::new(spi, dc);
//!
//! let mut config = config::Ssd1315DisplayConfig::new();
//! config.contrast = 0xff;
//!
//! let mut display = Ssd1315::new(interface);
//! display.set_custom_config(config);
//!
//! Circle::new(Point::new(1, 1), 40)
//!         .into_styled(PrimitiveStyle::with_fill(BinaryColor::On))
//!         .draw(&mut display)
//!         .unwrap();
//!
//! display.init_screen();
//! display.flush_screen();
//! ```
//!
//! Or use a pre-set config that was provided by `ssd1315`:
//! ```rust
//! let config = config::Ssd1315DisplayConfig::preset_config();
//! ```
//!
//! Now you can see the change of contrast!
//!
//! You might also want to draw some raw image(s) manually to fit your specific requirements.
//! That's no matter! You can draw it/them in an easy way:
//!
//! ```rust
//! let mut display = Ssd1315::new(interface);
//!
//! let raw_image = [[0b1010_1010; 8]; 128];
//! raw_image.draw_from_raw(&mut display);
//!
//! display.init_screen();
//! display.flush_screen();
//! ```

#![no_std]

mod command;
pub mod config;
pub mod interface;

use command::{oled_init, oled_set_cursor};
use config::Ssd1315DisplayConfig;

use display_interface::{DataFormat, WriteOnlyDataCommand};
use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::geometry::{OriginDimensions, Size};
use embedded_graphics::pixelcolor::raw::ToBytes;
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::Pixel;

/// A virtal SSD1315 device that holds an interface data
/// and a buffer that maps to the actual buffer in the SSD1315.
pub struct Ssd1315<DI> {
    interface: DI,
    buffer: [[u8; 128]; 8],
    config: Ssd1315DisplayConfig,
}

impl<DI: WriteOnlyDataCommand> Ssd1315<DI> {
    /// Create a new instance of SSD1315.
    ///
    /// The `interface` can be either an `I2c` interface or an `Spi` interface.
    pub fn new(interface: DI) -> Self {
        Self {
            interface,
            buffer: [[0; 128]; 8],
            config: Default::default(),
        }
    }

    /// Set your custom configs to SSD1315, or it'll be initialized with default one.
    ///
    /// You needn't to call this function if you keep all configs by default.
    pub fn set_custom_config(&mut self, config: Ssd1315DisplayConfig) {
        self.config = config;
    }

    /// Initialize SSD1315.
    pub fn init_screen(&mut self) {
        oled_init(&mut self.interface, self.config);
    }

    /// Flush SSD1315 buffer to make contents actually displays on the OLED screen.
    pub fn flush_screen(&mut self) {
        for (page, data) in self.buffer.iter().enumerate() {
            oled_set_cursor(&mut self.interface, page as u8);
            self.interface.send_data(DataFormat::U8(data)).unwrap();
        }

        self.buffer = [[0; 128]; 8];
    }
}

impl<DI> OriginDimensions for Ssd1315<DI> {
    fn size(&self) -> Size {
        Size::new(128, 64)
    }
}

impl<DI> DrawTarget for Ssd1315<DI> {
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

pub trait DrawFromRaw {
    fn draw_from_raw<DI>(&self, instance: &mut Ssd1315<DI>);
}

impl DrawFromRaw for [[u8; 128]; 8] {
    /// Draw an raw image.
    fn draw_from_raw<DI>(&self, instance: &mut Ssd1315<DI>) {
        instance.buffer.clone_from(self)
    }
}
