use crate::error::AppError;
use crate::packet::ClientPacket;
use crate::util::send_message;
use sonettobuf::{AutoUseExpirePowerItemReply, CmdId};
use tokio::net::TcpStream;

pub async fn on_auto_use_expire_power_item(
    cmd_id: CmdId,
    socket: &mut TcpStream,
    _req: ClientPacket,
) -> Result<(), AppError> {
    let data = AutoUseExpirePowerItemReply { used: None };

    send_message(socket, cmd_id, data, 0).await?;
    Ok(())
}
