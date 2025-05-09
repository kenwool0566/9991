use axum_server::tls_openssl::OpenSSLConfig;
use openssl::asn1::Asn1Time;
use openssl::bn::BigNum;
use openssl::hash::MessageDigest;
use openssl::nid::Nid;
use openssl::pkey::PKey;
use openssl::rsa::Rsa;
// use openssl::ssl::{Ssl, SslAcceptor, SslFiletype, SslMethod};
use openssl::x509::{X509, X509Extension, X509NameBuilder};

use std::fs::{self, File};
use std::io::{Write, stdin};
use std::path::Path;

use common::{CERT_DIR, CERT_FILE_PATH, DNS, HOST, KEY_FILE_PATH};

fn generate_cert() {
    let cert_dir = Path::new(CERT_DIR);

    if !cert_dir.exists() {
        fs::create_dir_all(cert_dir).unwrap();
    }

    let rsa = Rsa::generate(2048).unwrap();
    let pkey = PKey::from_rsa(rsa).unwrap();

    let mut x509_builder = X509::builder().unwrap();

    let serial = BigNum::from_u32(1).unwrap();
    let serial = serial.to_asn1_integer().unwrap();
    x509_builder.set_serial_number(&serial).unwrap();

    let not_before = Asn1Time::days_from_now(0).unwrap();
    let not_after = Asn1Time::days_from_now(365).unwrap();
    x509_builder.set_not_before(&not_before).unwrap();
    x509_builder.set_not_after(&not_after).unwrap();

    let mut name_builder = X509NameBuilder::new().unwrap();
    name_builder
        .append_entry_by_nid(Nid::COMMONNAME, HOST)
        .unwrap();
    let name = name_builder.build();
    x509_builder.set_subject_name(&name).unwrap();
    x509_builder.set_issuer_name(&name).unwrap();

    x509_builder.set_pubkey(&pkey).unwrap();

    #[allow(deprecated)]
    let subject_alt_name = X509Extension::new(
        None,
        None,
        "subjectAltName",
        &format!("IP:{},DNS:{}", HOST, DNS),
    )
    .unwrap();
    x509_builder.append_extension(subject_alt_name).unwrap();

    x509_builder.sign(&pkey, MessageDigest::sha256()).unwrap();

    let cert = x509_builder.build();
    let pem_cert = cert.to_pem().unwrap();
    let pem_key = pkey.private_key_to_pem_pkcs8().unwrap();

    File::create(KEY_FILE_PATH)
        .unwrap()
        .write_all(&pem_key)
        .unwrap();

    File::create(CERT_FILE_PATH)
        .unwrap()
        .write_all(&pem_cert)
        .unwrap();

    tracing::info!("Successfully generated certs in {} directory.", CERT_DIR);
}

// pub fn tls_builder() -> SslAcceptor {
//     let mut tls_builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();

//     tls_builder
//         .set_private_key_file(KEY_FILE_PATH, SslFiletype::PEM)
//         .unwrap();
//     tls_builder
//         .set_certificate_chain_file(CERT_FILE_PATH)
//         .unwrap();

//     tls_builder.check_private_key().unwrap();
//     tls_builder.build();
//     tls_builder
// }

pub fn check_cert_exists() {
    if !(Path::new(CERT_FILE_PATH).exists() && Path::new(KEY_FILE_PATH).exists()) {
        tracing::warn!("Certificate file is missing. Generating...");
        generate_cert();
        tracing::warn!(
            "Please install the certificate. Location: {}.",
            CERT_FILE_PATH
        );
        tracing::info!("Once the certificate is installed, press Enter to continue.");
        stdin().read_line(&mut String::with_capacity(1)).unwrap();
    }
}

pub fn get_openssl_config() -> OpenSSLConfig {
    OpenSSLConfig::from_pem_file(CERT_FILE_PATH, KEY_FILE_PATH).unwrap()
}
