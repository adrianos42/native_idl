use idl::analyzer::Analyzer;
use idl::idl_nodes::*;

use crate::rust::con_idl::{get_rust_ty_name, get_rust_ty_ref};

use crate::rust::string_pros::{StringPros, StringRustFmt};
use proc_macro2::{self, Literal, TokenStream};
use quote::format_ident;
use quote::{ToTokens, TokenStreamExt};
use std::{collections::HashMap, fmt};
use super::WSServerError;

pub struct WSInstance {
    module: Vec<TokenStream>,
}

impl ToTokens for WSInstance {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(&self.module);
    }
}

impl fmt::Display for WSInstance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result_code = String::new();

        self.module.iter().for_each(|value| {
            result_code += &value.to_string();
        });

        write!(f, "{}", result_code.rust_fmt())
    }
}

impl WSInstance {
    pub fn generate(
        package: &idl::ids::package::Package,
        analyzers: &[idl::analyzer::Analyzer],
    ) -> Result<Self, WSServerError> {
        let mut context = Self::new();

        let package_name = package.name();
        let package_ident = format_ident!("{}Package", package_name.to_pascal_case());

        context.module.push(quote! {
            use super::#package_ident;
            use ::idl_internal::{
                futures::{stream::SplitSink, SinkExt, StreamExt},
                tokio::{
                    self,
                    net::{TcpListener, TcpStream},
                    sync::{oneshot, RwLock},
                },
                tokio_tungstenite::{accept_async, WebSocketStream},
                tungstenite::{self, Message},
                Uuid,
            };
        });

        context.add_instance(package, analyzers)?;

        for analyzer in analyzers {
            for node in &analyzer.nodes {
                if let IdlNode::TypeInterface(value) = node { 
                    context.add_stream(value, analyzer)?;
                }
            }
        }

        Ok(context)
    }

    fn add_instance(
        &mut self,
        package: &idl::ids::package::Package,
        analyzers: &[idl::analyzer::Analyzer],
    ) -> Result<(), WSServerError> {
        let url_lit = Literal::string("127.0.0.1:3012");
        let buffer_capacity = quote! { 0x1000 };

        let package_name = package.name();
        let package_ident = format_ident!("{}Package", package_name.to_pascal_case());

        self.module.push(quote! {
            pub struct WsInstance {
                pub(crate) write: ::std::sync::Arc<RwLock<SplitSink<WebSocketStream<TcpStream>, Message>>>,
                pub(crate) dispatch: ::std::sync::Arc<RwLock<::std::collections::HashMap<Uuid, oneshot::Sender<Box<[u8]>>>>>,
                package_inst: #package_ident,                                
            }

            impl WsInstance {
                pub async fn run() {
                    let server = TcpListener::bind(#url_lit).await.unwrap();

                    while let Ok((stream, _)) = server.accept().await {
                        let peer = stream.peer_addr().expect("Expected peer addr");
                        tokio::spawn(async move {
                            if let Err(_err) = Self::handle_connection(peer, stream).await {}
                        });
                    }
                }

                async fn handle_connection(
                    _peer: ::std::net::SocketAddr,
                    stream: TcpStream,
                ) -> tungstenite::Result<()> {
                    let ws_stream = accept_async(stream)
                        .await
                        .expect("Error accepting connection");

                    let (write, mut read) = ws_stream.split();
                    let mut ws_instance = Self::new(write);

                    while let Some(message) = read.next().await {
                        match message? {
                            Message::Binary(data) => {
                                let mut output: Vec<u8> = Vec::with_capacity(#buffer_capacity);
                                match ws_instance
                                    .package_inst
                                    .parse_request_bytes(&mut data.as_slice(), &mut output)
                                {
                                    Ok(_) => {}
                                    Err(err) => {
                                        panic!();
                                    }
                                }

                                ws_instance
                                    .write
                                    .write()
                                    .await
                                    .send(tungstenite::Message::Binary(output))
                                    .await
                                    .unwrap();
                            }
                            Message::Ping(_) => {}
                            Message::Pong(_) => {}
                            Message::Close(_) => {}
                            _ => panic!("Invalid message type"),
                        }
                    }

                    Ok(())
                }

                fn new(write: SplitSink<WebSocketStream<TcpStream>, Message>) -> Self {
                    Self {
                        write: ::std::sync::Arc::new(RwLock::new(write)),
                        dispatch: ::std::sync::Arc::new(RwLock::new(::std::collections::HashMap::new())),
                        package_inst: #package_ident::new(),
                    }
                }
            }
        });

        Ok(())
    }

    fn add_stream(
        &mut self,
        ty_interface: &TypeInterface,
        analyzer: &Analyzer,
    ) -> Result<(), WSServerError> {
        let ident = &ty_interface.ident;

        let streams: Vec<(&TypeName, String)> = ty_interface
            .fields
            .iter()
            .filter_map(|node| match node {
                InterfaceNode::InterfaceField(field) => match &field.ty {
                    TypeName::TypeFunction(value) => match &value.return_ty {
                        TypeName::TypeStream(value) => Some((&value.s_ty, field.ident.to_owned())),
                        _ => None,
                    },
                    TypeName::TypeStream(value) => Some((&value.s_ty, field.ident.to_owned())),
                    _ => None,
                },
                _ => None,
            })
            .collect();

        let mut stream_im = vec![];
        for (_, name) in streams {
            let stream_name_ident = format_ident!("{}{}", &ident, name.to_pascal_case());
            stream_im.push(quote! {
                pub(super) struct #stream_name_ident {
                    pub(super) object_id: i64,
                    pub(super) handle: i64,
                }

                // impl Send for #stream_name_ident {} // TODO ??

                impl StreamInstance for #stream_name_ident {
                    fn wake_client(&self) {
                    }

                    fn get_id(&self) -> i64 {
                        self.object_id
                    }
                }
            });
        }

        self.module.push(quote! { #( #stream_im )* });


        Ok(())
    }

    fn new() -> Self {
        Self { module: vec![] }
    }
}
