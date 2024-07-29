use std::fs::File;
use std::io::{BufReader, Read};
use std::time::Duration;
use futures_util::{SinkExt, TryStreamExt};
use native_tls::{Certificate, Identity, TlsConnector};
use tokio::net::TcpStream;
use tokio_tungstenite::{client_async, client_async_tls, client_async_tls_with_config, connect_async, connect_async_tls_with_config, Connector};

#[tokio::main]
async fn main() {
    // let cx = TlsConnector::builder().danger_accept_invalid_certs(true).build().unwrap();
    // let cert = Certificate::from_pem(include_bytes!("../../cert.pem")).unwrap();
    // println!("{}", cert);
    let cert_file = File::open("./cert.pem").expect("Cannot open certificate file");
    let mut buf = Vec::new();
    BufReader::new(cert_file).read_to_end(&mut buf).unwrap();
    let cert = Certificate::from_pem(&buf).expect("Cannot parse certificate");

    let cx = TlsConnector::builder()
        // .disable_built_in_roots(true)
        // .danger_accept_invalid_certs(true)
        .disable_built_in_roots(true)
        .use_sni(true)
        .add_root_certificate(cert)
        .danger_accept_invalid_certs(true)
        .build().unwrap();


    let cx = tokio_native_tls::TlsConnector::from(cx);
    let stream = tokio::net::TcpStream::connect("127.0.0.1:3030").await.unwrap();
    let tls_stream = cx.connect("zkrush.com", stream).await.unwrap();
    let (mut ws_stream, _) = client_async_tls("ws://zkrush.com", tls_stream).await.unwrap();
    loop {
        ws_stream.send("hello".into()).await.unwrap();
        tokio::time::sleep(Duration::from_secs(5)).await;
    }

}
