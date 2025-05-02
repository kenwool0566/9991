use crate::error::AppError;
use crate::packet::ClientPacket;
use crate::util::send_message;
use sonettobuf::{CmdId, GetTurnbackInfoReply};
// use sonettobuf::TurnbackInfo;
use tokio::net::TcpStream;

pub async fn on_get_turnback_info(
    cmd_id: CmdId,
    socket: &mut TcpStream,
    _req: ClientPacket,
) -> Result<(), AppError> {
    let data = GetTurnbackInfoReply { info: None };

    send_message(socket, cmd_id, data, 0).await?;
    Ok(())
}
