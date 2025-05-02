use crate::error::AppError;
use crate::packet::ClientPacket;
use crate::util::send_message;
use sonettobuf::{CmdId, GetChargeInfoReply};
// use sonettobuf::ChargeInfo;
use tokio::net::TcpStream;

pub async fn on_get_charge_info(
    cmd_id: CmdId,
    socket: &mut TcpStream,
    _req: ClientPacket,
) -> Result<(), AppError> {
    let data = GetChargeInfoReply {
        infos: Vec::new(),
        sandbox_enable: None,
        sandbox_balance: None,
    };

    send_message(socket, cmd_id, data, 0).await?;
    Ok(())
}
