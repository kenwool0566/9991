use crate::cmd::*;
use crate::error::{AppError, CmdError};
use crate::packet::ClientPacket;
use sonettobuf::CmdId;
use tokio::net::TcpStream;

pub async fn dispatch_command(socket: &mut TcpStream, req: &[u8]) -> Result<(), AppError> {
    let req = ClientPacket::decode(req)?;

    let cmd_id = TryInto::<CmdId>::try_into(req.cmd_id as i32)
        .map_err(|_| AppError::Cmd(CmdError::UnregisteredCmd(req.cmd_id)))?;

    tracing::info!("Received Cmd: {:?}", cmd_id);

    match cmd_id {
        // ===== common =====
        CmdId::GetServerTimeRequestCmd => {
            common::on_get_server_time(CmdId::GetServerTimeRequestCmd, socket, req).await?
        }

        // ===== currency =====
        CmdId::GetCurrencyListRequestCmd => {
            currency::on_get_currency_list(CmdId::GetCurrencyListRequestCmd, socket, req).await?
        }

        // ===== guide =====
        CmdId::GetGuideInfoRequestCmd => {
            guide::on_get_guide_info(CmdId::GetGuideInfoRequestCmd, socket, req).await?
        }

        // ===== login =====
        CmdId::LoginRequestCmd => login::on_login(CmdId::LoginRequestCmd, socket, req).await?,
        CmdId::ReconnectRequestCmd => {
            login::on_reconnect(CmdId::ReconnectRequestCmd, socket, req).await?
        }

        // ===== player =====
        CmdId::GetPlayerInfoRequestCmd => {
            player::on_get_player_info(CmdId::GetPlayerInfoRequestCmd, socket, req).await?
        }

        // ===== stat =====
        CmdId::UpdateClientStatBaseInfoRequestCmd => {
            stat::on_update_client_stat_base_info(
                CmdId::UpdateClientStatBaseInfoRequestCmd,
                socket,
                req,
            )
            .await?
        }
        CmdId::ClientStatBaseInfoRequestCmd => {
            stat::on_client_stat_base_info(CmdId::ClientStatBaseInfoRequestCmd, socket, req).await?
        }

        v => return Err(AppError::Cmd(CmdError::UnhandledCmd(v))),
    };

    Ok(())
}
