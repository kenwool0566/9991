use crate::error::AppError;
use crate::packet::ClientPacket;
use crate::util::send_message;
use sonettobuf::{CmdId, CritterGetInfoReply};
// use sonettobuf::CritterInfo;
use tokio::net::TcpStream;

pub async fn on_critter_get_info(
    cmd_id: CmdId,
    socket: &mut TcpStream,
    _req: ClientPacket,
) -> Result<(), AppError> {
    let data = CritterGetInfoReply {
        critter_infos: Vec::new(),
    };

    send_message(socket, cmd_id, data, 0).await?;
    Ok(())
}
