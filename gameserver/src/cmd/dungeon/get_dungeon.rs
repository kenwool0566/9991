use crate::error::AppError;
use crate::packet::ClientPacket;
use crate::util::send_message;
use sonettobuf::{CmdId, GetDungeonReply};
// use sonettobuf::{UserDungeon, DungeonLastHeroGroup, RewardPointInfo, UserChapterTypeNum};
use tokio::net::TcpStream;

pub async fn on_get_dungeon(
    cmd_id: CmdId,
    socket: &mut TcpStream,
    _req: ClientPacket,
) -> Result<(), AppError> {
    let data = GetDungeonReply {
        dungeon_info_list: Vec::new(),
        last_hero_group: Vec::new(),
        map_ids: Vec::new(),
        elements: Vec::new(),
        reward_point_info: Vec::new(),
        equip_sp_chapters: Vec::new(),
        chapter_type_nums: Vec::new(),
        finish_elements: Vec::new(),
        finish_puzzles: Vec::new(),
        dungeon_info_size: None,
    };

    send_message(socket, cmd_id, data, 0).await?;
    Ok(())
}
