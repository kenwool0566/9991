use crate::error::AppError;
use crate::packet::ClientPacket;
use crate::util::send_message;
use sonettobuf::{CmdId, SetSimplePropertyReply};
// use sonettobuf::SetSimplePropertyRequest;
use tokio::net::TcpStream;

pub async fn on_set_simple_property(
    cmd_id: CmdId,
    socket: &mut TcpStream,
    _req: ClientPacket,
) -> Result<(), AppError> {
    let data = SetSimplePropertyReply {
        // empty...
    };

    send_message(socket, cmd_id, data, 0).await?;
    Ok(())
}
