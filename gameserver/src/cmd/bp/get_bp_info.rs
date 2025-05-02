use crate::error::AppError;
use crate::packet::ClientPacket;
use crate::util::send_message;
use sonettobuf::{CmdId, GetBpInfoReply};
// use sonettobuf::{BpScoreBonusInfo, BpSelfSelectBonus};
// use sonettobuf::GetBpInfoRequest;
use tokio::net::TcpStream;

pub async fn on_get_bp_info(
    cmd_id: CmdId,
    socket: &mut TcpStream,
    _req: ClientPacket,
) -> Result<(), AppError> {
    let data = GetBpInfoReply {
        id: None,
        score: None,
        pay_status: None,
        start_time: None,
        end_time: None,
        task_info: Vec::new(),
        score_bonus_info: Vec::new(),
        weekly_score: None,
        first_show: None,
        has_get_self_select_bonus: Vec::new(),
        sp_first_show: None,
    };

    send_message(socket, cmd_id, data, 0).await?;
    Ok(())
}
