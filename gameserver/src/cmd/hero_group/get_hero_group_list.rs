use crate::error::AppError;
use crate::packet::ClientPacket;
use crate::util::send_message;
use sonettobuf::{CmdId, GetHeroGroupListReply};
// use sonettobuf::{HeroGroupInfo, HeroGroupEquip};
use tokio::net::TcpStream;

pub async fn on_get_hero_group_list(
    cmd_id: CmdId,
    socket: &mut TcpStream,
    _req: ClientPacket,
) -> Result<(), AppError> {
    let data = GetHeroGroupListReply {
        group_info_list: Vec::new(),
    };

    send_message(socket, cmd_id, data, 0).await?;
    Ok(())
}
