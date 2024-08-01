# SSD1315

The SSD1315 OLED driver.

## Compatibility

This driver is developed for SSD1315, but it is also compatible with SSD1306. Please note that SSD1315 doesn't support parallel ports like 6800 and 8080, which means you cannot use parallel ports when using this crate.

As developed from scratch, this driver only supports new features provided by `embedded-hal` 1.0.0, on the contrary, that means this driver doesn't compatible with non-upgraded hal libraries (e.g. `stm32f1xx-hal`, which hasn't upgraded to `embedded-hal` 1.0.0 yet so far).

## Example

Here is a full example (The MCU model is STM32F411CEU6):

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

Thank you for your interest in contributing to this project! If you find anywhere contains bugs or any ideas that can make this project better and more effect, please open an issue or commit a pull request! :)