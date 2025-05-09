use crate::models::response::{JspLoadZoneRsp, ZoneInfo, ZoneUserInfo};
use axum::response::Json;
use common::USERNAME;

pub async fn get() -> Json<JspLoadZoneRsp> {
    let rsp = JspLoadZoneRsp {
        last_login_zone_id: 4,
        recommend_zone_id: 4,
        result_code: 0,
        user_infos: vec![ZoneUserInfo {
            id: 4, // zone_id
            level: 1,
            name: String::from(USERNAME),
            portrait: 171504,
        }],
        zone_infos: vec![ZoneInfo::zone_four()],
    };

    Json(rsp)
}
