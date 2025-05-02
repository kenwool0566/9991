use crate::error::AppError;
use crate::packet::ClientPacket;
use crate::util::send_message;
use sonettobuf::{CmdId, GetPlayerCardInfoReply};
// use sonettobuf::PlayerCardInfo;
use tokio::net::TcpStream;

pub async fn on_get_player_card_info(
    cmd_id: CmdId,
    socket: &mut TcpStream,
    _req: ClientPacket,
) -> Result<(), AppError> {
    let data = GetPlayerCardInfoReply {
        player_card_info: None,
    };

    send_message(socket, cmd_id, data, 0).await?;
    Ok(())
}
