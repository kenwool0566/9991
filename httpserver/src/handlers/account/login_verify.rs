use crate::models::response::{AccountLoginVerifyRsp, AccountLoginVerifyRspData, VerifyUserInfo};
use axum::response::Json;
use common::{USER_ID, USERNAME};

pub async fn post() -> Json<AccountLoginVerifyRsp> {
    let rsp = AccountLoginVerifyRsp {
        code: 200,
        msg: "success".to_string(),
        data: AccountLoginVerifyRspData {
            user_info: VerifyUserInfo {
                channel_id: 200,
                user_id: VerifyUserInfo::user_id(USER_ID),
                real_name_status: false,
                age: 0,
                adult: false,
                account_tags: String::new(),
                // if steam, vec!["steam".to_string()]
                bind_account_type_list: vec![String::from(USERNAME)],
                first_join_time: String::from("2025-04-12 19:40:55"),
                register_time: String::from("2025-04-12 19:40:07"),
                is_pay_account: true,
                first_join: false,
            },
            session_id: String::from("e9d32a1a0495313d5048da7d55caecfa"),
            token: String::from("386055493a39b76c3efa09ef762b2419"),
            expires_in: 604800,
            refresh_token: String::from("d0c393c4025ba901603378a7d97e4c36"),
        },
    };

    Json(rsp)
}
