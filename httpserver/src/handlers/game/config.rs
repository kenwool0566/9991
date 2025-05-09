use crate::models::response::GameConfigRsp;
use axum::response::Json;
use common::cur_time_ms_u128;

pub async fn get() -> Json<GameConfigRsp> {
    let time = cur_time_ms_u128();
    let rsp = GameConfigRsp::with_timestamp(time);
    Json(rsp)
}
