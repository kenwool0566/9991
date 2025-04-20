use actix_web::{get, post, HttpResponse, Responder};
use serde_json::json;
use crate::crypto::encrypt;
use server_config::{HOST, GAMESERVER_PORT};

// anything 7331 is uid!

#[post("/sdk/init")]
pub async fn sdk_init() -> impl Responder {
    let data = json!({
        "code": 200,
        "msg": "success",
        "data": {
            "loginAccountTypes": [1, 5, 10, 11, 12, 13, 14],
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
            "isDownloadService": true
        }
    });
    HttpResponse::Ok().content_type("application/json").body(encrypt(data))
}

#[post("/login/autologin")]
pub async fn login_autologin() -> impl Responder {
    let data = json!({
        "code": 200,
        "msg": "success",
        "data": {
            "token": "kenwool0566@notareal.email",
            "expiresIn": 0,
            "refreshToken": "null",
            "userId": 7331,
            "accountType": 10,
            "registrationAccountType": 10,
            "account": "kenwool0566@notareal.email",
            "realNameInfo": {
                "needRealName": false,
                "realNameStatus": true,
                "age": 18,
                "adult": true
            },
            "needActivate": false,
            "cipherMark": true,
            "firstJoin": false,
            "accountTags": ""
        }
    });
    HttpResponse::Ok().content_type("application/json").body(encrypt(data))
}

#[post("/login/mail")]
pub async fn login_mail() -> impl Responder {
    let data = json!({
        "code": 200,
        "msg": "success",
        "data": {
            "token": "kenwool0566@notareal.email",
            "expiresIn": 0,
            "refreshToken": "null",
            "userId": 7331,
            "accountType": 10,
            "registrationAccountType": 10,
            "account": "kenwool0566@notareal.email",
            "realNameInfo": {
                "needRealName": false,
                "realNameStatus": true,
                "age": 18,
                "adult": true
            },
            "needActivate": false,
            "cipherMark": true,
            "firstJoin": false,
            "accountTags": ""
        }
    });
    HttpResponse::Ok().content_type("application/json").body(encrypt(data))
}

#[post("/uidAccount/bindList")]
pub async fn uid_account_bind_list() -> impl Responder {
    let data = json!({
        "code": 200,
        "msg": "success",
        "data": [{
            "userId": 7331,
            "account": "kenwool0566@notareal.email",
            "accountType": 10
        }]
    });
    HttpResponse::Ok().content_type("application/json").body(encrypt(data))
}

#[post("/login/verify")]
pub async fn login_verify() -> impl Responder {
    let data = json!({
        "code": 200,
        "msg": "success",
        "data": {
            "userInfo": {
                "channelId": 200,
                "userId": "200_7331",
                "realNameStatus": false,
                "age": 0,
                "adult": false,
                "firstJoin": false,
                "accountTags": "",
                "bindAccountTypeList": [10],
                "firstJoinTime": "2023-11-03 01:02:53",
                "registerTime": "2023-11-03 01:02:50",
                "isPayAccount": false
            },
            "sessionId": "null",
            "token": "null",
            "expiresIn": 0,
            "refreshToken": "null"
        }
    });
    HttpResponse::Ok().content_type("application/json").body(encrypt(data))
}

#[get("/login.jsp")]
pub async fn login_jsp() -> impl Responder {
    HttpResponse::Ok().json(json!({
        "accountTags": "",
        "areaId": 4,
        "isAdmin": false,
        "resultCode": 0,
        "sessionId": "null",
        "userName": "200_200_7331",
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
            "level": 9,
            "name": "kenwool0566@notareal.email",
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
        "bakPid": 2,
        "bakPort": GAMESERVER_PORT,
        "ip": HOST,
        "isAdmin": false,
        "pid": 1,
        "port": GAMESERVER_PORT,
        "resultCode": 0,
        "state": 1,
        "tips": "Everything is temporary. Have fun!"
    }))
}
