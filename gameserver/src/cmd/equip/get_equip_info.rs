use crate::error::AppError;
use crate::packet::ClientPacket;
use crate::util::send_message;
use sonettobuf::{CmdId, GetEquipInfoReply};
// use sonettobuf::Equip;
use tokio::net::TcpStream;

pub async fn on_get_equip_info(
    cmd_id: CmdId,
    socket: &mut TcpStream,
    _req: ClientPacket,
) -> Result<(), AppError> {
    let data = GetEquipInfoReply { equips: Vec::new() };

    send_message(socket, cmd_id, data, 0).await?;
    Ok(())
}
