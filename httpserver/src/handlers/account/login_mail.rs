use crate::models::response::{AccountLoginRsp, AccountLoginRspData, AccountType, RealNameInfo};
use axum::response::Json;
use common::{USER_ID, USERNAME};

pub async fn post() -> Json<AccountLoginRsp> {
    let rsp = AccountLoginRsp {
        code: 200,
        msg: "success".to_string(),
        data: AccountLoginRspData {
            token: String::from("5c1ab43581bbed21ed0ddcab4711e7a4200"),
            expires_in: 596555,
            refresh_token: String::from("cf18850edcd7750539ee71a78c7d4f30200"),
            user_id: USER_ID,
            account_type: AccountType::Email,
            registration_account_type: AccountType::Email,
            account: format!(
                "{}****@gmail.com",
                USERNAME.chars().take(3).collect::<String>()
            ),
            real_name_info: RealNameInfo {
                need_real_name: false,
                real_name_status: true,
                age: 18,
                adult: true,
            },
            need_activate: false,
            cipher_mark: true,
            first_join: false,
            account_tags: String::new(),
        },
    };

    Json(rsp)
}
