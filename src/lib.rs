use bincode::Options;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufRead, BufReader};
use std::net::TcpStream;

#[derive(Debug, PartialEq)]
pub enum AppError {
    ParseError,
    FileError,
}

impl From<std::io::Error> for AppError {
    fn from(_e: std::io::Error) -> AppError {
        AppError::FileError
    }
}
impl std::error::Error for AppError {}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::ParseError => write!(f, "Parse Error"),
            AppError::FileError => write!(f, "File Error"),
        }
    }
}

pub mod commands;
mod parsing;
pub use crate::commands::TgtStatus;
pub use crate::commands::CMD3G;
pub use crate::parsing::ScannerCommand;

/// Queries Newson target for status (positions, inputs/outputs state, etc.)
pub fn get_status(stream: &mut TcpStream) -> std::io::Result<TgtStatus> {
    let reply = exchange(&commands::get_status(), stream).unwrap();
    let status: TgtStatus = bincode::deserialize(&reply).unwrap();
    Ok(status)
}

/// Sends a query and reads a reply
pub fn exchange(queries: &[CMD3G], stream: &mut TcpStream) -> std::io::Result<Vec<u8>> {
    send(queries, stream)?;
    let mut reply = [0_u8; 128];
    let n = stream.read(&mut reply)?;
    Ok(reply[..n].to_vec())
}

/// Sends commands to Newson target
pub fn send(commands: &[CMD3G], stream: &mut TcpStream) -> std::io::Result<()> {
    let mut buffer = serialize_commands(commands);
    let remainder = buffer.len() % 512;
    buffer.resize(buffer.len() + 512 - remainder, 0);
    stream.write_all(&buffer)?;
    Ok(())
}

/// Turns a vector of commands into a vector of bytes to be sent
fn serialize_commands(commands: &[CMD3G]) -> Vec<u8> {
    let mut buffer = Vec::new();
    commands.iter().for_each(|q| {
        buffer.append(
            &mut bincode::DefaultOptions::new()
                .with_fixint_encoding()
                .serialize(q)
                .unwrap(),
        )
    });
    buffer
}

/// Creates a vector of commands from a text file
pub fn parse_command_file(path: &str) -> Result<Vec<ScannerCommand>, AppError> {
    let file = File::open(path)?;
    let mut commands = Vec::new();
    for line in BufReader::new(file).lines() {
        if let Some(val) = parsing::parse_line(line.unwrap().as_str())? {
            commands.push(val)
        }
    }
    Ok(commands)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::Position;

    #[test]
    fn parse_file() {
        let commands = parse_command_file("resources/commands.txt").unwrap();
        let want = vec![
            ScannerCommand::ListOpen(4),
            ScannerCommand::SetJumpSpeed(1200.0),
            ScannerCommand::SetSpeed(1200.0),
            ScannerCommand::Jump(Position::new(-6.0, -6.0)),
            ScannerCommand::Line(Position::new(6.0, 6.0)),
            ScannerCommand::ListClose,
        ];
        commands
            .iter()
            .zip(want.iter())
            .for_each(|(got, want)| assert_eq!(got, want));
    }

    #[test]
    fn test_serialize() {
        let q = vec![CMD3G::new(0, 0, 0, 0, commands::CMD3G_OPCODE::INTGTID, 0)];
        let want = vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xC5, 0x00];
        let buffer = serialize_commands(&q);
        assert_eq!(buffer.len(), want.len());
        buffer
            .iter()
            .zip(want.iter())
            .for_each(|(got, want)| assert_eq!(got, want));
    }
}
