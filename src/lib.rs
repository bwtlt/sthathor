use std::fmt;
use std::io::prelude::*;
use std::net::TcpStream;

#[derive(Debug, PartialEq)]
pub enum AppError {
    ParseError,
}

impl std::error::Error for AppError {}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::ParseError => write!(f, "Parse Error"),
        }
    }
}

pub mod commands;
mod parsing;
pub use crate::commands::TgtStatus;
pub use crate::commands::CMD3G;

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
