use display_interface::{DataFormat, WriteOnlyDataCommand};
use InitCommand::*;

pub enum InitCommand {
    DisplaySwitch(u8),
    SetDisplayClockDivideRatioAndOscillatorFreq((u8, u8)),
    SetMultiplexRatio((u8, u8)),
    SetDisplayOffset((u8, u8)),
    SetDisplayStartLine(u8),
    SetSegmentRemap(u8),
    SetComOutputScanDirection(u8),
    SetComPinsHardwareConfig((u8, u8)),
    SetContrastControl((u8, u8)),
    SetPrechargePeriod((u8, u8)),
    SetVcomhDeselectLevel((u8, u8)),
    EntireDisplaySwitch(u8),
    SetNormalInverseDisplay(u8),
    SetChargePump((u8, u8)),
}

impl InitCommand {
    pub fn send_init_command<W: WriteOnlyDataCommand>(self, ins: &mut W) {
        match self {
            DisplaySwitch(cmd)
            | SetDisplayStartLine(cmd)
            | SetSegmentRemap(cmd)
            | SetComOutputScanDirection(cmd)
            | EntireDisplaySwitch(cmd)
            | SetNormalInverseDisplay(cmd) => {
                ins.send_commands(DataFormat::U8(&[cmd])).unwrap();
            }

            SetDisplayClockDivideRatioAndOscillatorFreq(cmd)
            | SetMultiplexRatio(cmd)
            | SetDisplayOffset(cmd)
            | SetComPinsHardwareConfig(cmd)
            | SetContrastControl(cmd)
            | SetPrechargePeriod(cmd)
            | SetVcomhDeselectLevel(cmd)
            | SetChargePump(cmd) => {
                ins.send_commands(DataFormat::U8(&[cmd.0, cmd.1])).unwrap();
            }
        }
    }
}

pub fn oled_init<W: WriteOnlyDataCommand>(ins: &mut W, _config: u8) {
    let init_commands = [
        // VDD/VBAT off State
        DisplaySwitch(0xae),
        // Initial Settings Configuration
        SetDisplayClockDivideRatioAndOscillatorFreq((0xd5, 0x90)),
    ];

    for cmd in init_commands {
        cmd.send_init_command(ins)
    }
}
