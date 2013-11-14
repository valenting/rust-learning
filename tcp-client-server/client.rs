use std::cell::Cell;
use std::rt::io::{Writer, Listener, Acceptor};
use std::rt::io::net::tcp::TcpStream;
use std::rt::io::net::ip::{SocketAddr, Ipv4Addr};
use std::rt::io;
use std::rt::io::buffered::BufferedReader;
use std::str;
use std::str::not_utf8;

fn main() {
	info!("Starting client");

	let o_stream = TcpStream::connect( SocketAddr{ip: Ipv4Addr(127, 0, 0, 1), port: 9123});

	let mut connection = match o_stream {
		Some(s) => s,
		None => fail!("No connection")
	};

	loop {
		let line = BufferedReader::new(io::stdin()).read_line();
        match line {
        	Some(s) => connection.write(s.as_bytes()),
        	_ => fail!("error")
        }
    }
}