use crate::error::AppError;
use crate::packet::ClientPacket;
use crate::util::send_message;
use sonettobuf::{CmdId, LoadFriendInfosReply};
use tokio::net::TcpStream;

pub async fn on_load_friend_infos(
    cmd_id: CmdId,
    socket: &mut TcpStream,
    _req: ClientPacket,
) -> Result<(), AppError> {
    let data = LoadFriendInfosReply {
        friend_ids: Vec::new(),
        black_list_ids: Vec::new(),
    };

    send_message(socket, cmd_id, data, 0).await?;
    Ok(())
}
