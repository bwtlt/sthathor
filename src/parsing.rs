use crate::AppError;
use crate::Position;
use regex::Regex;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum RhothorCommand {
    None,
    ListOpen(u32),
    ListClose,
    Jump(Position),
    SetIO,
    SetAnalog,
    Arc,
    Circle(Position, f32),
    CircleMove(Position, f32),
    Line(Position),
    WaitIO,
    Move(Position),
    SetSpeed(f32),
    SetJumpSpeed(f32),
    Sleep,
    Burst,
    SetLaser,
    SetLaserTimes,
    SetTarget,
    WhileIO,
    DoWhile,
    SetLoop,
    DoLoop,
}
impl FromStr for RhothorCommand {
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
            "rtListOpen" => RhothorCommand::ListOpen(parse_int(args)?),
            "rtListClose" => RhothorCommand::ListClose,
            "rtJumpTo" => RhothorCommand::Jump(parse_position(args)?),
            "rtMoveTo" => RhothorCommand::Move(parse_position(args)?),
            "rtLineTo" => RhothorCommand::Line(parse_position(args)?),
            "rtSetSpeed" => RhothorCommand::SetSpeed(parse_f32(args)?),
            _ => return Err(AppError::ParseError),
        };

        Ok(command)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_jump() {
        struct TestCase {
            got: Result<RhothorCommand, AppError>,
            want: Result<RhothorCommand, AppError>,
        }
        let test_cases = vec![
            TestCase {
                got: Ok(RhothorCommand::from_str("rtJumpTo(1234.5,777.42)").unwrap()),
                want: Ok(RhothorCommand::Jump(Position::new(1234.5, 777.42))),
            },
            TestCase {
                got: Ok(RhothorCommand::from_str("rtJumpTo(-1234,777)").unwrap()),
                want: Ok(RhothorCommand::Jump(Position::new(-1234.0, 777.0))),
            },
            TestCase {
                got: Ok(RhothorCommand::from_str("rtSetSpeed(1200)").unwrap()),
                want: Ok(RhothorCommand::SetSpeed(1200.0)),
            },
            TestCase {
                got: RhothorCommand::from_str("rtSetSpeed()"),
                want: Err(AppError::ParseError),
            },
            TestCase {
                // two many arguments
                got: RhothorCommand::from_str("rtSetSpeed(1.2, 0)"),
                want: Err(AppError::ParseError),
            },
            TestCase {
                // two many arguments
                got: RhothorCommand::from_str("rtJumpTo(1234.5,777.42,4.8)"),
                want: Err(AppError::ParseError),
            },
            TestCase {
                // syntax error
                got: RhothorCommand::from_str("rtJumpTo(1234.O,4.8)"),
                want: Err(AppError::ParseError),
            },
            TestCase {
                got: Ok(RhothorCommand::from_str("rtMoveTo(1.2,7.77)").unwrap()),
                want: Ok(RhothorCommand::Move(Position::new(1.2, 7.77))),
            },
        ];

        for test in test_cases {
            assert_eq!(test.got, test.want);
        }
    }
}
