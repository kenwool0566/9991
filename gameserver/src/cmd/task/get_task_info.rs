use crate::error::AppError;
use crate::packet::ClientPacket;
use crate::util::send_message;
use sonettobuf::{CmdId, GetTaskInfoReply};
// use sonettobuf::{Task, TaskActivityInfo};
// use sonettobuf::GetTaskInfoRequest;
use tokio::net::TcpStream;

pub async fn on_get_task_info(
    cmd_id: CmdId,
    socket: &mut TcpStream,
    _req: ClientPacket,
) -> Result<(), AppError> {
    let data = GetTaskInfoReply {
        task_info: Vec::new(),
        activity_info: Vec::new(),
        type_ids: Vec::new(),
    };

    send_message(socket, cmd_id, data, 0).await?;
    Ok(())
}
