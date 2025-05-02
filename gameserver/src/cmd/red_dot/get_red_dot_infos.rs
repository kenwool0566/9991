use crate::error::AppError;
use crate::packet::ClientPacket;
use crate::util::send_message;
use sonettobuf::{CmdId, GetRedDotInfosReply};
// use sonettobuf::GetRedDotInfosRequest;
// use sonettobuf::RedDotGroup;
use tokio::net::TcpStream;

pub async fn on_get_red_dot_infos(
    cmd_id: CmdId,
    socket: &mut TcpStream,
    _req: ClientPacket,
) -> Result<(), AppError> {
    let data = GetRedDotInfosReply {
        red_dot_infos: Vec::new(),
    };

    send_message(socket, cmd_id, data, 0).await?;
    Ok(())
}
