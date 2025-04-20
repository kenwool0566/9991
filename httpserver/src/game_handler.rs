use actix_web::{get, post, HttpResponse, HttpRequest, Responder};
use serde_json::json;

#[get("/patch/{id}/version")]
pub async fn patch_version(_req: HttpRequest) -> impl Responder {
    // for some reason the game fucking breaks if you use the version in "id"
    // let id = req.match_info().get("id").unwrap();
    HttpResponse::Ok().json(json!({
        "latestVersion": "105.0.505",
        "appStatus": 0,
        "loginUri": "https://127.0.0.1:21000/login/mail",
        "loginUriBak": "https://127.0.0.1:21000/login/mail",
        "forceUpdate": 0
    }))
}

#[get("/config")]
pub async fn get_config() -> impl Responder {
    use std::time::{SystemTime, UNIX_EPOCH};

    let server_timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis())
        .unwrap_or(0);

    HttpResponse::Ok().json(json!({
        "code": 0,
        "data": {
            "server_timestamp": server_timestamp,
            "sync_batch_size": 30,
            "sync_interval": 30,
            "disable_event_list": ["summon_client", "ta_app_click", "ta_app_crash", "ta_app_view"]
        },
        "msg": ""
    }))
}

#[post("/v1.0/c2s/session/app/nativepc/60001")]
pub async fn post_session() -> impl Responder {
    HttpResponse::Ok()
}

#[post("/connecttest.txt")]
pub async fn post_connect_test() -> impl Responder {
    HttpResponse::Ok()
}

#[get("/resource/60001/check")]
pub async fn get_resource_check() -> impl Responder {
    HttpResponse::Ok().json(json!({
        "jp-opveract": {
            "res": [{
                "hash": "7a4c0c2258ca1d68d53a8e3d048c3b76",
                "name": "merge/672156d52d98282d694140e9e9f95cb0_1.zip",
                "length": 251673455,
                "order": 1
            }],
            "latest_ver": "101.61",
            "download_url": "https://optionalres-res-hw.sl916.com/uploadzip/60001/4/63",
            "download_url_bak": "https://optionalres-res-bak-hw.sl916.com/uploadzip/60001/4/63"
        },
        "media-opveract": {
            "res": [{
                "hash": "3fd0a756ff1b2c5436f08fabb53e96ce",
                "name": "merge/a98c0a7fd44c8a651635a2148b5b88e1_1.zip",
                "length": 159656415,
                "order": 1
            }],
            "latest_ver": "101.61",
            "download_url": "https://optionalres-res-hw.sl916.com/uploadzip/60001/4/63",
            "download_url_bak": "https://optionalres-res-bak-hw.sl916.com/uploadzip/60001/4/63"
        },
        "res-opveract": {
            "res": [{
                "hash": "55a35d4266dc6c6721d675ffd6a68f75",
                "name": "merge/8771121e8f317a7b85264c3dc56eb936_1.zip",
                "length": 407195487,
                "order": 1
            }],
            "latest_ver": "101.61",
            "download_url": "https://optionalres-res-hw.sl916.com/uploadzip/60001/4/63",
            "download_url_bak": "https://optionalres-res-bak-hw.sl916.com/uploadzip/60001/4/63"
        },
        "zh-opveract": {
            "res": [{
                "hash": "fdda88e76c640fce2e7f12d44579037c",
                "name": "merge/9c9e6f8519b991c05e15c45a1cf58af5_1.zip",
                "length": 248671453,
                "order": 1
            }],
            "latest_ver": "101.61",
            "download_url": "https://optionalres-res-hw.sl916.com/uploadzip/60001/4/63",
            "download_url_bak": "https://optionalres-res-bak-hw.sl916.com/uploadzip/60001/4/63"
        },
        "en-opveract": {
            "res": [{
                "hash": "8009fc8e8f86a9fd0df2c730e425469b",
                "name": "merge/eb8eaeda8bd6d236d8205d53207a47e3_1.zip",
                "length": 239985098,
                "order": 1
            }],
            "latest_ver": "101.61",
            "download_url": "https://optionalres-res-hw.sl916.com/uploadzip/60001/4/63",
            "download_url_bak": "https://optionalres-res-bak-hw.sl916.com/uploadzip/60001/4/63"
        },
        "kr-opveract": {
            "res": [{
                "hash": "944a2a9cdae41f6634b82a89f4d47f83",
                "name": "merge/98d751a3e389e2a13c6ad5cbe0bebdd1_1.zip",
                "length": 255090299,
                "order": 1
            }],
            "latest_ver": "101.61",
            "download_url": "https://optionalres-res-hw.sl916.com/uploadzip/60001/4/63",
            "download_url_bak": "https://optionalres-res-bak-hw.sl916.com/uploadzip/60001/4/63"
        }
    }))
}

#[get("/noticecp/client/query")]
pub async fn get_notice() -> impl Responder {
    HttpResponse::Ok().json(json!({
        "code": 200,
        "msg": "成功",
        "data": []
    }))
}
