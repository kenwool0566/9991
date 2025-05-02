use crate::error::AppError;
use crate::packet::ClientPacket;
use crate::util::send_message;
use sonettobuf::{CmdId, MarkMainThumbnailReply};
use tokio::net::TcpStream;

pub async fn on_mark_main_thumbnail(
    cmd_id: CmdId,
    socket: &mut TcpStream,
    _req: ClientPacket,
) -> Result<(), AppError> {
    let data = MarkMainThumbnailReply {
        // empty..
    };

    send_message(socket, cmd_id, data, 0).await?;
    Ok(())
}
