use crate::models::response::{JspLoginRsp, ZoneInfo};
use axum::response::Json;
use common::USER_ID;

pub async fn get() -> Json<JspLoginRsp> {
    let rsp = JspLoginRsp {
        area_id: 4,
        session_id: String::from("d1af95a2-1891-4fd4-8550-6cfa1e1012a5"),
        zone_info: ZoneInfo::zone_four(),
        user_name: JspLoginRsp::username(USER_ID),
        ..Default::default()
    };

    Json(rsp)
}
