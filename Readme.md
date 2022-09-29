# upgraded-memory
A collection of code snippets and tests on GitHub because I don't know what else to do with them.

#### gendev_http [Rust]
My first time working with Rust [TcpStream][std_tcpstream]. Makes a GET request to
[my website](nyble.dev).

#### gendev_https [Rust]
Trying to get find the minimum amount of code that will fetch [my website's homepage](nyble.dev).
[rustls][rustls_repo] and [mio][mio_repo] are difficult separately so together... it's a struggle.
This repository is a modification of the [TlsClient][rustls_tlsclient] example in the rustls repo.

#### gendev_https_blocking [Rust]
An example of using rustls with blocking std::TcpStream sockets. I thought this was impossible but
[ctz][ctz] later found my ~~[Twitter thread][rustls_twt_thread]~~ where I struggled and called out
for help in a desperate attempt to get it working. He fixed it up and sent a [PR][pr_1]. Thank you. 

#### typeid [Rust]
Trying to figure out how the heck [bevy][bevy-src] and [futures][futures-src] do their cool thing
with types.

Got it with 1 paramter but multiple without a tuple looks ahh. [This][bevy-systemparam] code from
bevy might do to help.

#### lyingstatic [Rust]
The source for ~~[this tweet][lyingstatic-tweet]~~ a cursed thing.

[lyingstatic-tweet]: https://twitter.com/genuinebyte/status/1329947987448123397?s=20

[ctz]: https://github.com/ctz
[mio_repo]: https://github.com/carllerche/mio
[pr_1]: https://github.com/genuinebyte/upgraded-journey/pull/1
[rustls_repo]: https://github.com/ctz/rustls
[rustls_twt_thread]: https://twitter.com/genuinebyte/status/1113300356484747264
[rustls_tlsclient]: https://github.com/ctz/rustls/blob/master/rustls-mio/examples/tlsclient.rs
[std_tcpstream]: https://doc.rust-lang.org/std/net/struct.TcpStream.html
[bevy-src]: https://github.com/bevyengine
[futures-src]: https://github.com/rust-lang/futures-rs
[bevy-systemparam]: https://github.com/bevyengine/bevy/blob/7628f4a64e6f3eacfc4aad3bb6b3d54309722682/crates/bevy_ecs/src/system/system_param.rs
