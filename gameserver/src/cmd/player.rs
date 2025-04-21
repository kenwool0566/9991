use crate::DynError;
use crate::packet::ClientPacket;
use crate::util::send_raw_buffer;
use byteorder::{BE, ByteOrder};
use sonettobuf::CmdId;
use tokio::net::TcpStream;

pub async fn on_request_login(
    cmd_id: CmdId,
    socket: &mut TcpStream,
    _req: ClientPacket,
) -> Result<(), DynError> {
    // let (account_info_len, account_info) = ByteWriting::read_string(&req.data, 0).unwrap();
    // let (account_token_len, account_token) =
    //     ByteWriting::read_string(&req.data, account_info_len + 2).unwrap();
    // let connect_way = &req.data[account_info_len + account_token_len + 4];
    // println!(
    //     "new login: info={}, token={}, connect_way={}",
    //     account_info, account_token, connect_way
    // );

    // let user_id = account_info
    //     .split('_')
    //     .last()
    //     .unwrap()
    //     .parse::<u64>()
    //     .unwrap();

    let mut data = vec![0u8; 10];
    BE::write_u64(&mut data[2..10], 7331);
    send_raw_buffer(socket, cmd_id, data, 0).await?;

    Ok(())
}
