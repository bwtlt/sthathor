use crate::parsing::RhothorCommand;
use serde::Deserialize;
use serde::Serialize;

/// Atomic Newson command
#[derive(Debug, Serialize, PartialEq)]
pub struct CMD3G {
    x: u16,
    y: u16,
    xh: u8,
    yh: u8,
    op_code: CMD3G_OPCODE,
    target: u8,
}
impl CMD3G {
    pub fn new(x: u16, y: u16, xh: u8, yh: u8, op_code: CMD3G_OPCODE, target: u8) -> CMD3G {
        CMD3G {
            x,
            y,
            xh,
            yh,
            op_code,
            target,
        }
    }
    pub fn new_movement(pos: &RawPosition, op_code: CMD3G_OPCODE, target: u8) -> CMD3G {
        CMD3G {
            x: pos.x,
            y: pos.y,
            xh: pos.xh,
            yh: pos.yh,
            op_code,
            target,
        }
    }
}

/// Target status, including positions, inputs/outputs state, etc.
#[derive(Debug, Deserialize)]
pub struct TgtStatus {
    inputs: u16,
    outputs: u16,
    me: u8,
    return_value: u8,
    counter: u16,
    analog: [u8; 4],
    otfx: i32,
    otfy: i32,
    tablex: i32,
    tabley: i32,
    tablez: i32,
    deflx: i32,
    defly: i32,
    deflz: i32,
}
impl TgtStatus {
    pub fn get_position(&self) -> (i32, i32) {
        (self.deflx, self.defly)
    }
}

pub const TARGET: u8 = 0x01;
pub const SYSIDLE: u8 = 0x40;
pub const TGTALL: u8 = 0xFF;

/// Newson command types
#[derive(Debug, Serialize, PartialEq, Clone)]
#[serde(into = "u8")]
#[allow(non_camel_case_types)]
#[repr(u8)]
pub enum CMD3G_OPCODE {
    CMD3G_NOP = 0x00,
    CMD3G_SETIO = 0x02,
    CMD3G_SETSPEED = 0x03,
    CMD3G_JUMPTO = 0x04,
    CMD3G_MOVETO = 0x05,
    CMD3G_PULSTO = 0x06,
    CMD3G_LINETO = 0x07,
    CMD3G_PARAMS = 0x08,
    CMD3G_ARCMOVE = 0x09,
    CMD3G_ARCLINE = 0x0a,
    CMD3G_SPEED = 0x0b,
    CMD3G_JUMPSPEED = 0x0c,
    CMD3G_SETOSC = 0x0d,
    CMD3G_SETLOOP = 0x0e,
    CMD3G_DOLOOP = 0x0f,
    CMD3G_SLEEP = 0x10,
    CMD3G_SUSPEND = 0x11,
    CMD3G_SETDELAYS = 0x12,
    CMD3G_SETLIDLE = 0x13,
    CMD3G_SM_LASER = 0x14,
    CMD3G_EXCHLLINK = 0x15,
    CMD3G_TABLESET = 0x16,
    CMD3G_TABLESSIZE = 0x17,
    CMD3G_TABLEDELAY = 0x18,
    CMD3G_TABLE1D = 0x19,
    CMD3G_SETFDATA = 0x1a,
    CMD3G_TABLEMOVETO = 0x1b,
    CMD3G_SETIMGAIJ = 0x1c,
    CMD3G_SETIMGROT = 0x1d,
    CMD3G_SETIMGOFFS = 0x1e,
    CMD3G_SETIMGOFFSR = 0x1f,
    CMD3G_SETIMGOFFSZ = 0x20,
    CMD3G_SETFS = 0x21,
    CMD3G_SETFSZ = 0x22,
    CMD3G_SETSPFLTR = 0x23,
    CMD3G_BURST = 0x24,
    CMD3G_IDXFETCH = 0x25,
    CMD3G_LIST = 0x26,
    CMD3G_SETCNTR = 0x27,
    CMD3G_SETANA = 0x28,
    CMD3G_CFG_IO = 0x29,
    CMD3G_OTFSTEP = 0x2a,
    CMD3G_OTFRANGE = 0x2b,
    CMD3G_OTFSET = 0x2c,
    CMD3G_IOWAIT = 0x2d,
    CMD3G_OTFWAIT = 0x2e,
    CMD3G_OTFENABLE = 0x2f,
    CMD3G_SETMINGATE = 0x30,
    CMD3G_SETBULGE = 0x31,
    CMD3G_SETDDELAY = 0x32,
    CMD3G_CIRCLE = 0x33,
    CMD3G_TABLEJOG = 0x34,
    CMD3G_TABLELINETO = 0x35,
    CMD3G_TABLEJUMPTO = 0x36,
    CMD3G_TABLEARCLINE = 0x37,
    CMD3G_BRANCH = 0x38,
    CMD3G_SETAIJ = 0x39,
    CMD3G_SETROT = 0x3a,
    CMD3G_SETOFFS = 0x3b,
    CMD3G_SETOFFSZ = 0x3c,
    CMD3G_SETWOBBLE = 0x3d,
    CMD3G_JUMPTO3D = 0x3e,
    CMD3G_MOVETO3D = 0x3f,
    CMD3G_LINETO3D = 0x41,
    CMD3G_TABLEWHILEIO = 0x42,
    CMD3G_BSTR0 = 0x43,
    CMD3G_BSTRN = 0x44,
    CMD3G_UDPSEND = 0x45,
    CMD3G_TABLESNAP = 0x46,
    CMD3G_EOF = 0xFF,
    INTCMD = 0x80,
    INTSUSPEND = 0x81,
    INTRESUME = 0x82,
    INTABORT = 0x83,
    INTRUNMODE = 0x85,
    INTFLASHEP = 0x87,
    INTFLASHPP = 0x88,
    INTSETIO = 0x89,
    INTUARTOPEN = 0x8a,
    INTUARTWRITE = 0x8b,
    INTEXCHLLINK = 0x8c,
    INTTESTIO = 0x8d,
    INTWAITIDLE = 0x8e,
    INTMAPTGT = 0x8f,
    INTSCANTGT = 0x90,
    INTUDPSEND = 0x91,
    INTWAITIO = 0x92,
    INTREPLY = 0x40,
    INTSTATUS = 0xc0,
    INTTGTSTATUS = 0xc1,
    INTFLASHRD = 0xc4,
    INTGTID = 0xc5,
    INTFLASHRDY = 0xc6,
    INTUARTREAD = 0xc7,
    INTSCANREAD = 0xc8,
    INTGETIP = 0xc9,
}
impl From<CMD3G_OPCODE> for u8 {
    fn from(value: CMD3G_OPCODE) -> u8 {
        value as u8
    }
}

/// A 2D position
#[derive(Debug, PartialEq)]
pub struct Position {
    x: f64,
    y: f64,
}

/// A 2D position formatted for a Newson command
pub struct RawPosition {
    x: u16,
    y: u16,
    xh: u8,
    yh: u8,
}
impl Position {
    pub fn new(x: f64, y: f64) -> Position {
        Position { x, y }
    }
    pub fn to_raw(&self) -> RawPosition {
        RawPosition {
            x: ((self.x * 1000.0).round() as i32 & 0xFFFF) as u16,
            y: ((self.y * 1000.0).round() as i32 & 0xFFFF) as u16,
            xh: ((self.x * 1000.0).round() as i32 >> 16) as u8,
            yh: ((self.y * 1000.0).round() as i32 >> 16) as u8,
        }
    }
}

/// Turns a Rhothor command into a vector of atomic Newson commands
fn build_command(command: &RhothorCommand) -> Vec<CMD3G> {
    match command {
        RhothorCommand::ListOpen(_) => vec![], //TODO
        RhothorCommand::ListClose => vec![],   //TODO
        RhothorCommand::Jump(pos) => {
            vec![CMD3G::new_movement(
                &pos.to_raw(),
                CMD3G_OPCODE::CMD3G_JUMPTO,
                TARGET,
            )]
        }
        RhothorCommand::SetIO(value, mask) => vec![CMD3G::new(
            *value,
            *mask,
            0,
            0,
            CMD3G_OPCODE::CMD3G_SETIO,
            TARGET,
        )],
        RhothorCommand::SetAnalog(value, mask) => vec![CMD3G::new(
            *value,
            *mask,
            0,
            0,
            CMD3G_OPCODE::CMD3G_SETANA,
            TARGET,
        )],
        RhothorCommand::Arc(center, bf) => {
            if bf.abs() < 0.000001 {
                return vec![CMD3G::new_movement(
                    &center.to_raw(),
                    CMD3G_OPCODE::CMD3G_MOVETO,
                    TARGET,
                )];
            }
            vec![
                CMD3G::new_movement(&center.to_raw(), CMD3G_OPCODE::CMD3G_ARCLINE, TARGET),
                CMD3G::new(
                    ((bf.to_bits() & 0xFFFF0000) >> 16) as u16,
                    0,
                    0,
                    0,
                    CMD3G_OPCODE::CMD3G_PARAMS,
                    TARGET,
                ),
            ]
        }
        RhothorCommand::Circle(center, angle) => {
            vec![
                CMD3G::new_movement(&center.to_raw(), CMD3G_OPCODE::CMD3G_CIRCLE, TARGET),
                CMD3G::new(
                    (angle.to_bits() & 0xFFFF) as u16,
                    ((angle.to_bits() & 0xFFFF0000) >> 16) as u16,
                    0,
                    0,
                    CMD3G_OPCODE::CMD3G_PARAMS,
                    TARGET,
                ),
            ]
        }
        RhothorCommand::Line(pos) => {
            vec![CMD3G::new_movement(
                &pos.to_raw(),
                CMD3G_OPCODE::CMD3G_LINETO,
                TARGET,
            )]
        }
        RhothorCommand::WaitIO => vec![CMD3G::new(0, 0, 0, 0, CMD3G_OPCODE::CMD3G_NOP, 0)],
        RhothorCommand::Move(pos) => {
            vec![CMD3G::new_movement(
                &pos.to_raw(),
                CMD3G_OPCODE::CMD3G_MOVETO,
                TARGET,
            )]
        }
        RhothorCommand::SetSpeed(speed) => vec![CMD3G::new(
            (speed.to_bits() & 0xFFFF) as u16,
            ((speed.to_bits() & 0xFFFF0000) >> 16) as u16,
            0,
            0,
            CMD3G_OPCODE::CMD3G_SPEED,
            TARGET,
        )],
        RhothorCommand::SetJumpSpeed(speed) => vec![CMD3G::new(
            (speed.to_bits() & 0xFFFF) as u16,
            ((speed.to_bits() & 0xFFFF0000) >> 16) as u16,
            0,
            0,
            CMD3G_OPCODE::CMD3G_JUMPSPEED,
            TARGET,
        )],
        RhothorCommand::Sleep(time) => vec![CMD3G::new(
            *time,
            0,
            0,
            0,
            CMD3G_OPCODE::CMD3G_SLEEP,
            TARGET,
        )],
        RhothorCommand::Burst(time) => vec![CMD3G::new(
            *time,
            0,
            0,
            0,
            CMD3G_OPCODE::CMD3G_BURST,
            TARGET,
        )],
        RhothorCommand::SetLaser(on) => vec![CMD3G::new(
            *on as u16,
            0,
            0,
            0,
            CMD3G_OPCODE::CMD3G_SETLIDLE,
            TARGET,
        )],
        RhothorCommand::SetLaserTimes(on_delay, off_delay) => vec![CMD3G::new(
            *on_delay,
            *off_delay,
            0,
            0,
            CMD3G_OPCODE::CMD3G_SETDELAYS,
            TARGET,
        )],
        RhothorCommand::WhileIO => vec![CMD3G::new(0, 0, 0, 0, CMD3G_OPCODE::CMD3G_NOP, 0)],
        RhothorCommand::DoWhile => vec![CMD3G::new(0, 0, 0, 0, CMD3G_OPCODE::CMD3G_NOP, 0)],
        RhothorCommand::SetLoop => vec![CMD3G::new(0, 0, 0, 0, CMD3G_OPCODE::CMD3G_NOP, 0)],
        RhothorCommand::DoLoop => vec![CMD3G::new(0, 0, 0, 0, CMD3G_OPCODE::CMD3G_NOP, 0)],
        RhothorCommand::SetTarget(_) => vec![], //TODO
        _ => vec![],
    }
}

/// Turns a vector of Rhothor commands into a vector of atomic Newson commands
pub fn build_commandlist(command_vec: &[RhothorCommand]) -> Vec<CMD3G> {
    command_vec
        .iter()
        .flat_map(|cmd| build_command(cmd))
        .collect::<Vec<CMD3G>>()
}

/// Constructs a request to get status
pub fn get_status() -> Vec<CMD3G> {
    vec![CMD3G::new(0, 0x2C, 0, 0, CMD3G_OPCODE::INTSTATUS, TARGET)]
}

/// Constructs a request to get target ID
pub fn get_target_id() -> Vec<CMD3G> {
    vec![CMD3G::new(0, 0, 0, 0, CMD3G_OPCODE::INTGTID, 0)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_commands() {
        struct TestCase {
            rhothor_cmd: RhothorCommand,
            cmd3g_cmd: Vec<CMD3G>,
        }
        let test_cases = vec![
            TestCase {
                rhothor_cmd: RhothorCommand::Jump(Position::new(1.2, 3.4)),
                cmd3g_cmd: vec![CMD3G::new(
                    1200,
                    3400,
                    0,
                    0,
                    CMD3G_OPCODE::CMD3G_JUMPTO,
                    TARGET,
                )],
            },
            TestCase {
                rhothor_cmd: RhothorCommand::Move(Position::new(-1.2, -3.4)),
                cmd3g_cmd: vec![CMD3G::new(
                    64336,
                    62136,
                    255,
                    255,
                    CMD3G_OPCODE::CMD3G_MOVETO,
                    TARGET,
                )],
            },
            TestCase {
                rhothor_cmd: RhothorCommand::Line(Position::new(-0.2, 6.0)),
                cmd3g_cmd: vec![CMD3G::new(
                    65336,
                    6000,
                    255,
                    0,
                    CMD3G_OPCODE::CMD3G_LINETO,
                    TARGET,
                )],
            },
            TestCase {
                rhothor_cmd: RhothorCommand::SetSpeed(1200.0),
                cmd3g_cmd: vec![CMD3G::new(
                    0,
                    0x4496,
                    0,
                    0,
                    CMD3G_OPCODE::CMD3G_SPEED,
                    TARGET,
                )],
            },
            TestCase {
                rhothor_cmd: RhothorCommand::SetJumpSpeed(600.0),
                cmd3g_cmd: vec![CMD3G::new(
                    0,
                    0x4416,
                    0,
                    0,
                    CMD3G_OPCODE::CMD3G_JUMPSPEED,
                    TARGET,
                )],
            },
            TestCase {
                rhothor_cmd: RhothorCommand::Circle(Position::new(-0.5, 0.5), 360.0),
                cmd3g_cmd: vec![
                    CMD3G::new(0xFE0C, 500, 0xFF, 0, CMD3G_OPCODE::CMD3G_CIRCLE, TARGET),
                    CMD3G::new(0, 0x43B4, 0, 0, CMD3G_OPCODE::CMD3G_PARAMS, TARGET),
                ],
            },
            TestCase {
                rhothor_cmd: RhothorCommand::Arc(Position::new(-0.5, 0.5), 360.0),
                cmd3g_cmd: vec![
                    CMD3G::new(0xFE0C, 500, 0xFF, 0, CMD3G_OPCODE::CMD3G_ARCLINE, TARGET),
                    CMD3G::new(0x43B4, 0, 0, 0, CMD3G_OPCODE::CMD3G_PARAMS, TARGET),
                ],
            },
            TestCase {
                rhothor_cmd: RhothorCommand::Arc(Position::new(-0.5, 0.5), 0.0000001),
                cmd3g_cmd: vec![CMD3G::new(
                    0xFE0C,
                    500,
                    0xFF,
                    0,
                    CMD3G_OPCODE::CMD3G_MOVETO,
                    TARGET,
                )],
            },
            TestCase {
                rhothor_cmd: RhothorCommand::SetIO(512, 1024),
                cmd3g_cmd: vec![CMD3G::new(
                    512,
                    1024,
                    0,
                    0,
                    CMD3G_OPCODE::CMD3G_SETIO,
                    TARGET,
                )],
            },
            TestCase {
                rhothor_cmd: RhothorCommand::SetAnalog(1024, 512),
                cmd3g_cmd: vec![CMD3G::new(
                    1024,
                    512,
                    0,
                    0,
                    CMD3G_OPCODE::CMD3G_SETANA,
                    TARGET,
                )],
            },
            TestCase {
                rhothor_cmd: RhothorCommand::Sleep(500),
                cmd3g_cmd: vec![CMD3G::new(500, 0, 0, 0, CMD3G_OPCODE::CMD3G_SLEEP, TARGET)],
            },
        ];
        for test in test_cases {
            let got = build_command(&test.rhothor_cmd);
            assert_eq!(got.len(), test.cmd3g_cmd.len());
            assert!(
                got.iter().zip(test.cmd3g_cmd.iter()).all(|(a, b)| a == b),
                "got {:?}, wanted {:?}",
                got,
                test.cmd3g_cmd
            );
        }
    }
}
