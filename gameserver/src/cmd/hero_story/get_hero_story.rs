use crate::error::AppError;
use crate::packet::ClientPacket;
use crate::util::send_message;
use sonettobuf::{CmdId, GetHeroStoryReply};
// use sonettobuf::{HeroStoryInfo, HeroStoryTime};
use tokio::net::TcpStream;

pub async fn on_get_hero_story(
    cmd_id: CmdId,
    socket: &mut TcpStream,
    _req: ClientPacket,
) -> Result<(), AppError> {
    let data = GetHeroStoryReply {
        story_infos: Vec::new(),
        new_story_list: Vec::new(),
        times: Vec::new(),
        left_num: None,
        today_exchange: None,
        week_progress: None,
        week_has_get: None,
    };

    send_message(socket, cmd_id, data, 0).await?;
    Ok(())
}
