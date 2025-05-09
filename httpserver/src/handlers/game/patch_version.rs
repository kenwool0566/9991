use crate::models::{request::GamePatchVersionReq, response::GamePatchVersionRsp};
use axum::extract::Query;
use axum::response::Json;

pub async fn get(Query(query): Query<GamePatchVersionReq>) -> Json<GamePatchVersionRsp> {
    let rsp = GamePatchVersionRsp::with_version(&query.version);
    Json(rsp)
}
