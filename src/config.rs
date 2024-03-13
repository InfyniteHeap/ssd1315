pub mod brightness;
pub mod display;
pub mod rotation;

pub struct DisplayConfig {
    pub brightness: u32,
    pub rotation: rotation::DisplayRotation,
    pub display: display::DisplayMode,
}

impl DisplayConfig {
    pub fn new(
        brightness: u32,
        rotation: rotation::DisplayRotation,
        display: display::DisplayMode,
    ) -> Self {
        Self {
            brightness,
            rotation,
            display,
        }
    }
}
