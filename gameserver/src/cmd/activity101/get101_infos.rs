use crate::error::AppError;
use crate::packet::ClientPacket;
use crate::util::send_message;
use sonettobuf::{CmdId, Get101InfosReply};
// use sonettobuf::{Act101Info, Act101SpInfo};
// use sonettobuf::GetAct147InfosRequest;
use tokio::net::TcpStream;

pub async fn on_get101_infos(
    cmd_id: CmdId,
    socket: &mut TcpStream,
    _req: ClientPacket,
) -> Result<(), AppError> {
    let data = Get101InfosReply {
        infos: Vec::new(),
        login_count: None,
        activity_id: None,
        sp_infos: Vec::new(),
        got_once_bonus: None,
    };

    send_message(socket, cmd_id, data, 0).await?;
    Ok(())
}
