use crate::error::AppError;
use crate::packet::ClientPacket;
use crate::util::send_message;
use sonettobuf::{CmdId, GetAntiqueInfoReply};
// use sonettobuf::AntiqueInfo;
use tokio::net::TcpStream;

pub async fn on_get_antique_info(
    cmd_id: CmdId,
    socket: &mut TcpStream,
    _req: ClientPacket,
) -> Result<(), AppError> {
    let data = GetAntiqueInfoReply {
        antiques: Vec::new(),
    };

    send_message(socket, cmd_id, data, 0).await?;
    Ok(())
}
