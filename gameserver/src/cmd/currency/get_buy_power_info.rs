use crate::error::AppError;
use crate::packet::ClientPacket;
use crate::util::send_message;
use sonettobuf::{CmdId, GetBuyPowerInfoReply};
use tokio::net::TcpStream;

pub async fn on_get_buy_power_info(
    cmd_id: CmdId,
    socket: &mut TcpStream,
    _req: ClientPacket,
) -> Result<(), AppError> {
    let data = GetBuyPowerInfoReply {
        can_buy_count: None,
    };

    send_message(socket, cmd_id, data, 0).await?;
    Ok(())
}
