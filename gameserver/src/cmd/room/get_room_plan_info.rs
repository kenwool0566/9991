use crate::error::AppError;
use crate::packet::ClientPacket;
use crate::util::send_message;
use sonettobuf::{CmdId, GetRoomPlanInfoReply};
// use sonettobuf::RoomPlanInfo;
use tokio::net::TcpStream;

pub async fn on_get_room_plan_info(
    cmd_id: CmdId,
    socket: &mut TcpStream,
    _req: ClientPacket,
) -> Result<(), AppError> {
    let data = GetRoomPlanInfoReply {
        infos: Vec::new(),
        can_share_count: None,
        can_use_share_count: None,
        total_use_count: None,
    };

    send_message(socket, cmd_id, data, 0).await?;
    Ok(())
}
