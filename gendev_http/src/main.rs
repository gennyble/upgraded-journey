use std::io::{Read, Write};
use std::net::{TcpStream, ToSocketAddrs};

fn main() {
	let genuine_addrs = "genbyte.dev:80"
		.to_socket_addrs()
		.expect("Failed to parse hostname");

	let mut genuine_connection: Option<TcpStream> = None;
	for addr in genuine_addrs {
		if let Ok(val) = TcpStream::connect(addr) {
			genuine_connection = Some(val);
			break;
		}
	}

	let mut tcps = if let Some(val) = genuine_connection {
		val
	} else {
		panic!("Failed to connect to genbyte.dev");
	};

	tcps.write(b"GET / HTTP/1.0\r\n\r\n")
		.expect("Something went wrong trying to write GET request!");

	let mut buffer: Vec<u8> = Vec::new();
	tcps.read_to_end(&mut buffer)
		.expect("Something went wrong trying to read GET response!");

	let ascii = String::from_utf8_lossy(&buffer);
	println!("Response:\n{}", ascii);
}
