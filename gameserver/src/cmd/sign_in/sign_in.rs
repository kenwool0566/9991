use crate::error::AppError;
use crate::packet::ClientPacket;
use crate::util::send_message;
use sonettobuf::{CmdId, SignInReply};
use tokio::net::TcpStream;

pub async fn on_sign_in(
    cmd_id: CmdId,
    socket: &mut TcpStream,
    _req: ClientPacket,
) -> Result<(), AppError> {
    let data = SignInReply {
        day: None,
        birthday_hero_ids: Vec::new(),
    };

    send_message(socket, cmd_id, data, 0).await?;
    Ok(())
}
