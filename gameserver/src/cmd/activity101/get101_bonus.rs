use crate::error::AppError;
use crate::packet::ClientPacket;
use crate::util::send_message;
use sonettobuf::{CmdId, Get101BonusReply};
// use sonettobuf::Get101BonusRequest;
use tokio::net::TcpStream;

pub async fn on_get101_bonus(
    cmd_id: CmdId,
    socket: &mut TcpStream,
    _req: ClientPacket,
) -> Result<(), AppError> {
    let data = Get101BonusReply {
        id: None,
        activity_id: None,
    };

    send_message(socket, cmd_id, data, 0).await?;
    Ok(())
}
