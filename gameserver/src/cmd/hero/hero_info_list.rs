use crate::DynError;
use crate::packet::ClientPacket;
use crate::util::send_message;
use sonettobuf::{CmdId, HeroInfoListReply};
// use sonettobuf::{HeroBirthdayInfo, HeroInfo, SkinInfo, HeroAttribute, HeroExAttribute, HeroSpAttribute, HeroEquipAttribute, TalentCubeInfo};
use tokio::net::TcpStream;

pub async fn on_hero_info_list(
    cmd_id: CmdId,
    socket: &mut TcpStream,
    _req: ClientPacket,
) -> Result<(), DynError> {
    let data = HeroInfoListReply {
        heros: Vec::new(),
        touch_count_left: None,
        all_hero_skin: Vec::new(),
        birthday_infos: Vec::new(),
    };

    send_message(socket, cmd_id, data, 0).await?;
    Ok(())
}
