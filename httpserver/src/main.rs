use actix_web::{App, HttpServer};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use server_config::{CERT_FILE_PATH, HOST, HTTPSERVER_PORT, KEY_FILE_PATH};

mod crypto;

mod account_handler;
mod game_handler;
mod index_handler;

#[actix_web::main]
async fn main() {
    let mut ssl = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    ssl.set_private_key_file(KEY_FILE_PATH, SslFiletype::PEM)
        .unwrap();
    ssl.set_certificate_chain_file(CERT_FILE_PATH).unwrap();

    let addr = format!("{}:{}", HOST, HTTPSERVER_PORT);
    println!("https://{} is listening :3", &addr);

    HttpServer::new(|| {
        App::new()
            .service(account_handler::sdk_init)
            .service(account_handler::login_autologin)
            .service(account_handler::login_mail)
            .service(account_handler::login_config)
            .service(account_handler::uid_account_bind_list)
            .service(account_handler::login_verify)
            .service(account_handler::login_jsp)
            .service(account_handler::loadzone_jsp)
            .service(account_handler::startgame_jsp)
            .service(game_handler::patch_version)
            .service(game_handler::get_config)
            .service(game_handler::post_session)
            .service(game_handler::post_connect_test)
            .service(game_handler::post_receive_app)
            .service(game_handler::get_resource_check)
            .service(game_handler::get_notice)
            .service(index_handler::home)
            .service(index_handler::favicon)
    })
    .bind_openssl(&addr, ssl)
    .unwrap()
    .run()
    .await
    .unwrap();
}
