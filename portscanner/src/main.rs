/// portscanner.rs, just another network information gathering tool
/// written in rust, simple smalll and blazingly fast.. microseconds
/// to seconds (< 2) for a complete TCP port scan of a system.
///
///  NOT FINAL
///
///
/// Author: ElCodigoDominicano
///

use std::net::{
    IpAddr,
    Ipv4Addr,
    SocketAddr,
    TcpStream,
};
use std::env::consts::OS;
use std::process::exit;
use std::time::{Instant, Duration};

pub fn port_scanner() -> Box<Vec<u16>> {
    let mut vec_of_open_ports:  Box<Vec<u16>> = Box::new(Vec::new());

    let mut socket: SocketAddr = SocketAddr::new(
        IpAddr::V4(
            Ipv4Addr::new(127, 0, 0, 1)), 1);

    for port in 2..65535 {
        if OS == "linux" {
            if let Ok(_stream) = TcpStream::connect(&socket) {
                vec_of_open_ports.push(socket.port().to_owned());
            }
        } else if OS == "windows" {
            if let Ok(_stream) = TcpStream::connect_timeout(&socket, Duration::from_nanos(1)) {
                vec_of_open_ports.push(socket.port().to_owned());
            }
        } else {
            println!("My apologies, but {:?} hasn't been implemented yet.", OS.to_string());
            exit(0);
        }
        socket.set_port(port);
    }
    vec_of_open_ports
}

pub fn timer() {
    let now = Instant::now();

    let open_ports = port_scanner();

    let elapsed_time = now.elapsed();

    println!(
        "Found {:? } ports open..\n\
        Array of ports found open.. {:?}\n\
        Scan took {:.2?} to complete..",
        open_ports.len(),
        open_ports,
        elapsed_time
    );
}

fn main () {
    timer();
}
