use std::fs::File;
use std::io::Read;
use std::sync::Arc;
use grpc::{ClientStub, ClientStubExt};
use tls_api::{Certificate, TlsAcceptorBuilder, TlsConnector, TlsConnectorBuilder};
use crate::account_grpc::AccountClient;

pub mod account;
pub mod account_grpc;


pub fn tls_connector(ssl_path: &str) -> tls_api_native_tls::TlsConnector {
    let mut f = File::open(ssl_path).unwrap();
    let mut root_ca: Vec<u8> = Vec::new();
    let result = f.read_to_end(&mut root_ca);
    match result {
        Ok(_) => {
            // let root_ca = include_bytes!("root-ca.der");
            let certificate = Certificate::from_der(root_ca);
            let mut builder = tls_api_native_tls::TlsConnector::builder().unwrap();
            builder
                .add_root_certificate(certificate)
                .expect("add_root_certificate");
            builder.build().unwrap()
        }
        Err(_) => {
            panic!("")
        }
    }
}

pub fn tls_acceptor(ssl_path: &str, password: &str) -> tls_api_native_tls::TlsAcceptor {
    let mut f = File::open(ssl_path).unwrap();
    let mut pkcs12: Vec<u8> = Vec::new();
    let result = f.read_to_end(&mut pkcs12);
    match result {
        Ok(_) => {
            let builder = tls_api_native_tls::TlsAcceptorBuilder::from_pkcs12(pkcs12.as_slice(), password).unwrap();
            builder.build().unwrap()
        }
        Err(_) => {
            panic!("")
        }
    }
}

pub fn new_account_client(tls: bool, account_grpc_address: &str, grpc_port: u16, ssl_path: &str) -> AccountClient {
    return if tls {
        // This is a bit complicated, because we need to explicitly pass root CA here
        // because server uses self-signed certificate.
        let tls_option = httpbis::ClientTlsOption::Tls(
            "foobar.com".to_owned(),
            Arc::new(tls_connector(ssl_path)),
        );
        let grpc_client = Arc::new(
            grpc::ClientBuilder::new(account_grpc_address, grpc_port)
                .explicit_tls(tls_option)
                .build()
                .unwrap(),
        );
        AccountClient::with_client(grpc_client)
    } else {
        let client_conf = Default::default();
        AccountClient::new_plain(account_grpc_address, grpc_port, client_conf).unwrap()
    };
}
