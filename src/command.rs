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

pub(crate) fn oled_init<DI: WriteOnlyDataCommand>(ins: &mut DI) {
    let init_commands = [
        // VDD/VBAT off State
        DisplaySwitch(0xae),
        // Initial Settings Configuration
        SetDisplayClockDivideRatioAndOscillatorFreq(0xd5, 0x80),
        SetMultiplexRatio(0xa8, 0x3f),
        SetDisplayOffset(0xd3, 0x00),
        SetDisplayStartLine(0x40),
        SetSegmentRemap(0xa1),
        SetComOutputScanDirection(0xc8),
        SetComPinsHardwareConfig(0xda, 0x12),
        SetContrastControl(0x81, 0xcf),
        SetPrechargePeriod(0xd9, 0xf1),
        SetVcomhDeselectLevel(0xdb, 0x30),
        EntireDisplaySwitch(0xa4),
        SetNormalInverseDisplay(0xa6),
        // Clear Screen
        SetChargePump(0x8d, 0x14),
        DisplaySwitch(0xaf),
    ];

    for cmd in init_commands {
        cmd.init(ins)
    }
}

pub(crate) fn oled_set_cursor<DI: WriteOnlyDataCommand>(ins: &mut DI, page: u8) {
    let set_cursor_cmd = [SetPage(page), SetXCoordinate(0)];

    for cmd in set_cursor_cmd {
        cmd.set_cursor(ins)
    }
}

pub(crate) fn oled_clear_screen<DI: WriteOnlyDataCommand>(ins: &mut DI) {
    const VOID_ARRAY: [[u8; 128]; 8] = [[0; 128]; 8];

    for (page, data) in VOID_ARRAY.iter().enumerate() {
        oled_set_cursor(ins, page as u8);
        ins.send_data(DataFormat::U8(data)).unwrap();
    }
}
