use crate::error::AppError;
use crate::packet::ClientPacket;
use crate::util::send_raw_buffer;
use sonettobuf::CmdId;
use tokio::net::TcpStream;

pub async fn on_reconnect(
    cmd_id: CmdId,
    socket: &mut TcpStream,
    _req: ClientPacket,
) -> Result<(), AppError> {
    send_raw_buffer(socket, cmd_id, vec![0x01], 0).await?;
    Ok(())
}
