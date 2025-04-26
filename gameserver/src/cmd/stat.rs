use crate::DynError;
use crate::packet::ClientPacket;
use crate::util::{send_message, send_raw_buffer};
use sonettobuf::{CmdId, UpdateClientStatBaseInfoReply};
use tokio::net::TcpStream;

pub async fn on_update_client_stat_base_info(
    cmd_id: CmdId,
    socket: &mut TcpStream,
    _req: ClientPacket,
) -> Result<(), DynError> {
    let data = UpdateClientStatBaseInfoReply {
        account_bind_bonus: Some(0),
    };

    send_message(socket, cmd_id, data, 0).await?;
    Ok(())
}

pub async fn on_client_stat_base_info(
    cmd_id: CmdId,
    socket: &mut TcpStream,
    _req: ClientPacket,
) -> Result<(), DynError> {
    send_raw_buffer(socket, cmd_id, Vec::new(), 0).await?;
    Ok(())
}
