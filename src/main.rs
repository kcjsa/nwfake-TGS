use tokio::net::{TcpListener, TcpStream};
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::try_join;

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:8067").await?;
    println!("endserver listening on 0.0.0.0:8067");

    loop {
        let (mut client, addr) = listener.accept().await?;
        println!("accept {}", addr);

        tokio::spawn(async move {
            let _ = handle(client).await;
        });
    }
}

async fn handle(mut client: TcpStream) -> io::Result<()> {
    // 最初の一発だけ接続先 (host:port)
    let mut buf = [0u8; 1024];
    let n = client.read(&mut buf).await?;
    if n == 0 {
        return Ok(());
    }

    let authority = String::from_utf8_lossy(&buf[..n]).trim().to_string();
    let server = TcpStream::connect(&authority).await?;

    let (mut cr, mut cw) = client.split();
    let (mut sr, mut sw) = server.into_split();

    try_join!(
        io::copy(&mut cr, &mut sw),
        io::copy(&mut sr, &mut cw),
    )?;

    Ok(())
}

