#[macro_use]
extern crate lazy_static;
mod ws;

use ::async_stream::stream;
use ::byteorder::{ReadBytesExt, WriteBytesExt};
use ::futures::{
    stream::{Stream, StreamExt},
    SinkExt,
};
use ::idl_internal::{self, Uuid};
use ::std::collections::HashMap;
use ::std::io::Read;
use ::std::io::Write;

pub struct Test {
    instance_id: Uuid,
    stream_index: i64,
}

impl Test {
    #[allow(unused_braces)]
    pub async fn test_stream(&mut self) -> impl Stream<Item = i64> {
        let instance_id = self.instance_id;
        self.stream_index += 1;

        let mut _write_ref = crate::ws::WS_INSTANCE.write.write().await;
        let (_call_id, _write) = _write_ref.as_mut().expect("Invalid locked value");
        *_call_id += 1;

        let (mut _input, _response_event) =
            Self::_write_prefix(*_call_id, ::idl_internal::abi::MethodType::MethodCall).await;

        _input
            .write_u128::<::byteorder::BigEndian>(instance_id.as_u128())
            .unwrap();
            
        const _METHOD_HASH: [u8; 0x10] = [
            0x67, 0xba, 0xe3, 0x23, 0x06, 0x11, 0xfc, 0xe3, 0x88, 0x7b, 0x1e, 0x84, 0x88, 0x1d,
            0xcb, 0x08,
        ];
        _input.write_all(&_METHOD_HASH[..]).unwrap();

        _input
            .write_i64::<::byteorder::BigEndian>(idl_internal::abi::MethodCallType::Method as i64)
            .unwrap();

        _input
            .write_i64::<::byteorder::BigEndian>(self.stream_index)
            .unwrap();

        _input
            .write_i64::<::byteorder::BigEndian>(
                idl_internal::abi::AbiStreamReceiverState::Start.into(),
            )
            .unwrap();

        _write
            .send(::tungstenite::Message::binary(_input.into_boxed_slice()))
            .await
            .unwrap();

        let (sender, mut receiver) = tokio::sync::mpsc::channel::<Vec<u8>>(100);

        let _instance_streams = &mut crate::ws::WS_INSTANCE.stream_dispatch.write().await;
        let _instance_id = self.instance_id.as_u128();
        
        _instance_streams.get_mut(&_instance_id).unwrap().insert(self.stream_index, sender);

        let _ = _response_event.await.unwrap();

        stream! {
            while let Some(value) = receiver.recv().await {
                let mut _input = value.as_slice();
                let _object_id = _input.read_i64::<::byteorder::BigEndian>().unwrap();
                let _result = _input.read_i64::<::byteorder::BigEndian>().unwrap();
                yield _result;
            }
        }
    }

    pub async fn new() -> Self {
        crate::ws::WS_INSTANCE.wait_context().await;

        let mut write_ref = crate::ws::WS_INSTANCE.write.write().await;
        let (call_id, write) = write_ref.as_mut().expect("Invalid locked value");
        *call_id += 1;

        let (input, response_event) =
            Self::_write_prefix(*call_id, ::idl_internal::abi::MethodType::CreateInstance).await;

        write
            .send(::tungstenite::Message::binary(input.into_boxed_slice()))
            .await
            .unwrap();

        let response = response_event.await.unwrap();
        let mut response_sl = response.as_slice();

        match response_sl
            .read_i64::<::byteorder::BigEndian>()
            .unwrap()
            .into()
        {
            ::idl_internal::abi::AbiInternalError::Ok => Self {
                instance_id: Uuid::from_u128(
                    response_sl.read_u128::<::byteorder::BigEndian>().unwrap(),
                ),
                stream_index: 0,
            },
            _ => panic!("Could not create `Test` instance"),
        }
    }

    async fn _write_prefix(
        call_id: i64,
        method_type: ::idl_internal::abi::MethodType,
    ) -> (Vec<u8>, ::tokio::sync::oneshot::Receiver<Vec<u8>>) {
        let mut input: Vec<u8> = Vec::with_capacity(0x1000);
        const PACKAGE_HASH: [u8; 0x10] = [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00,
        ];
        input.write_all(&PACKAGE_HASH[..]).unwrap();
        const LIBRARY_HASH: [u8; 0x10] = [
            0x45, 0xe6, 0x1c, 0x9c, 0xbf, 0x6c, 0x8c, 0x5c, 0x7f, 0xa6, 0xd5, 0xe7, 0x79, 0x6a,
            0x91, 0x05,
        ];
        input.write_all(&LIBRARY_HASH[..]).unwrap();
        const INTERFACE_HASH: [u8; 0x10] = [
            0xa9, 0x88, 0xae, 0x33, 0x88, 0xba, 0x26, 0xdb, 0xa9, 0x20, 0xc4, 0xc2, 0x06, 0xbe,
            0xbd, 0xa2,
        ];
        input.write_all(&INTERFACE_HASH[..]).unwrap();
        input.write_i64::<::byteorder::BigEndian>(call_id).unwrap();
        input
            .write_i64::<::byteorder::BigEndian>(method_type.into())
            .unwrap();
        let (sender, response_event) = ::tokio::sync::oneshot::channel();
        let dispatch = crate::ws::WS_INSTANCE.dispatch.clone();
        dispatch.write().await.insert(call_id, sender);
        (input, response_event)
    }
}
