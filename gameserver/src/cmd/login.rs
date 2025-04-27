use crate::DynError;
use crate::packet::ClientPacket;
use crate::util::send_raw_buffer;
use byteorder::{BE, ByteOrder};
use server_config::USER_ID;
use sonettobuf::CmdId;
use tokio::net::TcpStream;

pub async fn on_login(
    cmd_id: CmdId,
    socket: &mut TcpStream,
    _req: ClientPacket,
) -> Result<(), DynError> {
    let mut data = vec![0u8; 10];
    BE::write_u64(&mut data[2..10], USER_ID);

    send_raw_buffer(socket, cmd_id, data, 0).await?;
    Ok(())
}

pub async fn on_reconnect(
    cmd_id: CmdId,
    socket: &mut TcpStream,
    _req: ClientPacket,
) -> Result<(), DynError> {
    send_raw_buffer(socket, cmd_id, vec![0x01], 0).await?;
    Ok(())
}
