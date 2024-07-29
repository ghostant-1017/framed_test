use std::net::SocketAddr;
use std::str::FromStr;
use tokio_native_tls::native_tls::TlsConnectorBuilder;

use tokio::net::TcpStream;
use tokio_tungstenite::{accept_async, MaybeTlsStream, WebSocketStream};
use native_tls::{Certificate, Identity, TlsAcceptor, TlsConnector};
use futures_util::stream::StreamExt;
use futures_util::sink::SinkExt;

#[tokio::main]
async fn main() {
    let bind_addr = SocketAddr::from_str("0.0.0.0:7777").unwrap();
    let listener = tokio::net::TcpListener::bind(bind_addr).await.unwrap();
    loop {
        match listener.accept().await {
            Ok((stream, peer)) => {
                tokio::spawn(async move {
                    if let Err(err) = handle_connection(stream, peer).await{
                        println!("{}", err);
                    }
                });
            }
            Err(err) => {
                println!("accept stream: {}", err);
            }
        }
    }

}

pub async fn handle_connection(stream: TcpStream, peer: SocketAddr) -> anyhow::Result<()>{
    println!("peer: {}", peer);
    let der = include_bytes!("../../identity.pfx");
    let cert = Identity::from_pkcs12(der, "1234")?;

    let tls_acceptor = tokio_native_tls::TlsAcceptor::from(native_tls::TlsAcceptor::builder(cert).build()?);
    let tls_stream = tls_acceptor.accept(stream).await?;
    println!("1111");

    // let ws_stream = tokio_tungstenite::MaybeTlsStream::NativeTls(tls_stream);

    // let tls_stream = tls_acceptor.accept(ws_stream).await?;
    println!("22222");
    let mut websocket = accept_async(tls_stream).await?;
    println!("33333");
    loop {
        let message = websocket.next().await.expect("")?;
        println!("{}", message);
    }

}

//
//
// pub struct MyCodec;
//
// pub struct Connection<Transport: AsyncRead + AsyncWrite> {
//     transport: Transport,
//     codec: MyCodec,
// }
//
// impl<Transport: AsyncRead + AsyncWrite> Connection<Transport> {
//     pub fn new(transport: Transport) -> Self {
//         Self {
//             transport,
//             codec: MyCodec,
//         }
//     }
// }
//

