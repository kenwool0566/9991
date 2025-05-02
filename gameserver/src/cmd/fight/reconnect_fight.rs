use crate::error::AppError;
use crate::packet::ClientPacket;
use crate::util::send_message;
use sonettobuf::{CmdId, ReconnectFightReply};
// use sonettobuf::{Fight, FightRound, FightReason, FightGroup};
use tokio::net::TcpStream;

pub async fn on_reconnect_fight(
    cmd_id: CmdId,
    socket: &mut TcpStream,
    _req: ClientPacket,
) -> Result<(), AppError> {
    let data = ReconnectFightReply {
        fight: None,
        last_round: None,
        fight_reason: None,
        fight_group: None,
    };

    send_message(socket, cmd_id, data, 0).await?;
    Ok(())
}
