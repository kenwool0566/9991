use crate::error::AppError;
use crate::packet::ClientPacket;
use crate::util::send_message;
use sonettobuf::{CmdId, GetExploreSimpleInfoReply};
// use sonettobuf::{ExploreChapterSimpleNO, ExploreMapSimpleNO};
use tokio::net::TcpStream;

pub async fn on_get_explore_simple_info(
    cmd_id: CmdId,
    socket: &mut TcpStream,
    _req: ClientPacket,
) -> Result<(), AppError> {
    let data = GetExploreSimpleInfoReply {
        last_map_id: None,
        chapter_simple: Vec::new(),
        map_simple: Vec::new(),
        unlock_map_ids: Vec::new(),
        is_show_bag: None,
    };

    send_message(socket, cmd_id, data, 0).await?;
    Ok(())
}
