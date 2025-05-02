use crate::error::AppError;
use crate::packet::ClientPacket;
use crate::util::send_message;
use sonettobuf::{CmdId, GetActivityInfosReply};
// use sonettobuf::ActivityInfo;
use tokio::net::TcpStream;

pub async fn on_get_activity_infos(
    cmd_id: CmdId,
    socket: &mut TcpStream,
    _req: ClientPacket,
) -> Result<(), AppError> {
    let data = GetActivityInfosReply {
        activity_infos: Vec::new(),
    };

    send_message(socket, cmd_id, data, 0).await?;
    Ok(())
}
