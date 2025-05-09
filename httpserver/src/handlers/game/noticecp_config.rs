use crate::models::response::GameNoticecpConfigRsp;
use axum::response::Json;

pub async fn get() -> Json<GameNoticecpConfigRsp> {
    let rsp = GameNoticecpConfigRsp {
        code: 200,
        msg: String::from("成功"),
        ..Default::default()
    };

    Json(rsp)
}
