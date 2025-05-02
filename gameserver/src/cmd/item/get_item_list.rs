use crate::error::AppError;
use crate::packet::ClientPacket;
use crate::util::send_message;
use sonettobuf::{CmdId, GetItemListReply};
// use sonettobuf::{Item, PowerItem, InsightItem};
use tokio::net::TcpStream;

pub async fn on_get_item_list(
    cmd_id: CmdId,
    socket: &mut TcpStream,
    _req: ClientPacket,
) -> Result<(), AppError> {
    let data = GetItemListReply {
        items: Vec::new(),
        power_items: Vec::new(),
        insight_items: Vec::new(),
    };

    send_message(socket, cmd_id, data, 0).await?;
    Ok(())
}
