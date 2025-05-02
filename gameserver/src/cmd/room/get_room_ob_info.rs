use crate::error::AppError;
use crate::packet::ClientPacket;
use crate::util::send_message;
use sonettobuf::{CmdId, GetRoomObInfoReply};
// use sonettobuf::{BlockInfo, BuildingInfo, FormulaInfo, RoomHeroData, ProductionLineInfo, RoomSkinInfo, RoadInfo};
use tokio::net::TcpStream;

pub async fn on_get_room_ob_info(
    cmd_id: CmdId,
    socket: &mut TcpStream,
    _req: ClientPacket,
) -> Result<(), AppError> {
    let data = GetRoomObInfoReply {
        infos: Vec::new(),
        building_infos: Vec::new(),
        formula_infos: Vec::new(),
        room_level: None,
        room_hero_datas: Vec::new(),
        production_lines: Vec::new(),
        has_get_room_themes: Vec::new(),
        need_block_data: None,
        skins: Vec::new(),
        road_infos: Vec::new(),
    };

    send_message(socket, cmd_id, data, 0).await?;
    Ok(())
}
