use idl::analyzer::Analyzer;
use idl::ids;

use crate::rust::con_idl::{get_rust_ty_name, get_rust_ty_ref};
use crate::rust::layers::bytes::client::{package::BytesPackage, BytesInterface};
use crate::rust::string_pros::{StringPros, StringRustFmt};
use proc_macro2::{self, Literal, TokenStream};
use quote::format_ident;
use quote::{ToTokens, TokenStreamExt};
use std::{collections::HashMap, fmt};


pub(crate) mod layer;
pub(super) mod ws_mod;

#[derive(Debug)]
pub enum WSClientError {
    UnexpectedType,
    Undefined,
}

pub struct WSClient {
    module: Vec<TokenStream>,
}

impl ToTokens for WSClient {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(&self.module);
    }
}

impl fmt::Display for WSClient {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result_code = String::new();

        self.module.iter().for_each(|value| {
            result_code += &value.to_string();
        });

        write!(f, "{}", result_code.rust_fmt())
    }
}

impl WSClient {
    pub fn generate(package: &idl::ids::package::Package, analyzers: &[idl::analyzer::Analyzer]) -> Result<Self, WSClientError> {
        let mut context = Self::new();

        context.module.push(quote! {
            use ::idl_internal::{
                futures::{stream::SplitSink, StreamExt},
                tokio::{
                    self,
                    net::TcpStream, 
                    sync::{oneshot, RwLock}
                },
                byteorder::{BigEndian, ReadBytesExt},
                tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream},
                tungstenite::Message,
                url::Url,
                Uuid,
            };
        });

        context.add_main(package, analyzers)?;

        Ok(context)
    }

    fn add_main(&mut self, package: &idl::ids::package::Package, analyzers: &[idl::analyzer::Analyzer]) -> Result<(), WSClientError> {
        let url_lit = Literal::string("ws://localhost:3012/socket");

        let response_body = quote! {
            let _sender = WS_INSTANCE
                .dispatch
                .write()
                .await
                .remove(&_call_id)
                .expect("call id not found");
            _sender.send(_response.into_boxed_slice()).unwrap();
        };

        let ws_package = BytesPackage::generate(package, analyzers, response_body, true).unwrap();

        self.module.push(quote! {
            lazy_static! {
                pub(crate) static ref WS_INSTANCE: WsInstance = WsInstance::new();
            }

            #[derive(Default)]
            pub(crate) struct WsInstance {
                pub(crate) write: ::std::sync::Arc<RwLock<Option<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>>>>,
                pub(crate) dispatch: ::std::sync::Arc<RwLock<::std::collections::HashMap<Uuid, oneshot::Sender<Box<[u8]>>>>>,
            }

            impl WsInstance {
                pub(crate) async fn wait_context(&self) {
                    while let None = self.write.read().await.as_ref() {}
                }
            
                fn new() -> Self {
                    let context = Self::default();
                
                    let context_lock = context.write.clone();
                
                    tokio::spawn(async move {
                        let url = Url::parse(#url_lit).unwrap();
                    
                        let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
                    
                        let (write, read) = ws_stream.split();
                    
                        context_lock.write().await.replace(write);
                    
                        read.for_each(|message| async {
                            match message {
                                Ok(msg) => match msg {
                                    Message::Binary(data) => {
                                        parse_response_bytes(&mut data.as_slice()).await.expect("Could not read response data");
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

            #ws_package
        });

        Ok(())
    }

    fn new() -> Self {
        Self { module: vec![] }
    }
}

pub struct WSInterface {
    module: Vec<TokenStream>,
}

impl ToTokens for WSInterface {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(&self.module);
    }
}

impl fmt::Display for WSInterface {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result_code = String::new();

        self.module.iter().for_each(|value| {
            result_code += &value.to_string();
        });

        write!(f, "{}", result_code.rust_fmt())
    }
}

impl WSInterface {
    pub fn generate(package: &ids::package::Package, analyzer: &Analyzer) -> Result<Self, WSClientError> {
        let mut context = Self::new();

        let new_body = quote! {
            let mut write_ref = crate::ws::WS_INSTANCE.write.write().await;
            let write = write_ref.as_mut().expect("Invalid locked value");
            write.send(::idl_internal::tungstenite::Message::binary(input.into_boxed_slice())).await.unwrap();

            let response = response_event.await.unwrap();
            let mut response_data = &response[..];
        };
        let new_prefix_body = quote! { crate::ws::WS_INSTANCE.wait_context().await; };

        let write_prefix_type = quote! { ::idl_internal::tokio::sync::oneshot::Receiver<Box<[u8]>> };

        let write_prefix_body = quote! {
            let (sender, response_event) = ::idl_internal::tokio::sync::oneshot::channel();
            let dispatch = crate::ws::WS_INSTANCE.dispatch.clone();
            dispatch.write().await.insert(call_id, sender);
        };

        let method_body = quote! {
            let mut _write_ref = crate::ws::WS_INSTANCE.write.write().await;
            let _write = _write_ref.as_mut().expect("Invalid locked value");
            _write.send(::idl_internal::tungstenite::Message::binary(_input.into_boxed_slice())).await.unwrap();
            let _response = _response_event.await.unwrap();
            let mut _response_data = &_response[..];
        };

        let bytes_interface =
            BytesInterface::generate(
                package, 
                analyzer, 
                true, 
                new_body, 
                new_prefix_body,
                write_prefix_type,
                write_prefix_body,
                method_body,
            )
                .unwrap();

        context.module.push(bytes_interface.to_token_stream());

        Ok(context)
    }

    fn new() -> Self {
        Self { module: vec![] }
    }
}