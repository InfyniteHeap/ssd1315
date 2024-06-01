use embedded_graphics_core::draw_target::DrawTarget;
use embedded_graphics_core::geometry::{OriginDimensions, Size};
use embedded_graphics_core::pixelcolor::raw::ToBytes;
use embedded_graphics_core::pixelcolor::BinaryColor;
use embedded_graphics_core::Pixel;

use crate::{DrawFromRaw, Ssd1315};

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
                (x @ 0..=127, y @ 0..=7) => {
                    self.buffer[0][x as usize] |= color.to_ne_bytes()[0] << y
                }
                (x @ 0..=127, y @ 8..=15) => {
                    self.buffer[1][x as usize] |= color.to_ne_bytes()[0] << (y - 8)
                }
                (x @ 0..=127, y @ 16..=23) => {
                    self.buffer[2][x as usize] |= color.to_ne_bytes()[0] << (y - 16)
                }
                (x @ 0..=127, y @ 24..=31) => {
                    self.buffer[3][x as usize] |= color.to_ne_bytes()[0] << (y - 24)
                }
                (x @ 0..=127, y @ 32..=39) => {
                    self.buffer[4][x as usize] |= color.to_ne_bytes()[0] << (y - 32)
                }
                (x @ 0..=127, y @ 40..=47) => {
                    self.buffer[5][x as usize] |= color.to_ne_bytes()[0] << (y - 40)
                }
                (x @ 0..=127, y @ 48..=55) => {
                    self.buffer[6][x as usize] |= color.to_ne_bytes()[0] << (y - 48)
                }
                (x @ 0..=127, y @ 56..=63) => {
                    self.buffer[7][x as usize] |= color.to_ne_bytes()[0] << (y - 56)
                }
                _ => unreachable!(
                    "`x` coordinate or `page` coordinate indexed out of bound of its corresponding size of OLED screen!"
                ),
            }
        }

        Ok(())
    }
}

impl DrawFromRaw for [[u8; 128]; 8] {
    /// Draw an raw image.
    fn draw_from_raw<DI>(&self, instance: &mut Ssd1315<DI>) {
        instance.buffer.clone_from(self)
    }
}
