use byteorder::{BE, ByteOrder};
use bytes::BytesMut;
use common::{GAMESERVER_PORT, HOST, init_tracing};
use tokio::io::AsyncReadExt;
use tokio::net::TcpListener;

mod cmd;
mod bin_packet;
mod error;
mod handler;
mod packet;
mod util;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let addr = format!("{}:{}", HOST, GAMESERVER_PORT);
    let listener = TcpListener::bind(&addr).await.unwrap();

    init_tracing();
    tracing::info!("Listening on tcp://{}", &addr);

    loop {
        let (mut socket, client) = listener.accept().await.unwrap();
        tracing::info!("New Client: {:?}", client);

        tokio::spawn(async move {
            let mut peek_buf = [0; 4];
            let peek_cnt = socket.peek(&mut peek_buf).await.unwrap();
            assert_eq!(peek_cnt, 4);

            let packet_size = BE::read_i32(&peek_buf[..]) as usize + 4;
            let mut buffer = BytesMut::with_capacity(packet_size);

            match socket.read_buf(&mut buffer).await {
                Ok(_) => {
                    if let Err(e) = handler::dispatch_command(&mut socket, &buffer[..]).await {
                        tracing::error!("{e}");
                    }
                }
                Err(e) => {
                    tracing::error!("{e}");
                    return;
                }
            }
        });
    }
}
