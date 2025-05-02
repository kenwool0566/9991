use crate::error::AppError;
use crate::packet::ClientPacket;
use crate::util::send_message;
use sonettobuf::{CmdId, GetSummonInfoReply};
// use sonettobuf::SummonPoolInfo;
use tokio::net::TcpStream;

pub async fn on_get_summon_info(
    cmd_id: CmdId,
    socket: &mut TcpStream,
    _req: ClientPacket,
) -> Result<(), AppError> {
    let data = GetSummonInfoReply {
        free_equip_summon: None,
        is_show_new_summon: None,
        new_summon_count: None,
        pool_infos: Vec::new(),
        total_summon_count: None,
    };

    send_message(socket, cmd_id, data, 0).await?;
    Ok(())
}
