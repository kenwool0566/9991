use crate::error::AppError;
use crate::packet::ClientPacket;
use crate::util::send_message;
use sonettobuf::{CmdId, InstructionDungeonInfoReply};
use tokio::net::TcpStream;

pub async fn on_instruction_dungeon_info(
    cmd_id: CmdId,
    socket: &mut TcpStream,
    _req: ClientPacket,
) -> Result<(), AppError> {
    let data = InstructionDungeonInfoReply {
        unlock_ids: Vec::new(),
        get_reward_ids: Vec::new(),
        get_final_reward: None,
        open_ids: Vec::new(),
    };

    send_message(socket, cmd_id, data, 0).await?;
    Ok(())
}
