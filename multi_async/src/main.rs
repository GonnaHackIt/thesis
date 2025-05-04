use std::io::Result;
use tokio::{
    fs,
    io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

#[tokio::main]
async fn main() {
    // bind listener to socket
    let listener = TcpListener::bind("0.0.0.0:80")
        .await
        .expect("Can't bind to socket");

    // wait for connection and handle it
    loop {
        match listener.accept().await {
            Ok((connection, _addr)) => tokio::spawn(async {
                let result = handle_connection(connection).await;

                match result {
                    Err(err) => println!("Error during handling request: {err}"),
                    Ok(_) => {}
                }
            }),
            Err(err) => {
                println!("Error on incoming connection: {err}");
                continue;
            }
        };
    }
}

async fn handle_connection(mut connection: TcpStream) -> Result<()> {
    // random delay for test
    //std::thread::sleep(std::time::Duration::from_millis(rng.random_range(10..=30)));

    // simulate cpu work
    for _ in 0..1_000_000 {
        let val = 5;
        std::hint::black_box(val);
    }

    // io work
    let file_content = fs::read(format!("files/{}", rand::random_range(0..300))).await?;

    // specify the buffer for header
    let mut header = [0u8; 8];

    // read the header - how many bytes are going to be sent
    connection.read_exact(&mut header).await?;

    // turn 8 bytes into usize
    let len = usize::from_le_bytes(header);

    // specify the buffer for message
    let mut buffer = vec![0; len];

    // read the message
    connection.read_exact(&mut buffer[0..len]).await?;

    // anti-optimization
    std::hint::black_box(&mut buffer);

    //send back some file
    let len = file_content.len();
    header = len.to_le_bytes();

    connection.write_all(&header).await?;
    connection.write_all(&file_content).await?;

    Ok(())
}
