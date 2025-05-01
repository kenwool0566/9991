use crate::DynError;
use crate::packet::ClientPacket;
use crate::util::send_message;
use sonettobuf::{CmdId, GetHeroGroupCommonListReply};
// use sonettobuf::{HeroGourpType, HeroGroupInfo, HeroGroupEquip};
use tokio::net::TcpStream;

pub async fn on_get_hero_group_common_list(
    cmd_id: CmdId,
    socket: &mut TcpStream,
    _req: ClientPacket,
) -> Result<(), DynError> {
    let data = GetHeroGroupCommonListReply {
        hero_group_commons: Vec::new(),
        hero_gourp_types: Vec::new(),
    };

    send_message(socket, cmd_id, data, 0).await?;
    Ok(())
}
