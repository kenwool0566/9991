use crate::DynError;
use crate::packet::ClientPacket;
use crate::util::{DAY_MS, YEAR_MS, send_message, send_raw_buffer, time_ms_u64};
use byteorder::{BE, ByteOrder};
use server_config::USER_ID;
use sonettobuf::{CmdId, GetPlayerInfoReply, HeroSimpleInfo, OpenInfo, PlayerInfo};
use tokio::net::TcpStream;

pub async fn on_login(
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
    BE::write_u64(&mut data[2..10], USER_ID);
    send_raw_buffer(socket, cmd_id, data, 0).await?;

    Ok(())
}

pub async fn on_get_player_info(
    cmd_id: CmdId,
    socket: &mut TcpStream,
    _req: ClientPacket,
) -> Result<(), DynError> {
    let cur_time = time_ms_u64();

    let show_heros = vec![HeroSimpleInfo {
        hero_id: 3023,
        level: Some(60),
        rank: Some(3),
        ex_skill_level: Some(1),
        skin: None,
    }];

    let player_info = PlayerInfo {
        user_id: Some(USER_ID),
        name: Some(String::from("kenwool")),
        portrait: Some(1),
        level: Some(75),
        exp: Some(0),
        signature: Some(String::from("I alone am the honored one.")),
        birthday: None,
        show_heros,
        register_time: Some((cur_time - YEAR_MS) as i64),
        hero_rare_nn_count: Some(0),
        hero_rare_n_count: Some(0),
        hero_rare_r_count: Some(0),
        hero_rare_sr_count: Some(1),
        hero_rare_ssr_count: Some(0),
        last_episode_id: Some(1),
        last_login_time: Some((cur_time - DAY_MS) as i64),
        last_logout_time: Some((cur_time - (DAY_MS / 2)) as i64),
        character_age: vec![18],
    };

    let open_infos = vec![OpenInfo {
        id: USER_ID as i32,
        is_open: true,
    }];

    let data = GetPlayerInfoReply {
        player_info: Some(player_info),
        openinfos: open_infos,
        can_rename: Some(true),
        main_thumbnail: Some(true),
        ext_rename: Some(DAY_MS as i32),
    };

    send_message(socket, cmd_id, data, 0).await?;
    Ok(())
}

pub async fn on_lost(
    cmd_id: CmdId,
    socket: &mut TcpStream,
    _req: ClientPacket,
) -> Result<(), DynError> {
    send_raw_buffer(socket, cmd_id, vec![0x01], 0).await?;
    Ok(())
}
