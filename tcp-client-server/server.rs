use std::cell::Cell;
use std::rt::io::{Writer, Listener, Acceptor};
use std::rt::io::net::tcp::TcpListener;
use std::rt::io::net::ip::{SocketAddr, Ipv4Addr};
use std::str;
use std::str::not_utf8;

fn main() {
	info!("Starting Server");

	let o_listener =  TcpListener::bind(
		SocketAddr { ip: Ipv4Addr(127, 0, 0, 1), port: 9123}).listen();

	let mut o_acceptor = match o_listener {
    	Some(listener) => listener,
    	None => fail!("Failed to open listener.")
	};

	let o_stream = o_acceptor.accept();

	let mut stream = match o_stream {
		Some(stream) => stream,
		None => fail!("No stream")
	};

	let mut buf: [u8, ..100] = [0, ..100];

	loop {
		stream.read(buf);
		do not_utf8::cond.trap(|_| ~"Error").inside {
			// Displays the entered string, or "Error" if the socket is remotely closed
			println(str::from_utf8(buf));
		}
	} 
}