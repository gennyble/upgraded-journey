use rustls::{ClientConfig, ClientSession, Stream};
use std::io::{Read, Write};
use std::net::{TcpStream, ToSocketAddrs};
use std::sync::Arc;
use webpki::DNSNameRef;

fn main() {
	let genuine_addrs = "genbyte.dev:443"
		.to_socket_addrs()
		.expect("Failed to parse hostname");

	let mut config = ClientConfig::new();
	config
		.root_store
		.add_server_trust_anchors(&webpki_roots::TLS_SERVER_ROOTS);

	let arc_config = Arc::new(config);
	let genuine_com = DNSNameRef::try_from_ascii_str("genbyte.dev").unwrap();
	let mut tls = ClientSession::new(&arc_config, genuine_com);

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

	let mut tls_stream = Stream::new(&mut tls, &mut tcps);
	tls_stream.write(b"GET / HTTP/1.0\r\n\r\n").unwrap();

	let mut plain = Vec::new();
	let err = tls_stream.read_to_end(&mut plain);

	let ascii = String::from_utf8_lossy(&plain);
	println!("Response:\n{}", ascii);
	println!("Result: {:?}", err); 
}
