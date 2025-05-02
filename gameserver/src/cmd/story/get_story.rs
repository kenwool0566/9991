use crate::error::AppError;
use crate::packet::ClientPacket;
use crate::util::send_message;
use sonettobuf::{CmdId, GetStoryReply};
// use sonettobuf::ProcessingStoryInfo;
use tokio::net::TcpStream;

pub async fn on_get_story(
    cmd_id: CmdId,
    socket: &mut TcpStream,
    _req: ClientPacket,
) -> Result<(), AppError> {
    let data = GetStoryReply {
        finish_list: Vec::new(),
        processing_list: Vec::new(),
    };

    send_message(socket, cmd_id, data, 0).await?;
    Ok(())
}
