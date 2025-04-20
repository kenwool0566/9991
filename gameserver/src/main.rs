use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use server_config::{HOST, GAMESERVER_PORT, BUFFER_SIZE};

mod packet;
use packet::ClientPacket;
use packet::ServerPacket;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let addr = format!("{}:{}", HOST, GAMESERVER_PORT);
    let listener = TcpListener::bind(&addr).await.unwrap();
    println!("tcp {} is listening :3", &addr);

    loop {
        let (mut socket, client) = listener.accept().await.unwrap();
        println!("connected: {:?}", client);

        tokio::spawn(async move {
            let mut buffer = [0u8; BUFFER_SIZE];
            match socket.read(&mut buffer).await {
                Ok(0) => return,
                Ok(n) => {
                    println!("{:?}", ClientPacket::decode(&buffer[..n]));
                    socket.write_all(&buffer[..n]).await.unwrap();
                }
                Err(e) => eprintln!("lmao: {}", e),
            }
        });
    }
}
