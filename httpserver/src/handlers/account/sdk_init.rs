use crate::models::response::{
    AccountSdkInitRsp, AccountSdkInitRspData, AccountType, BizSwitch, GameChannel, ShowButtons,
    UserCenterItem,
};
use axum::{extract::Query, response::Json};
use std::collections::HashMap;

pub async fn post(query: Query<HashMap<String, String>>) -> Json<AccountSdkInitRsp> {
    let has_query = !query.is_empty();

    let data = if has_query {
        AccountSdkInitRspData {
            game_channel: Some(GameChannel {
                game_id: 60001,
                channel_id: 200,
                cp_name: "重返未来：1999".to_string(),
                app_id: "1".to_string(),
                app_key: "1".to_string(),
                call_interval: 600,
                relogin_interval: 60,
                relogin_times: 5,
                is_record_debug: false,
            }),
            biz_switch: Some(BizSwitch {
                open_real_name_window: false,
                force_real_name_auth: false,
            }),
            is_download_service: Some(true),
            is_show_stop_service_baffle: Some(false),
            is_ignore_file_missing: Some(false),
            is_open_c_m_p: Some(false),
            show_buttons: Some(ShowButtons { notice: true }),
            login_account_types: None,
            user_center_items: None,
            only_mail: None,
            is_unsupport_change_volume: false,
        }
    } else {
        AccountSdkInitRspData {
            login_account_types: Some(vec![AccountType::Email, AccountType::Bluepoch]),
            user_center_items: Some(vec![
                UserCenterItem {
                    r#type: 1,
                    lab_title: "账号管理".to_string(),
                },
                UserCenterItem {
                    r#type: 2,
                    lab_title: "客服".to_string(),
                },
                UserCenterItem {
                    r#type: 3,
                    lab_title: "隐私条约".to_string(),
                },
                UserCenterItem {
                    r#type: 4,
                    lab_title: "账号注销".to_string(),
                },
            ]),
            only_mail: Some(false),
            is_unsupport_change_volume: false,
            game_channel: None,
            biz_switch: None,
            is_download_service: None,
            is_show_stop_service_baffle: None,
            is_ignore_file_missing: None,
            is_open_c_m_p: None,
            show_buttons: None,
        }
    };

    let rsp = AccountSdkInitRsp {
        code: 200,
        msg: "success".to_string(),
        data,
    };

    Json(rsp)
}
