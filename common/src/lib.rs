pub const HOST: &str = "127.0.0.1";

pub const HTTPSERVER_PORT: u16 = 21000;
// you'll have to change the openssl.cnf when making the cert, if you want to change things above
// oh and, PEM
pub const KEY_FILE_PATH: &str = "./cert/localhost.key";
pub const CERT_FILE_PATH: &str = "./cert/localhost.crt";

pub const GAMESERVER_PORT: u16 = 23301;
pub const BUFFER_SIZE: usize = 1024;

pub const USER_ID: u64 = 7331;

pub fn init_tracing() {
    #[cfg(target_os = "windows")]
    ansi_term::enable_ansi_support().unwrap();

    tracing_subscriber::fmt().init();
}
