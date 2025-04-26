use crate::DynError;
use crate::packet::ClientPacket;
use crate::util::{send_message, time_ms_u64};
use sonettobuf::{CmdId, GetServerTimeReply};
use tokio::net::TcpStream;

pub async fn on_get_server_time(
    cmd_id: CmdId,
    socket: &mut TcpStream,
    _req: ClientPacket,
) -> Result<(), DynError> {
    let data = GetServerTimeReply {
        server_time: Some(time_ms_u64()),
        ..Default::default()
    };

    send_message(socket, cmd_id, data, 0).await?;
    Ok(())
}
