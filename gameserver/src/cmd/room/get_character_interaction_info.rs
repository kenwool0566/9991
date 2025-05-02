use crate::error::AppError;
use crate::packet::ClientPacket;
use crate::util::send_message;
use sonettobuf::{CmdId, GetCharacterInteractionInfoReply};
// use sonettobuff::CharacterInteractionInfo;
use tokio::net::TcpStream;

pub async fn on_get_character_interaction_info(
    cmd_id: CmdId,
    socket: &mut TcpStream,
    _req: ClientPacket,
) -> Result<(), AppError> {
    let data = GetCharacterInteractionInfoReply {
        infos: Vec::new(),
        interaction_count: None,
    };

    send_message(socket, cmd_id, data, 0).await?;
    Ok(())
}
