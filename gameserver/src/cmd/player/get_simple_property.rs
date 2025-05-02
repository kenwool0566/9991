use crate::error::AppError;
use crate::packet::ClientPacket;
use crate::util::send_message;
use sonettobuf::{CmdId, GetSimplePropertyReply};
// use sonettobuf::SimpleProperty;
use tokio::net::TcpStream;

pub async fn on_get_simple_property(
    cmd_id: CmdId,
    socket: &mut TcpStream,
    _req: ClientPacket,
) -> Result<(), AppError> {
    let data = GetSimplePropertyReply {
        simple_properties: Vec::new(),
    };

    send_message(socket, cmd_id, data, 0).await?;
    Ok(())
}
