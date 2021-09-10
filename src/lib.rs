use serde::Deserialize;
use serde::Serialize;
use std::io::prelude::*;
use std::net::TcpStream;

#[derive(Serialize)]
pub struct CMD3G {
    x: u16,
    y: u16,
    xh: u8,
    yh: u8,
    op_code: u8,
    target: u8,
}
impl CMD3G {
    pub fn new(x: u16, y: u16, xh: u8, yh: u8, op_code: u8, target: u8) -> CMD3G {
        CMD3G {
            x,
            y,
            xh,
            yh,
            op_code,
            target,
        }
    }
}

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

pub enum RhothorCommand {
    ListOpen,
    ListClose,
    Jump(Position),
    SetIO,
    SetAnalog,
    Arc,
    Line(Position),
    WaitIO,
    Move(Position),
    SetSpeed(f64),
    SetJumpSpeed(f64),
    Sleep,
    Burst,
    SetLaser,
    SetLaserTumes,
    SetTarget,
    WhileIO,
    DoWhile,
    SetLoop,
    DoLoop,
}

pub struct Position {
    x: f64,
    y: f64,
}
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
            x: ((self.x * 1000.0).round() as u32 & 0xFFFF) as u16,
            y: ((self.y * 1000.0).round() as u32 & 0xFFFF) as u16,
            xh: ((self.x * 1000.0).round() as u32 >> 16) as u8,
            yh: ((self.y * 1000.0).round() as u32 >> 16) as u8,
        }
    }
}

pub fn build_commandlist(command_vec: &[RhothorCommand]) -> Vec<CMD3G> {
    command_vec
        .iter()
        .map(|cmd| match cmd {
            RhothorCommand::SetSpeed(speed) => CMD3G::new(
                (speed.to_bits() & 0xFFFF) as u16,
                ((speed.to_bits() & 0xFFFF0000) >> 16) as u16,
                0,
                0,
                0x0b,
                1,
            ),
            RhothorCommand::Jump(pos) => {
                let pos = pos.to_raw();
                CMD3G::new(pos.x, pos.y, pos.xh, pos.yh, 0x04, 1)
            }
            _ => CMD3G::new(0, 0, 0, 0, 0, 0),
        })
        .collect::<Vec<CMD3G>>()
}

pub fn get_status(stream: &mut TcpStream) -> std::io::Result<TgtStatus> {
    let query = vec![CMD3G::new(0, 0x2C, 0, 0, 0xC1, 1)];
    let reply = exchange(&query, stream).unwrap();
    let status: TgtStatus = bincode::deserialize(&reply).unwrap();
    Ok(status)
}

pub fn exchange(queries: &[CMD3G], stream: &mut TcpStream) -> std::io::Result<Vec<u8>> {
    let mut buffer = Vec::new();
    queries
        .iter()
        .for_each(|q| buffer.append(&mut bincode::serialize(q).unwrap()));
    stream.write_all(&buffer)?;
    let mut reply = [0_u8; 128];
    let n = stream.read(&mut reply)?;
    Ok(reply[..n].to_vec())
}
