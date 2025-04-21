use crate::DynError;
use crate::cmd::player;
use crate::packet::ClientPacket;
use sonettobuf::CmdId;
use tokio::net::TcpStream;

pub async fn dispatch_command(socket: &mut TcpStream, req: &[u8]) -> Result<(), DynError> {
    let req = match ClientPacket::decode(req) {
        Some(pk) => pk,
        None => return Err("failed decoding client packet".into()),
    };

    println!("got cmd: {}", req.cmd_id);

    match TryInto::<CmdId>::try_into(req.cmd_id as i32) {
        Ok(CmdId::LoginCmd) => player::on_request_login(CmdId::LoginCmd, socket, req).await?,

        Err(_) => return Err(format!("unknown cmd_id: {}", req.cmd_id).into()),
        _ => return Err(format!("unhandled: {}", req.cmd_id).into()),
    };

    Ok(())
}
