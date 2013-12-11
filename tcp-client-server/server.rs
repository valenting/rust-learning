use std::io::{Listener, Acceptor};
use std::io::net::tcp::TcpListener;
use std::io::net::ip::{SocketAddr, Ipv4Addr};
use std::str;
use std::str::not_utf8;
use std::cell::Cell;

fn main() {
	// To view debug messages run server with the following command
	// RUST_LOG=server=3 ./server
	info!("Starting Server");

	let o_listener =  TcpListener::bind(
		SocketAddr { ip: Ipv4Addr(127, 0, 0, 1), port: 9123}).listen();

	let mut acceptor = o_listener;
	//		SAME AS:
	//
	// let mut o_acceptor = match o_listener {
	// 	Some(listener) => listener,
	// 	None => fail!("Failed to open listener.")
	// };

	loop {
		let o_stream = acceptor.accept();
		let stream = o_stream.unwrap();
		// We need to wrap stream in a cell, to pass it to the new task
		let cell = Cell::new(stream);
		do spawn {
			let mut stream = cell.take();
			let name = stream.peer_name().unwrap();
			loop {
				let mut buf: [u8, ..100] = [0, ..100];
				let count = stream.read(buf);
				match count {
					Some(x) => println!("Received {} bytes on {:s}", x, name.to_str()),
					None => { println("EOF"); break; }
				}
				do not_utf8::cond.trap(|_| ~"Error").inside {
					// Displays the entered string, or "Error" if the socket is remotely closed
					println(str::replace(str::from_utf8(buf),"\n",""));
				}
			}
		}
	}
	
	 
}
