use crate::models::response::{AccountLoginConfigRsp, AccountLoginConfigRspData};
use axum::response::Json;

pub async fn post() -> Json<AccountLoginConfigRsp> {
    let rsp = AccountLoginConfigRsp {
        code: 200,
        msg: "success".to_string(),
        data: AccountLoginConfigRspData {
            af_whitelist: false,
        },
    };

    Json(rsp)
}
