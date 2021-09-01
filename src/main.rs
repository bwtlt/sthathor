use serde::Deserialize;
use serde::Serialize;
use std::io::prelude::*;
use std::net::TcpStream;
use std::str::Bytes;

#[derive(Serialize)]
struct CMD3G {
    x: u16,
    y: u16,
    xh: u8,
    yh: u8,
    op_code: u8,
    target: u8,
}
impl CMD3G {
    fn new(x: u16, y: u16, xh: u8, yh: u8, op_code: u8, target: u8) -> CMD3G {
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
struct TgtStatus {
    inputs: u16,
    outputs: u16,
    me: u8,
    ReturnValue: u8,
    h_counter: u16,
    c_Analog: [u8; 4],
    d_OtfX: i32,
    d_OtfY: i32,
    d_TableX: i32,
    d_TableY: i32,
    d_TableZ: i32,
    d_DeflX: i32,
    d_DeflY: i32,
    d_DeflZ: i32,
}

enum TrajectoryType {
    Jump,
}
fn build_trajectory(trajectory: TrajectoryType) -> Vec<u8> {
    let mut command_buffer = Vec::new();
    command_buffer.append(&mut bincode::serialize(&CMD3G::new(0, 0, 0, 0, 0x4A, 1)).unwrap());
    let speed: u32 = 0x43960000;
    command_buffer.append(
        &mut bincode::serialize(&CMD3G::new(
            (speed & 0xFFFF) as u16,
            ((speed & 0xFFFF0000) >> 16) as u16,
            0,
            0,
            0x0b,
            1,
        ))
        .unwrap(),
    );
    match trajectory {
        TrajectoryType::Jump => {
            let position = 2184;
            command_buffer.append(
                &mut bincode::serialize(&CMD3G::new(position, position, 0, 0, 0x04, 1)).unwrap(),
            );
        }
        _ => (),
    }
    command_buffer
}

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("192.168.0.6:10002")?;

    let query = CMD3G::new(0, 0, 0, 0, 0xC5, 0);
    let bytes = bincode::serialize(&query).unwrap();
    stream.write_all(&bytes)?;
    let mut reply = [0 as u8; 64];
    stream.read(&mut reply)?;
    println!("reply: {:?}", reply);

    let query = CMD3G::new(0, 0x2C, 0, 0, 0xC1, 1);
    let bytes = bincode::serialize(&query).unwrap();
    stream.write_all(&bytes)?;
    let mut reply = [0 as u8; 44];
    stream.read(&mut reply)?;
    let status: TgtStatus = bincode::deserialize(&reply).unwrap();
    println!(
        "scanners are at ({} um, {} um)",
        status.d_DeflX, status.d_DeflY
    );

    stream.write_all(&build_trajectory(TrajectoryType::Jump))?;
    let query = CMD3G::new(0, 0x2C, 0, 0, 0xC1, 1);
    let bytes = bincode::serialize(&query).unwrap();
    stream.write_all(&bytes)?;
    let mut reply = [0 as u8; 44];
    stream.read(&mut reply)?;
    let status: TgtStatus = bincode::deserialize(&reply).unwrap();
    println!(
        "scanners are at ({} um, {} um)",
        status.d_DeflX, status.d_DeflY
    );

    Ok(())
} // the stream is closed here
