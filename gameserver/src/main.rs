use byteorder::{BE, ByteOrder};
use bytes::BytesMut;
use server_config::{BUFFER_SIZE, GAMESERVER_PORT, HOST};
use tokio::io::AsyncReadExt;
use tokio::net::TcpListener;

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
            // this shit is horrible
            let mut temp_buffer = BytesMut::with_capacity(BUFFER_SIZE);
            let mut temp_buffer2 = BytesMut::with_capacity(BUFFER_SIZE * 2);

            loop {
                match socket.read_buf(&mut temp_buffer).await {
                    Ok(0) => return,
                    Ok(_) => {
                        temp_buffer2.extend_from_slice(&temp_buffer);
                        temp_buffer.clear();

                        while temp_buffer2.len() >= 4 {
                            let size = BE::read_u32(&temp_buffer2[0..4]) as usize + 4;

                            if temp_buffer2.len() < size {
                                break;
                            }

                            let buffer = temp_buffer2.split_to(size);
                            println!("buffer (complete): {:?}", &buffer);
                            // println!("remaining temp_buffer2: {:?}", &temp_buffer2);
                            // println!("calculated size: {}", size);
                            // println!("actual size: {}", buffer.len());

                            if let Err(e) =
                                handler::dispatch_command(&mut socket, &buffer[..]).await
                            {
                                eprintln!("{}", e);
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("read error: {}", e);
                        return;
                    }
                }
            }
        });
    }
}
