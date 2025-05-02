use crate::error::AppError;
use crate::packet::ClientPacket;
use crate::util::send_message;
use sonettobuf::{Act160GetInfoReply, CmdId};
// use sonettobuf::Act160MissionInfo;
// use sonettobuf::Act160GetInfoRequest;
use tokio::net::TcpStream;

pub async fn on_act160_get_info(
    cmd_id: CmdId,
    socket: &mut TcpStream,
    _req: ClientPacket,
) -> Result<(), AppError> {
    let data = Act160GetInfoReply {
        activity_id: None,
        act160_infos: Vec::new(),
    };

    send_message(socket, cmd_id, data, 0).await?;
    Ok(())
}
