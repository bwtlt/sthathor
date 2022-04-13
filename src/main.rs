use clap::{crate_version, App, Arg};
use std::error::Error;
use std::net::IpAddr;
use std::net::Ipv4Addr;
use std::net::SocketAddr;
use std::net::{Shutdown, TcpStream};
use std::time::Duration;

use rusthor::commands;
use rusthor::*;

fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("rusthor")
        .version(crate_version!())
        .author("bwatelet")
        .about("Rhothor clone in Rust.")
        .arg(
            Arg::with_name("IP_ADDRESS")
                .help("Scanners IP address, e.g. 192.168.0.6")
                .index(1)
                .required(true),
        )
        .arg(
            Arg::with_name("COMMANDS_FILE")
                .short("f")
                .long("file")
                .value_name("FILE")
                .help("Load a Rhothor commands file")
                .takes_value(true),
        )
        .get_matches();

    let port = 10002;
    let ip_address = match matches.value_of("IP_ADDRESS").unwrap().parse::<Ipv4Addr>() {
        Ok(addr) => addr,
        Err(_) => return Err(From::from("Failed to parse IP address")),
    };

    let socket_address = SocketAddr::new(IpAddr::V4(ip_address), port);
    let mut stream = TcpStream::connect_timeout(&socket_address, Duration::new(5, 0))?;

    if matches.is_present("COMMANDS_FILE") {
        let command_list = commands::build_commandlist(&parse_command_file(
            matches.value_of("COMMANDS_FILE").unwrap(),
        )?);
        send(&command_list, &mut stream)?;
    }

    stream.shutdown(Shutdown::Both)?;

    Ok(())
}
