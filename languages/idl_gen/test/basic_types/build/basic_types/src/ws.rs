use ::byteorder::{BigEndian, ReadBytesExt};
use ::futures::{stream::SplitSink, StreamExt};
use ::idl_internal::Uuid;
use ::tokio::{
    self,
    net::TcpStream,
    sync::{oneshot, RwLock},
};
use ::tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};
use ::tungstenite::Message;
use ::url::Url;

lazy_static! {
    pub(crate) static ref WS_INSTANCE: WsInstance = WsInstance::new();
}

#[derive(Default)]
pub(crate) struct WsInstance {
    pub(crate) write: ::std::sync::Arc<
        RwLock<
            Option<(
                i64,
                SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
            )>,
        >,
    >,
    pub(crate) dispatch:
        ::std::sync::Arc<RwLock<::std::collections::HashMap<i64, oneshot::Sender<Vec<u8>>>>>,
    pub(crate) stream_dispatch: ::std::sync::Arc<
        RwLock<
            ::std::collections::HashMap<
                u128,
                ::std::collections::HashMap<i64, tokio::sync::mpsc::Sender<Vec<u8>>>,
            >,
        >,
    >,
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
            context_lock.write().await.replace((0, write));
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
            let mut _library_hash: [u8; 0x10] = [0x0; 0x10];
            _input.read_exact(&mut _library_hash[..])?;

            match _library_hash[..] {
                [0x45, 0xe6, 0x1c, 0x9c, 0xbf, 0x6c, 0x8c, 0x5c, 0x7f, 0xa6, 0xd5, 0xe7, 0x79, 0x6a, 0x91, 0x05] =>
                {
                    let mut _interface_hash: [u8; 0x10] = [0x0; 0x10];
                    _input.read_exact(&mut _interface_hash[..])?;

                    match _interface_hash[..] {
                        [0xa9, 0x88, 0xae, 0x33, 0x88, 0xba, 0x26, 0xdb, 0xa9, 0x20, 0xc4, 0xc2, 0x06, 0xbe, 0xbd, 0xa2] => {
                            match _input.read_i64::<BigEndian>()?.into() {
                                ::idl_internal::abi::MethodType::CreateInstance
                                | ::idl_internal::abi::MethodType::DisposeInstance
                                | ::idl_internal::abi::MethodType::MethodCall => {
                                    let _call_id = _input.read_i64::<BigEndian>()?;
                                    let mut _response = Vec::new();
                                    _input.read_to_end(&mut _response).unwrap();
                                    let _sender = WS_INSTANCE
                                        .dispatch
                                        .write()
                                        .await
                                        .remove(&_call_id)
                                        .expect("`call id` not found");
                                    _sender.send(_response).unwrap();
                                }
                                ::idl_internal::abi::MethodType::StreamValue => {
                                    let _instance_id = _input.read_u128::<BigEndian>().unwrap();
                                    let _object_id = _input.read_i64::<BigEndian>().unwrap();
                                    let mut _response = Vec::new();
                                    _input.read_to_end(&mut _response).unwrap();

                                    let _sender = WS_INSTANCE
                                        .stream_dispatch
                                        .read()
                                        .await
                                        .get(&_instance_id)
                                        .and_then(|v| v.get(&_object_id))
                                        .unwrap()
                                        .clone();

                                    _sender.send(_response).await.unwrap();
                                }
                            }
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
