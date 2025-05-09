use crate::models::response::JspStartGameRsp;
use axum::response::Json;
use common::{GAMESERVER_PORT, HOST};

pub async fn get() -> Json<JspStartGameRsp> {
    let rsp = JspStartGameRsp {
        bak_ip: String::from(HOST),
        bak_port: GAMESERVER_PORT,
        ip: String::from(HOST),
        port: GAMESERVER_PORT,
        state: 1,
        ..Default::default()
    };

    Json(rsp)
}
