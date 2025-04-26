use crate::DynError;
use crate::cmd::*;
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
        // ===== common =====
        Ok(CmdId::GetServerTimeRequestCmd) => {
            common::on_get_server_time(CmdId::GetServerTimeRequestCmd, socket, req).await?
        }

        // ===== currency =====
        Ok(CmdId::GetCurrencyListRequestCmd) => {
            currency::on_get_currency_list(CmdId::GetCurrencyListRequestCmd, socket, req).await?
        }

        // ===== guide =====
        Ok(CmdId::GetGuideInfoRequestCmd) => {
            guide::on_get_guide_info(CmdId::GetGuideInfoRequestCmd, socket, req).await?
        }

        // ===== player =====
        Ok(CmdId::LoginCmd) => player::on_login(CmdId::LoginCmd, socket, req).await?,
        Ok(CmdId::GetPlayerInfoRequestCmd) => {
            player::on_get_player_info(CmdId::GetPlayerInfoRequestCmd, socket, req).await?
        }
        Ok(CmdId::LostCmd) => player::on_lost(CmdId::LostCmd, socket, req).await?,

        // ===== stat =====
        Ok(CmdId::UpdateClientStatBaseInfoRequestCmd) => {
            stat::on_update_client_stat_base_info(
                CmdId::UpdateClientStatBaseInfoRequestCmd,
                socket,
                req,
            )
            .await?
        }
        Ok(CmdId::ClientStatBaseInfoRequestCmd) => {
            stat::on_client_stat_base_info(CmdId::ClientStatBaseInfoRequestCmd, socket, req).await?
        }

        // ===== error handling =====
        Err(_) => return Err(format!("unknown cmd_id: {}", req.cmd_id).into()),
        _ => return Err(format!("unhandled: {}", req.cmd_id).into()),
    };

    Ok(())
}
