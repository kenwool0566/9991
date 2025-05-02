use crate::error::AppError;
use crate::packet::ClientPacket;
use crate::util::send_message;
use sonettobuf::{CmdId, GetWeekwalkInfoReply};
// use sonettobuf::WeekwalkInfo;
use tokio::net::TcpStream;

pub async fn on_get_weekwalk_info(
    cmd_id: CmdId,
    socket: &mut TcpStream,
    _req: ClientPacket,
) -> Result<(), AppError> {
    let data = GetWeekwalkInfoReply {
        info: None,
        time_this_week: None,
    };

    send_message(socket, cmd_id, data, 0).await?;
    Ok(())
}
