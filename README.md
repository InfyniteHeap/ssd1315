# SSD1315

The SSD1315 OLED driver.

## Compatibility

This driver is developed for SSD1315, but it is also theoretically compatible with SSD1306. Please note that SSD1315 doesn't support parallel ports like 6800 and 8080, which means you cannot use parallel ports when using this crate.

As developed from scratch, this driver supports new features provided in `embedded-hal` 1.0.0, but that also means this driver doesn't compatible with older hal libraries (e.g. `stm32f1xx-hal`, which hasn't adopted `embedded-hal` 1.0.0 yet).

## License

This software is distributed under GPL-3.0 license.

## Contribution

Contributions to this project are welcome! If you find anywhere contains bugs, please open an issue or commit a pull request.