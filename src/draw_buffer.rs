use core::convert::Infallible;
use core::panic;

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
    type Error = Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for Pixel(coord, color) in pixels.into_iter() {
            let x = coord.x;
            let y = coord.y;

            if (0..128).contains(&x) && (0..64).contains(&y) {
                // Owing to most ARM devices adopt little-endian, we
                // use `to_le_bytes` to declare the bytes should order
                // with little-endian.
                self.buffer[y as usize / 8][x as usize] |= color.to_le_bytes()[0] << (y % 8);
            } else {
                // HACK: It seems not suitable to just trigger panic here.
                panic!()
            }
        }

        Ok(())
    }
}

impl DrawFromRaw for [[u8; 128]; 8] {
    /// Draw raw image.
    fn draw_from_raw<DI>(&self, instance: &mut Ssd1315<DI>) {
        instance.buffer.clone_from(self)
    }
}
