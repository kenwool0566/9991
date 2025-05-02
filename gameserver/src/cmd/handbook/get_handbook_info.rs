use crate::error::AppError;
use crate::packet::ClientPacket;
use crate::util::send_message;
use sonettobuf::{CmdId, GetHandbookInfoReply};
// use sonettobuf::{Handbook, ChatperElementInfo};
use tokio::net::TcpStream;

pub async fn on_get_handbook_info(
    cmd_id: CmdId,
    socket: &mut TcpStream,
    _req: ClientPacket,
) -> Result<(), AppError> {
    let data = GetHandbookInfoReply {
        infos: Vec::new(),
        element_info: Vec::new(),
    };

    send_message(socket, cmd_id, data, 0).await?;
    Ok(())
}
