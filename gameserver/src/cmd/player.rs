use crate::DynError;
use crate::packet::ClientPacket;
use crate::util::{DAY_MS, YEAR_MS, send_message, send_raw_buffer, time_ms_u64};
use common::USER_ID;
use sonettobuf::{CmdId, GetPlayerInfoReply, HeroSimpleInfo, PlayerInfo};
// use sonettobuf::OpenInfo;
use tokio::net::TcpStream;

// placeholder for now
// should probably make em all their own functions and call in on_get_player_info
macro_rules! multi_send_test {
    ( $socket:expr; $($cmd:ident $data:expr;)* ) => {
        $(
            send_raw_buffer($socket, CmdId::$cmd, $data, 0).await?;
        )*
    };
}

pub async fn on_get_player_info(
    cmd_id: CmdId,
    socket: &mut TcpStream,
    _req: ClientPacket,
) -> Result<(), DynError> {
    let cur_time = time_ms_u64();

    let show_heros = vec![HeroSimpleInfo {
        hero_id: 3023,
        level: Some(1),
        rank: Some(1),
        ex_skill_level: Some(1),
        skin: None,
    }];

    let player_info = PlayerInfo {
        user_id: Some(USER_ID),
        name: Some(String::from("kenwool")),
        portrait: Some(170001),
        level: Some(1),
        exp: Some(0),
        signature: Some(String::from("I alone am the honored one.")),
        birthday: Some(String::from("")),
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
        character_age: Vec::new(),
    };

    let open_infos = Vec::new();

    let data = GetPlayerInfoReply {
        player_info: Some(player_info),
        openinfos: open_infos,
        can_rename: Some(true),
        main_thumbnail: Some(false),
        ext_rename: Some(0),
    };

    send_message(socket, cmd_id, data, 0).await?;

    multi_send_test![
        socket;
        GetSimplePropertyRequestCmd Vec::new();
        GetClothInfoRequestCmd Vec::new();
        HeroInfoListRequestCmd Vec::new();
        GetHeroGroupCommonListRequestCmd Vec::new();
        GetHeroGroupListRequestCmd Vec::new();
        GetItemListRequestCmd Vec::new();
        GetDungeonRequestCmd Vec::new();
        ReconnectFightRequestCmd Vec::new();
        GetBuyPowerInfoRequestCmd Vec::new();
        GetEquipInfoRequestCmd Vec::new();
        GetStoryRequestCmd Vec::new();
        GetChargeInfoRequestCmd Vec::new();
        GetMonthCardInfoRequestCmd Vec::new();
        // Skipping GetBlockPackageInfoRequsetCmd (not in proto)
        GetBuildingInfoRequestCmd Vec::new();
        GetCharacterInteractionInfoRequestCmd Vec::new();
        GetSummonInfoRequestCmd Vec::new();
        GetAchievementInfoRequestCmd Vec::new();
        GetDialogInfoRequestCmd Vec::new();
        GetAntiqueInfoRequestCmd Vec::new();
        GetWeekwalkInfoRequestCmd Vec::new();
        GetExploreSimpleInfoRequestCmd Vec::new();
        GetTowerInfoRequestCmd Vec::new();
        GetPlayerCardInfoRequestCmd Vec::new();
        LoadFriendInfosRequestCmd Vec::new();
        GetSignInInfoRequestCmd Vec::new();
        GetStoreInfosRequestCmd Vec::new();
        GetBgmInfoRequestCmd Vec::new();
        GetRoomObInfoRequestCmd Vec::new();
        CritterGetInfoRequestCmd Vec::new();
        GetAllMailsRequestCmd Vec::new();
        GetActivityInfosRequestCmd Vec::new();
        GetHandbookInfoRequestCmd Vec::new();
        GetRedDotInfosRequestCmd Vec::new();
        InstructionDungeonInfoRequestCmd Vec::new();
        GetBpInfoRequestCmd Vec::new();
        GetTurnbackInfoRequestCmd Vec::new();
        GetSettingInfosRequestCmd Vec::new();
        GetHeroStoryRequestCmd Vec::new();
        GetRoomPlanInfoRequestCmd Vec::new();
        Act160GetInfoRequestCmd Vec::new();
        GetAct186InfoRequestCmd Vec::new();
        GetAct147InfosRequestCmd Vec::new();
        Get101InfosRequestCmd Vec::new();
        GetTaskInfoRequestCmd Vec::new();
        SetSimplePropertyRequestCmd Vec::new();
        MarkMainThumbnailRequestCmd Vec::new();
        AutoUseExpirePowerItemRequestCmd Vec::new();
        SignInRequestCmd Vec::new();
        GetAssistBonusRequestCmd Vec::new();
        Get101BonusRequestCmd Vec::new();
        GetAct189InfoRequestCmd Vec::new();
    ];

    Ok(())
}
