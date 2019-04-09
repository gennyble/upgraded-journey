# upgraded-memory
A collection of code snippets and tests on GitHub because I don't know what else to do with them.

#### gendev_http
My first time working with Rust TcpStream. Makes a GET request to [my website](genbyte.dev).

#### gendev_https
Trying to get find the minimum amount of code that will fetch [my website's homepage](genbyte.dev).

[The first shot at this](https://github.com/genuinebyte/upgraded-journey/commit/5ad1e691f3600c921089c30ac172652ef53f7da3)
was using std::net::TcpStream
sockets. These did not work. ~~I went looking on the #rust IRC for help and we came to the conclusion
that rustls wasn't designed for blocking IO.~~ See gendev_https_blocking

#### gendev_https_blocking
An example of using rustls with blocking std::TcpStream sockets. I thought this was impossible but
[ctz][ctz] later found my [Twitter thread][rustls_twt_thread] where I struggled and called out
for help in a desperate attempt to get it working. He fixed it up and sent a PR. Thank you. 

[ctz]: https://github.com/ctz
[rustls_twt_thread]: https://twitter.com/genuinebyte/status/1113300356484747264
