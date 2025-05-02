use crate::error::AppError;
use crate::packet::ClientPacket;
use crate::util::send_message;
use sonettobuf::{CmdId, UpdateClientStatBaseInfoReply};
use tokio::net::TcpStream;

pub async fn on_update_client_stat_base_info(
    cmd_id: CmdId,
    socket: &mut TcpStream,
    _req: ClientPacket,
) -> Result<(), AppError> {
    let data = UpdateClientStatBaseInfoReply {
        account_bind_bonus: Some(0),
    };

    send_message(socket, cmd_id, data, 0).await?;
    Ok(())
}
