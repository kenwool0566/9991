use crate::error::AppError;
use crate::packet::ClientPacket;
use crate::util::send_message;
use sonettobuf::{CmdId, GetTowerInfoReply};
// use sonettobuf::{TowerOpenNO, TowerNO, AssistBossNO};
use tokio::net::TcpStream;

pub async fn on_get_tower_info(
    cmd_id: CmdId,
    socket: &mut TcpStream,
    _req: ClientPacket,
) -> Result<(), AppError> {
    let data = GetTowerInfoReply {
        tower_opens: Vec::new(),
        towers: Vec::new(),
        assist_bosses: Vec::new(),
        mop_up_times: None,
    };

    send_message(socket, cmd_id, data, 0).await?;
    Ok(())
}
