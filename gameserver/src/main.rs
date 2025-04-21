use tokio::net::TcpListener;
use tokio::io::AsyncReadExt;
use server_config::{HOST, GAMESERVER_PORT, BUFFER_SIZE};

mod cmd;
mod handler;
mod packet;
mod util;

type DynError = Box<dyn std::error::Error + Send + Sync>;

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
                    if let Err(e) = handler::dispatch_command(&mut socket, &buffer[..n]).await {
                        eprintln!("{}", e);
                    }
                }
                Err(e) => eprintln!("lmao: {}", e),
            }
        });
    }
}
