use crate::error::AppError;
use crate::packet::ClientPacket;
use crate::util::send_message;
use sonettobuf::{CmdId, GetDialogInfoReply};
use tokio::net::TcpStream;

pub async fn on_get_dialog_info(
    cmd_id: CmdId,
    socket: &mut TcpStream,
    _req: ClientPacket,
) -> Result<(), AppError> {
    let data = GetDialogInfoReply {
        dialog_ids: Vec::new(),
    };

    send_message(socket, cmd_id, data, 0).await?;
    Ok(())
}
