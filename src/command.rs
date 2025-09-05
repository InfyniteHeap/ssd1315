use crate::config::Ssd1315DisplayConfig;
use InitCommand::*;
use SetCursorCommand::*;

use display_interface::{DataFormat, WriteOnlyDataCommand};

/// Commands that are used to initialize OLED screen.
enum InitCommand {
    DisplaySwitch(u8),
    EntireDisplaySwitch(u8),

    SetDisplayClockDivideRatioAndOscillatorFreq(u8, u8),
    SetMultiplexRatio(u8, u8),
    SetDisplayOffset(u8, u8),
    SetDisplayStartLine(u8),
    SetSegmentRemap(u8),
    SetComOutputScanDirection(u8),
    SetComPinsHardwareConfig(u8, u8),
    SetContrastControl(u8, u8),
    SetPrechargePeriod(u8, u8),
    SetVcomhDeselectLevel(u8, u8),
    SetNormalInverseDisplay(u8),

    SetChargePump(u8, u8),
}

/// Commands that are used to set the cursor of the OLED screen.
enum SetCursorCommand {
    // Set the page where data will be written.
    SetPage(u8),
    // Set the x coordinate where data will be written.
    SetXCoordinate(u8),
}

enum DisplaySwitch {
    On = 0xaf,
    Off = 0xae,
}

impl InitCommand {
    fn init<DI: WriteOnlyDataCommand>(self, ins: &mut DI) {
        match self {
            DisplaySwitch(cmd)
            | SetDisplayStartLine(cmd)
            | SetSegmentRemap(cmd)
            | SetComOutputScanDirection(cmd)
            | EntireDisplaySwitch(cmd)
            | SetNormalInverseDisplay(cmd) => {
                ins.send_commands(DataFormat::U8(&[cmd])).unwrap();
            }

            SetDisplayClockDivideRatioAndOscillatorFreq(cmd0, cmd1)
            | SetMultiplexRatio(cmd0, cmd1)
            | SetDisplayOffset(cmd0, cmd1)
            | SetComPinsHardwareConfig(cmd0, cmd1)
            | SetContrastControl(cmd0, cmd1)
            | SetPrechargePeriod(cmd0, cmd1)
            | SetVcomhDeselectLevel(cmd0, cmd1)
            | SetChargePump(cmd0, cmd1) => {
                ins.send_commands(DataFormat::U8(&[cmd0, cmd1])).unwrap();
            }
        }
    }
}

impl SetCursorCommand {
    fn set_cursor<DI: WriteOnlyDataCommand>(self, ins: &mut DI) {
        match self {
            SetPage(page) => ins.send_commands(DataFormat::U8(&[0xb0 | page])).unwrap(),
            SetXCoordinate(x) => {
                ins.send_commands(DataFormat::U8(&[x & 0x0f])).unwrap();
                ins.send_commands(DataFormat::U8(&[0x10 | ((x >> 4) & 0x0f)]))
                    .unwrap();
            }
        }
    }
}

pub(crate) fn oled_init<DI: WriteOnlyDataCommand>(ins: &mut DI, config: Ssd1315DisplayConfig) {
    const ENTIRE_DISPLAY_ON: u8 = 0xa4;

    [
        // VDD/VBAT off State
        DisplaySwitch(DisplaySwitch::Off as u8),
        // Initial Settings Configuration
        SetDisplayClockDivideRatioAndOscillatorFreq(
            0xd5,
            config.display_clock_divide_ratio_and_oscillator_freq,
        ),
        SetMultiplexRatio(0xa8, config.multiplex_ratio),
        SetDisplayOffset(0xd3, config.display_offset),
        SetDisplayStartLine(config.display_start_line),
        SetSegmentRemap(config.segment_remap as u8),
        SetComOutputScanDirection(config.com_output_scan_direction as u8),
        SetComPinsHardwareConfig(0xda, config.com_pins_hardware_config),
        SetContrastControl(0x81, config.contrast),
        SetPrechargePeriod(0xd9, config.precharge_period),
        SetVcomhDeselectLevel(0xdb, config.v_comh_select_level),
        EntireDisplaySwitch(ENTIRE_DISPLAY_ON),
        SetNormalInverseDisplay(config.normal_or_inverse_display as u8),
        // Clear Screen
        SetChargePump(0x8d, config.charge_pump as u8),
        DisplaySwitch(DisplaySwitch::On as u8),
    ]
    .into_iter()
    .for_each(|cmd| cmd.init(ins));
}

pub(crate) fn oled_set_cursor<DI: WriteOnlyDataCommand>(ins: &mut DI, page: u8) {
    [SetPage(page), SetXCoordinate(0)]
        .into_iter()
        .for_each(|cmd| cmd.set_cursor(ins))
}
