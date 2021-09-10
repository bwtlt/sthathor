use clap::{crate_version, App, Arg};
use std::error::Error;
use std::net::IpAddr;
use std::net::Ipv4Addr;
use std::net::SocketAddr;
use std::net::TcpStream;
use std::time::Duration;

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
        .get_matches();

    let port = 10002;
    let ip_address = match matches.value_of("IP_ADDRESS").unwrap().parse::<Ipv4Addr>() {
        Ok(addr) => addr,
        Err(_) => return Err(From::from("Failed to parse IP address")),
    };

    let socket_address = SocketAddr::new(IpAddr::V4(ip_address), port);
    let mut stream = TcpStream::connect_timeout(&socket_address, Duration::new(5, 0))?;

    let query = vec![CMD3G::new(0, 0, 0, 0, 0xC5, 0)];
    let reply = exchange(&query, &mut stream)?;
    println!("reply: {:?}", reply);

    let command = RhothorCommand::Jump(Position::new(0.0, 0.0));
    let commands = vec![command];
    let reply = exchange(&build_commandlist(&commands), &mut stream)?;
    println!("reply: {:?}", reply);

    Ok(())
}
