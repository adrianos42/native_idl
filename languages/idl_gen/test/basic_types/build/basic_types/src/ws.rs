use ::idl_internal::{
    byteorder::{BigEndian, ReadBytesExt},
    futures::{stream::SplitSink, StreamExt},
    tokio::{
        self,
        net::TcpStream,
        sync::{oneshot, RwLock},
    },
    tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream},
    tungstenite::Message,
    url::Url,
    Uuid,
};
lazy_static! {
    pub(crate) static ref WS_INSTANCE: WsInstance = WsInstance::new();
}
#[derive(Default)]
pub(crate) struct WsInstance {
    pub(crate) write: ::std::sync::Arc<
        RwLock<Option<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>>>,
    >,
    pub(crate) dispatch:
        ::std::sync::Arc<RwLock<::std::collections::HashMap<Uuid, oneshot::Sender<Box<[u8]>>>>>,
}
impl WsInstance {
    pub(crate) async fn wait_context(&self) {
        while let None = self.write.read().await.as_ref() {}
    }
    fn new() -> Self {
        let context = Self::default();
        let context_lock = context.write.clone();
        tokio::spawn(async move {
            let url = Url::parse("ws://localhost:3012/socket").unwrap();
            let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
            let (write, read) = ws_stream.split();
            context_lock.write().await.replace(write);
            read.for_each(|message| async {
                match message {
                    Ok(msg) => match msg {
                        Message::Binary(data) => {
                            parse_response_bytes(&mut data.as_slice())
                                .await
                                .expect("Could not read response data");
                        }
                        Message::Ping(_) => {}
                        Message::Pong(_) => {}
                        Message::Close(_) => {}
                        _ => panic!("Invalid message type"),
                    },
                    Err(_err) => {}
                }
            })
            .await;
        });
        context
    }
}
async fn parse_response_bytes<R: ::std::io::Read>(_input: &mut R) -> Result<(), ::std::io::Error> {
    let mut _package_hash: [u8; 0x10] = [0x0; 0x10];
    _input.read_exact(&mut _package_hash[..])?;
    match _package_hash[..] {
        [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00] =>
        {
            let mut _hash: [u8; 0x10] = [0x0; 0x10];
            _input.read_exact(&mut _hash[..])?;
            let mut _interface_hash: [u8; 0x10] = [0x0; 0x10];
            _input.read_exact(&mut _interface_hash[..])?;
            let _call_id = Uuid::from_u128(_input.read_u128::<BigEndian>()?);
            let mut _response = Vec::new();
            _input.read_to_end(&mut _response).unwrap();
            match _hash[..] {
                [0x51, 0xd4, 0x78, 0x33, 0xc8, 0xde, 0xfa, 0x41, 0xb0, 0xbd, 0xf0, 0xf3, 0x2c, 0x0c, 0x24, 0xff] => {
                    match _interface_hash[..] {
                        [0x18, 0x24, 0x29, 0x1b, 0xd4, 0xfc, 0x67, 0x7a, 0xd5, 0x05, 0xb9, 0x11, 0x66, 0xb5, 0x51, 0xca]
                        | [0xca, 0x1d, 0x7f, 0x90, 0xbb, 0x1f, 0x8e, 0x73, 0x7c, 0x9a, 0xa2, 0x0b, 0x2e, 0xeb, 0x32, 0xd9] =>
                        {
                            let _sender = WS_INSTANCE
                                .dispatch
                                .write()
                                .await
                                .remove(&_call_id)
                                .expect("call id not found");
                            _sender.send(_response.into_boxed_slice()).unwrap();
                        }
                        _ => panic!("Invalid interface hash value"),
                    }
                }
                _ => panic!("Invalid library hash value"),
            }
        }
        _ => panic!("Invalid package hash value"),
    }
    Ok(())
}
