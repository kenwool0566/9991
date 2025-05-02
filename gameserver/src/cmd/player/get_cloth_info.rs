use crate::error::AppError;
use crate::packet::ClientPacket;
use crate::util::send_message;
use sonettobuf::{CmdId, GetClothInfoReply};
// use sonettobuf::{PlayerClothInfo, PlayerCloth};
use tokio::net::TcpStream;

pub async fn on_get_cloth_info(
    cmd_id: CmdId,
    socket: &mut TcpStream,
    _req: ClientPacket,
) -> Result<(), AppError> {
    let data = GetClothInfoReply { cloth_infos: None };

    send_message(socket, cmd_id, data, 0).await?;
    Ok(())
}
