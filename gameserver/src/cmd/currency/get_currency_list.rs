use crate::DynError;
use crate::packet::ClientPacket;
use crate::util::{DAY_MS, send_message, time_ms_u64};
use sonettobuf::{CmdId, Currency, GetCurrencyListReply, GetCurrencyListRequest};
use tokio::net::TcpStream;

pub async fn on_get_currency_list(
    cmd_id: CmdId,
    socket: &mut TcpStream,
    req: ClientPacket,
) -> Result<(), DynError> {
    let req = req.decode_message::<GetCurrencyListRequest>();
    let cur_time = time_ms_u64();
    let currency_list = req
        .currency_ids
        .iter()
        .map(|i| Currency {
            currency_id: Some(*i as u32),
            quantity: Some(10_000),
            last_recover_time: Some(cur_time),
            expired_time: Some(cur_time + DAY_MS),
        })
        .collect::<Vec<Currency>>();

    let data = GetCurrencyListReply { currency_list };

    send_message(socket, cmd_id, data, 0).await?;
    Ok(())
}
