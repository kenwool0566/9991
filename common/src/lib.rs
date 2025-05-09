pub const HOST: &str = "127.0.0.1";
pub const DNS: &str = "localhost";
pub const HTTPSERVER_PORT: u16 = 21000;
pub const GAMESERVER_PORT: u16 = 23301;

pub const CERT_DIR: &str = "./cert";
pub const KEY_FILE_PATH: &str = "./cert/localhost.key";
pub const CERT_FILE_PATH: &str = "./cert/localhost.crt";

// todo: user regist n allat shit
pub const USER_ID: u64 = 7331;
pub const USERNAME: &str = "kenwool";

pub fn init_tracing() {
    #[cfg(target_os = "windows")]
    ansi_term::enable_ansi_support().unwrap();

    tracing_subscriber::fmt().init();
}

use std::time::{SystemTime, UNIX_EPOCH};

pub fn cur_time_ms_u128() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis())
        .unwrap_or(0)
}
