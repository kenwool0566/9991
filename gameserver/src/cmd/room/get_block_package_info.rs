use crate::error::AppError;
use crate::packet::ClientPacket;
use crate::util::send_message;
use sonettobuf::{CmdId, GetBlockPackageInfoReply};
// use sonettobuf::SpecialBlockInfo;
use tokio::net::TcpStream;

pub async fn on_get_block_package_info(
    cmd_id: CmdId,
    socket: &mut TcpStream,
    _req: ClientPacket,
) -> Result<(), AppError> {
    let data = GetBlockPackageInfoReply {
        block_package_ids: Vec::new(),
        special_blocks: Vec::new(),
    };

    send_message(socket, cmd_id, data, 0).await?;
    Ok(())
}
