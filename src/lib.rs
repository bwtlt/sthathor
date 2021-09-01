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

pub enum TrajectoryType {
    Jump,
    Move,
}
pub fn build_trajectory(trajectory: TrajectoryType) -> Vec<CMD3G> {
    let mut command_buffer = vec![CMD3G::new(0, 0, 0, 0, 0x4A, 1)];
    let speed: u32 = 0x43960000;
    command_buffer.push(CMD3G::new(
        (speed & 0xFFFF) as u16,
        ((speed & 0xFFFF0000) >> 16) as u16,
        0,
        0,
        0x0b,
        1,
    ));
    match trajectory {
        TrajectoryType::Jump => {
            let position = 2184;
            command_buffer.push(CMD3G::new(position, position, 0, 0, 0x04, 1));
        }
        TrajectoryType::Move => (),
    }
    command_buffer
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
