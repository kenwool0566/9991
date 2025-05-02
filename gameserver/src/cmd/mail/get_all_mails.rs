use crate::error::AppError;
use crate::packet::ClientPacket;
use crate::util::send_message;
use sonettobuf::{CmdId, GetAllMailsReply};
// use sonettobuf::Mail;
use tokio::net::TcpStream;

pub async fn on_get_all_mails(
    cmd_id: CmdId,
    socket: &mut TcpStream,
    _req: ClientPacket,
) -> Result<(), AppError> {
    let data = GetAllMailsReply { mails: Vec::new() };

    send_message(socket, cmd_id, data, 0).await?;
    Ok(())
}
