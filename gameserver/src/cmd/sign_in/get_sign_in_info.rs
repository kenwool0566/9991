use crate::error::AppError;
use crate::packet::ClientPacket;
use crate::util::send_message;
use sonettobuf::{CmdId, GetSignInInfoReply};
// use sonettobuf::MonthCardHistory;
use tokio::net::TcpStream;

pub async fn on_get_sign_in_info(
    cmd_id: CmdId,
    socket: &mut TcpStream,
    _req: ClientPacket,
) -> Result<(), AppError> {
    let data = GetSignInInfoReply {
        has_sign_in_days: Vec::new(),
        addup_sign_in_day: None,
        has_get_addup_bonus: Vec::new(),
        open_function_time: None,
        has_month_card_days: Vec::new(),
        month_card_history: Vec::new(),
        birthday_hero_ids: Vec::new(),
        reward_mark: None,
    };

    send_message(socket, cmd_id, data, 0).await?;
    Ok(())
}
