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
		}
	}

	fn ready(&mut self, poll: &mut Poll, event: &Event) {
		if event.token() != self.token {
			return;
		}

		if event.readiness().is_readable() {
			self.do_read();
		}

		if event.readiness().is_writable() {
			self.do_write();
		}

		self.reregister(poll);
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
		let w = self.client.wants_write();

		if r && w {
			return Ready::readable() | Ready::writable();
		} else if w {
			return Ready::writable();
		} else {
			return Ready::readable();
		}
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
	let host = "genbyte.dev:443";
	let host_addr = lookup(host).expect("Couldn't find IP from hostname");
	let tcps = TcpStream::connect(&host_addr).expect("Couldn't connect to host");

	let mut tls_config = ClientConfig::new();
	tls_config
		.root_store
		.add_server_trust_anchors(&webpki_roots::TLS_SERVER_ROOTS);
	let arc_config = Arc::new(tls_config);

	let dns_ref = DNSNameRef::try_from_ascii_str(host).expect("Couldn't construct DNS ref");

	let tlsc = TlsClient::new(tcps, dns_ref, &arc_config);
}
