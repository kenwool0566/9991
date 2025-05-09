use crate::models::request::GameC2SSessionReq;
use axum::extract::Json;
use axum::http::StatusCode;

pub async fn post(Json(_req): Json<GameC2SSessionReq>) -> StatusCode {
    StatusCode::ACCEPTED
}
