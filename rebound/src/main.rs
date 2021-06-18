use std::net::{TcpListener, TcpStream};
use smol::{Async, io, io::AsyncReadExt};

async fn serve(mut stream: Async<TcpStream>) -> io::Result<()> {
    io::copy(&stream, &mut &stream).await?;
    Ok(())
}

fn main() {
    smol::block_on(async {
        let listener = Async::<TcpListener>::bind(([127, 0, 0, 1], 8000)).unwrap();

        loop {
            let (stream, _) = listener.accept().await.unwrap();

            smol::spawn(async move {
                serve(stream).await
            }).detach();
        }
    });
}
