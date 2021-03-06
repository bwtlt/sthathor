use crate::commands::Position;
use crate::AppError;
use regex::Regex;
use std::str::FromStr;

/// Representation of a scanner command with its parameters
#[derive(Debug, PartialEq)]
pub enum ScannerCommand {
    None,
    ListOpen(u32),
    ListClose,
    Jump(Position),
    SetIO(u16, u16),
    SetAnalog(u16, u16),
    Arc(Position, f32),
    Circle(Position, f32),
    CircleMove(Position, f32),
    Line(Position),
    WaitIO,
    Move(Position),
    SetSpeed(f32),
    SetJumpSpeed(f32),
    Sleep(u16),
    Burst(u16),
    SetLaser(bool),
    SetLaserTimes(u16, u16),
    SetTarget(u32),
    WhileIO,
    DoWhile,
    SetLoop,
    DoLoop,
}
/// Parse a scanner command string (e.g. "rtMoveTo(3.0, 4.5)") into the corresponding enum
impl FromStr for ScannerCommand {
    type Err = AppError;

    fn from_str(s: &str) -> Result<Self, AppError> {
        fn parse_position(s: &str) -> Result<Position, AppError> {
            let pos = s.split(',').collect::<Vec<&str>>();
            if pos.len() != 2 {
                return Err(AppError::ParseError);
            }
            match (
                pos.get(0).unwrap().parse::<f64>(),
                pos.get(1).unwrap().parse::<f64>(),
            ) {
                (Ok(x), Ok(y)) => Ok(Position::new(x, y)),
                _ => Err(AppError::ParseError),
            }
        }

        fn parse_f32(s: &str) -> Result<f32, AppError> {
            match s.parse::<f32>() {
                Ok(val) => Ok(val),
                _ => Err(AppError::ParseError),
            }
        }

        fn parse_int(s: &str) -> Result<u32, AppError> {
            match s.parse::<u32>() {
                Ok(val) => Ok(val),
                _ => Err(AppError::ParseError),
            }
        }

        let re = Regex::new(r"(?P<command>[A-z]+)\((?P<args>(.*))\)").unwrap();
        let caps = re.captures(s).unwrap();
        let args = caps.name("args").unwrap().as_str();
        let command = match caps.name("command").unwrap().as_str() {
            "rtListOpen" => ScannerCommand::ListOpen(parse_int(args)?),
            "rtListClose" => ScannerCommand::ListClose,
            "rtJumpTo" => ScannerCommand::Jump(parse_position(args)?),
            "rtMoveTo" => ScannerCommand::Move(parse_position(args)?),
            "rtLineTo" => ScannerCommand::Line(parse_position(args)?),
            "rtSetSpeed" => ScannerCommand::SetSpeed(parse_f32(args)?),
            "rtSetJumpSpeed" => ScannerCommand::SetJumpSpeed(parse_f32(args)?),
            "rtSetTarget" => ScannerCommand::SetTarget(parse_int(args)?),
            _ => return Err(AppError::ParseError),
        };

        Ok(command)
    }
}

/// Parse a scanner script line, ditching comments and empty lines
pub fn parse_line(s: &str) -> Result<Option<ScannerCommand>, AppError> {
    let s = s.trim();
    if s.starts_with("//") || s.is_empty() {
        return Ok(None);
    }
    Ok(Some(ScannerCommand::from_str(s)?))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_command() {
        struct TestCase {
            got: Result<ScannerCommand, AppError>,
            want: Result<ScannerCommand, AppError>,
        }
        let test_cases = vec![
            TestCase {
                got: Ok(ScannerCommand::from_str("rtJumpTo(1234.5,777.42)").unwrap()),
                want: Ok(ScannerCommand::Jump(Position::new(1234.5, 777.42))),
            },
            TestCase {
                got: Ok(ScannerCommand::from_str("rtJumpTo(-1234,777)").unwrap()),
                want: Ok(ScannerCommand::Jump(Position::new(-1234.0, 777.0))),
            },
            TestCase {
                got: Ok(ScannerCommand::from_str("rtSetSpeed(1200)").unwrap()),
                want: Ok(ScannerCommand::SetSpeed(1200.0)),
            },
            TestCase {
                got: ScannerCommand::from_str("rtSetSpeed()"),
                want: Err(AppError::ParseError),
            },
            TestCase {
                // two many arguments
                got: ScannerCommand::from_str("rtSetSpeed(1.2, 0)"),
                want: Err(AppError::ParseError),
            },
            TestCase {
                // two many arguments
                got: ScannerCommand::from_str("rtJumpTo(1234.5,777.42,4.8)"),
                want: Err(AppError::ParseError),
            },
            TestCase {
                // syntax error
                got: ScannerCommand::from_str("rtJumpTo(1234.O,4.8)"),
                want: Err(AppError::ParseError),
            },
            TestCase {
                got: Ok(ScannerCommand::from_str("rtMoveTo(1.2,7.77)").unwrap()),
                want: Ok(ScannerCommand::Move(Position::new(1.2, 7.77))),
            },
        ];

        for test in test_cases {
            assert_eq!(test.got, test.want);
        }
    }

    #[test]
    fn line_parsing() {
        assert!(parse_line("// This is a comment").unwrap().is_none());
        assert!(parse_line("   ").unwrap().is_none());
        assert_eq!(
            parse_line("rtMoveTo(1.2,3.4)").unwrap().unwrap(),
            ScannerCommand::Move(Position::new(1.2, 3.4))
        );
    }
}
