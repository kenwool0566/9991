use crate::crypto::encrypt_sdk;
use actix_web::{HttpResponse, Responder, get, post};
use common::{GAMESERVER_PORT, HOST, USER_ID};
use serde_json::json;

#[post("/sdk/init")]
pub async fn sdk_init() -> impl Responder {
    let data = json!({
        "code": 200,
        "msg": "success",
        "data": {
            "loginAccountTypes": [1, 5, 10, 11, 12, 13, 14, 15],
            "userCenterItems": [
                { "type": 1, "labTitle": "账号管理" },
                { "type": 2, "labTitle": "客服" },
                { "type": 3, "labTitle": "隐私条约" },
                { "type": 4, "labTitle": "账号注销" }
            ],
            "onlyMail": false,
            "gameChannel": {
                "gameId": 60001,
                "channelId": 200,
                "cpName": "重返未来：1999",
                "appId": "1",
                "appKey": "1",
                "callInterval": 600,
                "reloginInterval": 60,
                "reloginTimes": 5,
                "isRecordDebug": false
            },
            "bizSwitch": {
                "openRealNameWindow": false,
                "forceRealNameAuth": false
            },
            "isDownloadService": true,
            "isShowStopServiceBaffle": false,
            "isIgnoreFileMissing": true,
            "isOpenCMP": false,
            "showButtons": {
                "Notice": true
            },
            "isUnsupportChangeVolume": false,
        }
    });
    HttpResponse::Ok()
        .content_type("application/json")
        .body(encrypt_sdk(data))
}

#[post("/login/autologin")]
pub async fn login_autologin() -> impl Responder {
    let data = json!({
        "code": 200,
        "msg": "success",
        "data": {
            "token": "de7620137059cae84f2383565186f779200",
            "expiresIn": 583050,
            "refreshToken": "97cb7be15a73e5d63983be4ad19275b5200",
            "userId": USER_ID,
            "accountType": 15,
            "registrationAccountType": 15,
            "account": "kenwool",
            "realNameInfo": {
                "needRealName": false,
                "realNameStatus": true,
                "age": 18,
                "adult": true
            },
            "needActivate": false,
            "cipherMark": false,
            "firstJoin": false,
            "accountTags": ""
        }
    });
    HttpResponse::Ok()
        .content_type("application/json")
        .body(encrypt_sdk(data))
}

#[post("/login/mail")]
pub async fn login_mail() -> impl Responder {
    let data = json!({
        "code": 200,
        "msg": "success",
        "data": {
            "token": "de7620137059cae84f2383565186f779200",
            "expiresIn": 583050,
            "refreshToken": "97cb7be15a73e5d63983be4ad19275b5200",
            "userId": USER_ID,
            "accountType": 15,
            "registrationAccountType": 15,
            "account": "kenwool",
            "realNameInfo": {
                "needRealName": false,
                "realNameStatus": true,
                "age": 18,
                "adult": true
            },
            "needActivate": false,
            "cipherMark": false,
            "firstJoin": false,
            "accountTags": ""
        }
    });
    HttpResponse::Ok()
        .content_type("application/json")
        .body(encrypt_sdk(data))
}

#[post("/login/config")]
pub async fn login_config() -> impl Responder {
    let data = json!({
        "code": 200,
        "msg": "success",
        "data": {
            "afWhitelist": false
        }
    });
    HttpResponse::Ok()
        .content_type("application/json")
        .body(encrypt_sdk(data))
}

#[post("/uidAccount/bindList")]
pub async fn uid_account_bind_list() -> impl Responder {
    let data = json!({
        "code": 200,
        "msg": "success",
        "data": [{
            "userId": USER_ID,
            "account": "kenwool",
            "accountType": 15
        }]
    });
    HttpResponse::Ok()
        .content_type("application/json")
        .body(encrypt_sdk(data))
}

#[post("/login/verify")]
pub async fn login_verify() -> impl Responder {
    let data = json!({
        "code": 200,
        "msg": "success",
        "data": {
            "userInfo": {
                "channelId": 200,
                "userId": format!("200_{}", USER_ID),
                "realNameStatus": false,
                "age": 0,
                "adult": false,
                "firstJoin": false,
                "accountTags": "",
                "bindAccountTypeList": ["steam"],
                "firstJoinTime": "2024-04-12 19:40:54",
                "registerTime": "2024-04-12 19:39:07",
                "isPayAccount": false
            },
            "sessionId": "534f8be7e3744135a951a25f828c89bc",
            "token": "de7620137059cae84f2383565186f779200",
            "expiresIn": 604800,
            "refreshToken": "97cb7be15a73e5d63983be4ad19275b5200"
        }
    });
    HttpResponse::Ok()
        .content_type("application/json")
        .body(encrypt_sdk(data))
}

#[get("/login.jsp")]
pub async fn login_jsp() -> impl Responder {
    HttpResponse::Ok().json(json!({
        "accountTags": "",
        "areaId": 4,
        "isAdmin": false,
        "resultCode": 0,
        "sessionId": "b381a905-4ba9-4639-ae78-caf67859f16a",
        "userName": format!("200_200_{}", USER_ID),
        "zoneInfo": {
            "default": true,
            "id": 4,
            "name": "GL",
            "prefix": "",
            "state": 1
        }
    }))
}

#[get("/loadzone.jsp")]
pub async fn loadzone_jsp() -> impl Responder {
    HttpResponse::Ok().json(json!({
        "lastLoginZoneId": 4,
        "recommendZoneId": 4,
        "resultCode": 0,
        "userInfos": [{
            "id": 4,
            "level": 1,
            "name": "kenwool",
            "portrait": 170001
        }],
        "zoneInfos": [{
            "default": true,
            "prefix": "",
            "name": "GL",
            "id": 4,
            "state": 1
        }]
    }))
}

#[get("/startgame.jsp")]
pub async fn startgame_jsp() -> impl Responder {
    HttpResponse::Ok().json(json!({
        "bakIp": HOST,
        "bakPid": 1,
        "bakPort": GAMESERVER_PORT,
        "ip": HOST,
        "isAdmin": false,
        "pid": 1,
        "port": GAMESERVER_PORT,
        "resultCode": 0,
        "state": 1,
        "tips": "Maintenance time: 05:00 - 10:00, Apr.24th (UTC-5). For more info, please check Notice or our official X page."
    }))
}
