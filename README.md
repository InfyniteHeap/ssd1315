# SSD1315

The SSD1315 OLED driver.

## Compatibility

This driver was developed for the SSD1315 and is also compatible with the SSD1306. Please note that the SSD1315 does not support parallel ports (such as 6800 or 8080), so you cannot use parallel ports with this crate.

> [!NOTE]
> Since this driver was developed from scratch, it supports only the new features provided by `embedded-hal` 1.0.0.
>
> Please verify whether the HAL library you use has already adopted `embedded-hal` 1.0.0, or consider using [ssd1306](https://github.com/rust-embedded-community/ssd1306).

## Example

Here is a full example (the MCU model is STM32F411CEU6):

```rust
#![deny(unsafe_code)]
#![no_std]
#![no_main]

use cortex_m::asm::nop;
use cortex_m_rt::entry;
use embedded_graphics::{
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Circle, PrimitiveStyle},
};
use panic_halt as _;
use ssd1315::*;
use stm32f4xx_hal::{
    i2c::{DutyCycle, Mode},
    pac,
    prelude::*,
};

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.freeze();

    let gpiob = dp.GPIOB.split();
    let (scl, sda) = (
        gpiob.pb8.into_alternate_open_drain(),
        gpiob.pb9.into_alternate_open_drain(),
    );

    let i2c = dp.I2C1.i2c(
        (scl, sda),
        Mode::fast(400000.Hz(), DutyCycle::Ratio2to1),
        &clocks,
    );

    let interface = interface::I2cDisplayInterface::new_interface(i2c);
    let config = config::Ssd1315DisplayConfig::preset_config();

    let mut display = Ssd1315::new(interface);
    display.set_custom_config(config);

    Circle::new(Point::new(0, 0), 40)
        .into_styled(PrimitiveStyle::with_fill(BinaryColor::On))
        .draw(&mut display)
        .unwrap();

    display.init_screen();
    display.flush_screen();

    loop {
        nop()
    }
}
```

## License

This software is distributed under GPL-3.0 license.

## Contributing

Thank you for your interest in contributing to this project! If you find any bugs or have suggestions to improve this project, please open an issue or submit a pull request! :)
