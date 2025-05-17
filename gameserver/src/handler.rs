use crate::bin_packet::get_packet_by_id;
use crate::error::{AppError, CmdError};
use sonettobuf::CmdId;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

pub async fn dispatch_command(socket: &mut TcpStream, req: &[u8]) -> Result<(), AppError> {
    let req = crate::packet::ClientPacket::decode(req)?;

    let cmd_id = TryInto::<CmdId>::try_into(req.cmd_id as i32)
        .map_err(|_| AppError::Cmd(CmdError::UnregisteredCmd(req.cmd_id)))?;

    tracing::info!("Received Cmd: {:?}", cmd_id);

    let Some(packet_data) = get_packet_by_id(req.cmd_id) else {
        return Err(AppError::Cmd(CmdError::UnhandledCmd(cmd_id)));
    };

    socket.write_all(&packet_data).await?;
    socket.flush().await?;

    Ok(())
}
