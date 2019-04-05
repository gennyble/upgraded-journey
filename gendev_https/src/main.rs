use mio::event::Events;
use mio::net::TcpStream;
use mio::{Poll, PollOpt, Ready, Token};
use rustls::{ClientConfig, ClientSession, Session};
use std::io::{Read, Write};
use std::net::ToSocketAddrs;
use std::sync::Arc;
use webpki::DNSNameRef;

fn main() {
	let CLIENT = Token(0);
	let mut tcps = get_connection();

	let poll = Poll::new().unwrap();
	poll.register(
		&tcps,
		CLIENT,
		Ready::readable() | Ready::writable(),
		PollOpt::edge(),
	)
	.unwrap();

	let mut config = ClientConfig::new();
	config
		.root_store
		.add_server_trust_anchors(&webpki_roots::TLS_SERVER_ROOTS);
	let arc_config = Arc::new(config);
	let genbyte_dev = DNSNameRef::try_from_ascii_str("genbyte.dev").unwrap();
	let mut client = ClientSession::new(&arc_config, genbyte_dev);

	client.write(b"GET / HTTP/1.0\r\n\r\n").unwrap();

	let mut events = Events::with_capacity(64);
	let mut wrote_request = false;
	loop {
		poll.poll(&mut events, None).unwrap();

		for event in events.iter() {
			match event.token() {
				CLIENT => {
					if !wrote_request && event.readiness().is_writable() && client.wants_write() {
						print!("Trying to write request...");
						println!("{} bytes", client.write_tls(&mut tcps).unwrap());
						wrote_request = true;
					}

					if event.readiness().is_readable() && client.wants_read() {
						//read_tls returns non-zero
						let tls_read_size = client.read_tls(&mut tcps).unwrap();
						//No isse on process_new_packets
						client.process_new_packets().unwrap();

						let mut plain = Vec::new();
						//read_to_end returns Ok(0)??
						let plain_read_size = client.read_to_end(&mut plain).unwrap();

						let ascii = String::from_utf8(plain).unwrap();
						println!(
							"Response[{}|{}]:\n{}",
							tls_read_size, plain_read_size, ascii
						);
						return;
					}
				}
			}
		}
	}
}

fn get_connection() -> TcpStream {
	let genuine_addrs = "genbyte.dev:443"
		.to_socket_addrs()
		.expect("Failed to parse hostname");

	let mut genuine_connection: Option<TcpStream> = None;
	for addr in genuine_addrs {
		if let Ok(val) = TcpStream::connect(&addr) {
			println!("Using address: {}", addr);
			genuine_connection = Some(val);
			break;
		}
	}

	if let Some(val) = genuine_connection {
		val
	} else {
		panic!("Failed to connect to genbyte.dev");
	}
}
