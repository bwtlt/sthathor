use regex::Regex;
use serde::Deserialize;
use serde::Serialize;
use std::io::prelude::*;
use std::net::TcpStream;
use std::str::FromStr;

#[derive(Debug, Serialize, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub enum RhothorCommand {
    None,
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
impl FromStr for RhothorCommand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"(?P<command>[A-z]+)\((?P<args>(.*))\)").unwrap();
        let caps = re.captures(s).unwrap();
        let command = match caps.name("command").unwrap().as_str() {
            "rtJumpTo" => {
                let pos = caps
                    .name("args")
                    .unwrap()
                    .as_str()
                    .split(',')
                    .collect::<Vec<&str>>();
                if pos.len() != 2 {
                    return Err(());
                }
                RhothorCommand::Jump(Position::new(
                    pos.get(0).unwrap().parse::<f64>().unwrap(),
                    pos.get(1).unwrap().parse::<f64>().unwrap(),
                ))
            }
            _ => return Err(()),
        };

        Ok(command)
    }
}

#[derive(Debug, PartialEq)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_jump_command() {
        let got = build_commandlist(&vec![RhothorCommand::Jump(Position::new(0.0, 0.0))]);
        let want = vec![CMD3G::new(0, 0, 0, 0, 0x04, 1)];
        assert_eq!(got.len(), want.len());
        assert!(got.iter().zip(want.iter()).all(|(a, b)| a == b));
    }

    #[test]
    fn parse_jump() {
        let got = RhothorCommand::from_str("rtJumpTo(1234.5,777.42)").unwrap();
        let want = RhothorCommand::Jump(Position::new(1234.5, 777.42));
        assert_eq!(got, want);

        let got = RhothorCommand::from_str("rtJumpTo(-1234,777)").unwrap();
        let want = RhothorCommand::Jump(Position::new(-1234.0, 777.0));
        assert_eq!(got, want);

        let got = RhothorCommand::from_str("rtJumpTo(1234.5,777.42,4.8)");
        assert_eq!(got, Err(()));
    }
}
