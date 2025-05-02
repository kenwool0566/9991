use crate::error::AppError;
use crate::packet::ClientPacket;
use crate::util::send_message;
use sonettobuf::{CmdId, GetAssistBonusReply};
use tokio::net::TcpStream;

pub async fn on_get_assist_bonus(
    cmd_id: CmdId,
    socket: &mut TcpStream,
    _req: ClientPacket,
) -> Result<(), AppError> {
    let data = GetAssistBonusReply {
        assist_bonus: None,
        has_receive_assist_bonus: None,
    };

    send_message(socket, cmd_id, data, 0).await?;
    Ok(())
}
