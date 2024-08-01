/// Normal = `0xa0` (RESET), Remapped = `0xa1`.
#[derive(Clone, Copy)]
pub enum SegmentRemap {
    /// Column address `0` is mapped to `SEG0`. (RESET)
    Normal = 0xa0,
    /// Column address `127` is mapped to `SEG0`.
    Remapped = 0xa1,
}

/// Normal = `0xc0` (RESET), Remapped = `0xc8`.
#[derive(Clone, Copy)]
pub enum ComOutputScanDirection {
    /// Normal Mode (RESET)
    ///
    /// Scan from `COM0` to `COM[N-1]`.
    ///
    /// Where `N` is the Multiplex ratio.
    Normal = 0xc0,
    /// Remapped Mode
    ///
    /// Scan from `COM[N-1]` to `COM0`.
    ///
    /// Where `N` is the Multiplex ratio.
    Remapped = 0xc8,
}

/// Normal = `0xa6` (RESET), Inverse = `0xa7`.
#[derive(Clone, Copy)]
pub enum NormalOrInverseDisplay {
    Normal = 0xa6,
    Inverse = 0xa7,
}

/// Disable = `0x10`,
/// V7Point5 = `0x14` (RESET),
/// V8Point5 = `0x94`,
/// V9Point0 = `0x95`.
#[derive(Clone, Copy)]
pub enum ChargePump {
    Disable = 0x10,
    /// 7.5V (RESET)
    V7Point5 = 0x14,
    /// 8.5V
    V8Point5 = 0x94,
    /// 9.0V
    V9Point0 = 0x95,
}

/// Configurations that applies to SSD1315.
///
/// All configurations can be found in the SSD1315 Command Table,
/// which is in the SSD1315 datasheet.
///
/// Note: In the following bits details,
/// `x` represents either `0` or `1`
/// while `*` represents this bit is useless, and it could be both `0` and `1`.
/// It's better to assign options with a hex number when configuring options.
#[derive(Clone, Copy)]
pub struct Ssd1315DisplayConfig {
    /// Bits detail: `0bxxxx_xxxx`.
    ///
    /// The lower 4 bits define the divide ratio of the display clocks,
    /// while the higher 4 bits set the oscillator frequency, or *f*<sub>OSC</sub>.
    ///
    /// Divide ratio = `A[3:0] + 1`. RESET = `0b0000` (divide ratio = 1).
    ///
    /// Oscillator Frequency increases with the value of `A[7:4]` and vice versa.
    /// Range: `0b0000` - `0b1111` (RESET = `0b1000`).
    pub display_clock_divide_ratio_and_oscillator_freq: u8,
    /// Bits detail: `0b**xx_xxxx`.
    ///
    /// Set MUX ratio from `0` to `N+1`.
    ///
    /// N = A\[5:0]: `0b00_1111` - `0b11_1111`. RESET = `0b11_1111`, or 64MUX.
    ///
    /// A\[7:6] can be arbitrary values, but it's better to keep it `0b00`.
    pub multiplex_ratio: u8,
    /// Bits detail: `0b**xx_xxxx`.
    ///
    /// Set vertical shift by COM from `0b0000_0000` to `0b0011_1111`. The value is reset to `0b0000_0000` after RESET.
    pub display_offset: u8,
    /// Bits detail: `0b01xx_xxxx`.
    ///
    /// Set display RAM display start line register from `0b00_0000` to `0b11_1111`.
    ///
    /// Display start line register is reset to `0b0100_0000` after RESET.
    pub display_start_line: u8,
    /// Bits detail: `0b1010_000x`.
    ///
    /// Set segment remap.
    pub segment_remap: SegmentRemap,
    /// Bits detail: `0b1100_x000`.
    ///
    /// Set COM output scan direction.
    pub com_output_scan_direction: ComOutputScanDirection,
    /// Bits detail: `0b00xx_0010`.
    ///
    /// A\[4] = `0b0`, Sequential COM pin configuration,
    /// A\[4] = `0b1` (RESET), Alternative COM pin Configuration.
    ///
    /// A\[5] = `0b0` (RESET), Disable COM Left/Right remap,
    /// A\[5] = `0b1`, Enable COM Left/Right remap.
    pub com_pins_hardware_config: u8,
    /// Bits detail: `0bxxxx_xxxx`.
    ///
    /// Contrast increases as the value increases (RESET=`0x7f`).
    ///
    /// A\[7:0] valid range: `0b0000_0001`-`0b1111_1111`.
    pub contrast: u8,
    /// Bits detail: `0bxxxx_xxxx`.
    ///
    /// A\[3:0]: Phase 1 period of up to 30 DCLK, Clocks 0 is invalid entry (RESET = `0b0010`).
    ///
    /// A\[7:4]: Phase 2 period of up to 30 DCLK, Clocks 0 is invalid entry (RESET = `0b0010`).
    pub precharge_period: u8,
    /// Bits detail: `0b00xx_0000`.
    ///
    /// Set V<sub>COMH</sub> select voltage level.
    ///
    /// |`A[5:4]`|Hex Code|V<sub>COMH</sub> deselect level|
    /// |:------:|:------:|:-----------------------------:|
    /// |`0x00`  |`0x00`  |~0.65×V<sub>cc</sub>           |
    /// |`0x01`  |`0x10`  |~0.71×V<sub>cc</sub>           |
    /// |`0x10`  |`0x20`  |~0.77×V<sub>cc</sub> (RESET)   |
    /// |`0x11`  |`0x30`  |~0.83×V<sub>cc</sub>           |
    pub v_comh_select_level: u8,
    /// Bits detail: `0b1010_011x`.
    ///
    /// Set normal/inverse display.
    pub normal_or_inverse_display: NormalOrInverseDisplay,
    /// Bits detail: `0bx001_0x0x`.
    ///
    /// Enable/Disable internal charge pump:
    ///
    /// A\[2] = `0b0`, Disable charge pump (RESET),
    /// A\[2] = `0b1`, Enable charge pump during display on.
    ///
    /// |`A[7]`|`A[0]`|Hex Code|Charge Pump Mode|
    /// |:----:|:----:|:------:|:--------------:|
    /// |`0x0` |`0x0` |`0x14`  |7.5V (RESET)    |
    /// |`0x1` |`0x0` |`0x94`  |8.5V            |
    /// |`0x1` |`0x1` |`0x95`  |9.0V            |
    pub charge_pump: ChargePump,
}

impl Ssd1315DisplayConfig {
    /// A pre-set configuration.
    pub fn preset_config() -> Self {
        Self {
            display_clock_divide_ratio_and_oscillator_freq: 0x90,
            segment_remap: SegmentRemap::Remapped,
            com_output_scan_direction: ComOutputScanDirection::Remapped,
            contrast: 0xb0,
            v_comh_select_level: 0x30,
            ..Default::default()
        }
    }
}

impl Default for Ssd1315DisplayConfig {
    fn default() -> Self {
        Self {
            display_clock_divide_ratio_and_oscillator_freq: 0x80,
            multiplex_ratio: 0x3f,
            display_offset: 0x00,
            display_start_line: 0x40,
            segment_remap: SegmentRemap::Normal,
            com_output_scan_direction: ComOutputScanDirection::Normal,
            com_pins_hardware_config: 0x12,
            contrast: 0x7f,
            precharge_period: 0x22,
            v_comh_select_level: 0x20,
            normal_or_inverse_display: NormalOrInverseDisplay::Normal,
            charge_pump: ChargePump::V7Point5,
        }
    }
}
