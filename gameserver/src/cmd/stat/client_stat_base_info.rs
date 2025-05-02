use crate::error::AppError;
use crate::packet::ClientPacket;
use crate::util::send_raw_buffer;
use sonettobuf::CmdId;
use tokio::net::TcpStream;

pub async fn on_client_stat_base_info(
    cmd_id: CmdId,
    socket: &mut TcpStream,
    _req: ClientPacket,
) -> Result<(), AppError> {
    send_raw_buffer(socket, cmd_id, Vec::new(), 0).await?;
    Ok(())
}
