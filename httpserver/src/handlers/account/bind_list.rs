use crate::models::response::{AccountBindListRsp, AccountBindListRspData, AccountType};
use axum::response::Json;
use common::{USER_ID, USERNAME};

pub async fn post() -> Json<AccountBindListRsp> {
    let rsp = AccountBindListRsp {
        code: 200,
        msg: "success".to_string(),
        data: vec![AccountBindListRspData {
            user_id: USER_ID,
            account: format!(
                "{}****@gmail.com",
                USERNAME.chars().take(3).collect::<String>()
            ),
            account_type: AccountType::Email,
        }],
    };

    Json(rsp)
}
