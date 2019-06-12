use mio::event::{Event, Events};
use mio::net::TcpStream;
use mio::{Poll, PollOpt, Ready, Token};
use rustls::{ClientConfig, ClientSession, Session};
use std::io::Error as IoError;
use std::io::Result as IoResult;
use std::io::{ErrorKind, Read, Write};
use std::net::SocketAddr;
use std::net::ToSocketAddrs;
use std::sync::Arc;
use webpki::DNSNameRef;
//https://github.com/ctz/rustls/blob/master/rustls-mio/examples/tlsclient.rs

struct TlsClient {
	socket: TcpStream,
	client: ClientSession,
	token: Token,
	request_written: bool,
	closing: bool,
}

impl TlsClient {
	fn new(
		tcps: TcpStream,
		hostname: DNSNameRef,
		config: &Arc<ClientConfig>,
		token: Token,
	) -> Self {
		TlsClient {
			socket: tcps,
			client: ClientSession::new(config, hostname),
			token,
			request_written: false,
			closing: false,
		}
	}

	fn ready(&mut self, poll: &mut Poll, event: &Event) -> bool {
		if event.token() != self.token {
			return true;
		}

		if event.readiness().is_readable() {
			self.do_read();
		}

		if event.readiness().is_writable() {
			self.do_write();
		}

		if self.closing {
			return false;
		} else {
			self.reregister(poll);
			return true;
		}
	}

	fn register(&self, poll: &mut Poll) -> IoResult<()> {
		poll.register(
			&self.socket,
			self.token,
			self.interest(),
			PollOpt::level() | PollOpt::oneshot(),
		)
	}

	fn reregister(&self, poll: &mut Poll) -> IoResult<()> {
		poll.reregister(
			&self.socket,
			self.token,
			self.interest(),
			PollOpt::level() | PollOpt::oneshot(),
		)
	}

	fn interest(&self) -> Ready {
		let r = self.client.wants_read();
		let w = if self.request_written == false {
			self.client.wants_write()
		} else {
			false
		};

		if r && w {
			return Ready::readable() | Ready::writable();
		} else if w {
			return Ready::writable();
		} else {
			return Ready::readable();
		}
	}

	fn do_write(&mut self) {
		if !self.request_written {
			self.request_written = true;
			let write = self.client.write(b"GET / HTTP/1.0\r\n\r\n").unwrap();
			let write_tls = self.client.write_tls(&mut self.socket).unwrap();
			println!("[do_write:99] Request written");
			println!("{} bytes were written before encryption, {} after", write, write_tls);
		}
	}

	//TODO: https://github.com/ctz/rustls/blob/master/rustls-mio/examples/tlsclient.rs#L100
	fn do_read(&mut self) {
		println!("[do_read:105] Request to read");

		// Read TLS data from the Socket
		let read_res = self.client.read_tls(&mut self.socket);
		if read_res.is_err() {
			println!("TLS Read Error: {:?}", read_res.unwrap_err());
			self.closing = true;
			return;
		}

		// If ready but no data: EOF
		if read_res.unwrap() == 0 {
			println!("No data available, must be EoF...");
			self.closing = true;
			return;
		}

		// Reading TLS data might have caused errors. These are malformed TLS
		// and are fatal.
		let process_res = self.client.process_new_packets();
		if process_res.is_err() {
			println!("TLS Process Error: {:?}", process_res.unwrap_err());
			self.closing = true;
			return;
		}

		// Having read and processed the TLS data, there might be plaintext
		// waiting to be read. It will be printed to STDOUT.
		let mut buffer: Vec<u8> = Vec::new();
		let read_end_res = self.client.read_to_end(&mut buffer);
		if read_end_res.is_err() {
			println!("TLS Read Error:{:?}", read_end_res.unwrap_err());
			self.closing = true;
			return;
		}

		let read_end_size = read_end_res.unwrap();
		if read_end_size == 0 {
			println!("Response is zero, waiting still...");
			return;
		}

		let plaintext = String::from_utf8_lossy(&buffer);
		println!("[{}] Response:\n{}", read_end_size, plaintext);

		// All we wanted was a response back from the server, the socket can be
		// closed.
		self.closing = true;
	}
}

fn lookup(host: &str) -> IoResult<SocketAddr> {
	let mut addrs = host.to_socket_addrs()?;

	if let Some(addr) = addrs.next() {
		Ok(addr)
	} else {
		Err(IoError::new(ErrorKind::NotFound, "Could not find host"))
	}
}

fn main() {
	let host_no_port = "genbyte.dev";
	let host = format!("{}:443", host_no_port);
	let host_addr = lookup(&host).expect("Couldn't find IP from hostname");
	let tcps = TcpStream::connect(&host_addr).expect("Couldn't connect to host");

	let mut tls_config = ClientConfig::new();
	tls_config
		.root_store
		.add_server_trust_anchors(&webpki_roots::TLS_SERVER_ROOTS);
	let arc_config = Arc::new(tls_config);

	let dns_ref = DNSNameRef::try_from_ascii_str(host_no_port).expect("Couldn't construct DNS ref");

	let tok = Token(1);

	let mut tlsc = TlsClient::new(tcps, dns_ref, &arc_config, tok);

	let mut poll = Poll::new().expect("Failed to create poll");
	let mut events = Events::with_capacity(32);
	tlsc.register(&mut poll)
		.expect("Failed to register TlsClient with poll");

	loop {
		poll.poll(&mut events, None).expect("Failed to poll");

		for e in events.iter() {
			if !tlsc.ready(&mut poll, &e) {
				println!("All done here...");
				return;
			}
		}
	}
}
