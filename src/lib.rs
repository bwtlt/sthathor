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
pub use crate::parsing::RhothorCommand;

pub fn get_status(stream: &mut TcpStream) -> std::io::Result<TgtStatus> {
    let reply = exchange(&commands::get_status(), stream).unwrap();
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

pub fn parse_command_file(path: &str) -> Result<Vec<RhothorCommand>, AppError> {
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
            RhothorCommand::SetTarget(1),
            RhothorCommand::ListOpen(5),
            RhothorCommand::SetJumpSpeed(600.0),
            RhothorCommand::SetSpeed(600.0),
            RhothorCommand::Jump(Position::new(0.0, -3.0)),
            RhothorCommand::ListClose,
        ];
        commands
            .iter()
            .zip(want.iter())
            .for_each(|(got, want)| assert_eq!(got, want));
    }
}
