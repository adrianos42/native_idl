#[macro_use]
extern crate lazy_static;
mod ws;
use ::idl_internal::{
    byteorder::{ReadBytesExt, WriteBytesExt},
    futures::SinkExt,
    Uuid,
};
use ::std::collections::HashMap;
use ::std::io::Read;
use ::std::io::Write;
#[derive(Debug, Clone)]
pub struct Point {
    pub x: i64,
    pub y: i64,
    pub name: String,
}
#[derive(Debug, Clone)]
pub enum Types {
    Integers(i64),
    Sttrings(String),
    Ntsst(Box<Point>),
}
#[derive(Debug, Copy, Clone)]
pub enum Names {
    Pamela,
    Bianca,
    Dominique,
    Stephany,
}
trait PointEx {
    fn from_bytes<R: ::std::io::Read>(_input: &mut R) -> Point;
    fn into_bytes<W: ::std::io::Write>(&self, _output: &mut W);
}
impl PointEx for Point {
    #[allow(unused_braces)]
    fn from_bytes<R: ::std::io::Read>(_input: &mut R) -> Point {
        Point {
            x: {
                _input
                    .read_i64::<::idl_internal::byteorder::BigEndian>()
                    .unwrap()
            },
            y: {
                _input
                    .read_i64::<::idl_internal::byteorder::BigEndian>()
                    .unwrap()
            },
            name: {
                let _length = _input
                    .read_i64::<::idl_internal::byteorder::BigEndian>()
                    .unwrap();
                let mut _data = vec![0; _length as usize];
                _input.read_exact(&mut _data[..]).unwrap();
                unsafe { String::from_utf8_unchecked(_data) }
            },
        }
    }
    #[allow(unused_braces)]
    fn into_bytes<W: ::std::io::Write>(&self, _output: &mut W) {
        _output
            .write_i64::<::idl_internal::byteorder::BigEndian>(self.x)
            .unwrap();
        _output
            .write_i64::<::idl_internal::byteorder::BigEndian>(self.y)
            .unwrap();
        let _length = self.name.len() as i64;
        let mut _data = self.name.as_bytes();
        _output
            .write_i64::<::idl_internal::byteorder::BigEndian>(_length)
            .unwrap();
        _output.write_all(&_data[..]).unwrap();
    }
}
trait TypesEx {
    fn from_bytes<R: ::std::io::Read>(_input: &mut R) -> Types;
    fn into_bytes<W: ::std::io::Write>(&self, _output: &mut W);
}
impl TypesEx for Types {
    #[allow(unused_braces)]
    #[allow(unused_parens)]
    fn into_bytes<W: ::std::io::Write>(&self, _output: &mut W) {
        match self {
            Types::Integers(_value_field) => {
                _output
                    .write_i64::<::idl_internal::byteorder::BigEndian>(0)
                    .unwrap();
                _output
                    .write_i64::<::idl_internal::byteorder::BigEndian>((*_value_field))
                    .unwrap();
            }
            Types::Sttrings(_value_field) => {
                _output
                    .write_i64::<::idl_internal::byteorder::BigEndian>(1)
                    .unwrap();
                let _length = (*_value_field).len() as i64;
                let mut _data = (*_value_field).as_bytes();
                _output
                    .write_i64::<::idl_internal::byteorder::BigEndian>(_length)
                    .unwrap();
                _output.write_all(&_data[..]).unwrap();
            }
            Types::Ntsst(_value_field) => {
                _output
                    .write_i64::<::idl_internal::byteorder::BigEndian>(2)
                    .unwrap();
                {
                    (*_value_field).into_bytes(_output);
                }
            }
        }
    }
    #[allow(unused_braces)]
    fn from_bytes<R: ::std::io::Read>(_input: &mut R) -> Types {
        let variant = _input
            .read_i64::<::idl_internal::byteorder::BigEndian>()
            .unwrap();
        match variant {
            0 => Types::Integers({
                _input
                    .read_i64::<::idl_internal::byteorder::BigEndian>()
                    .unwrap()
            }),
            1 => Types::Sttrings({
                let _length = _input
                    .read_i64::<::idl_internal::byteorder::BigEndian>()
                    .unwrap();
                let mut _data = vec![0; _length as usize];
                _input.read_exact(&mut _data[..]).unwrap();
                unsafe { String::from_utf8_unchecked(_data) }
            }),
            2 => Types::Ntsst(Box::new({ Point::from_bytes(_input) })),
            _ => panic!("Invalid variant value for type list"),
        }
    }
}
trait NamesEx {
    fn from_bytes<R: ::std::io::Read>(_input: &mut R) -> Names;
    fn into_bytes<W: ::std::io::Write>(&self, _output: &mut W);
}
impl NamesEx for Names {
    #[allow(unused_braces)]
    #[allow(unused_parens)]
    fn into_bytes<W: ::std::io::Write>(&self, _output: &mut W) {
        _output
            .write_i64::<::idl_internal::byteorder::BigEndian>(match self {
                Names::Pamela => 0,
                Names::Bianca => 1,
                Names::Dominique => 2,
                Names::Stephany => 3,
            })
            .unwrap()
    }
    #[allow(unused_braces)]
    fn from_bytes<R: ::std::io::Read>(_input: &mut R) -> Names {
        let variant = _input
            .read_i64::<::idl_internal::byteorder::BigEndian>()
            .unwrap();
        match variant {
            0 => Names::Pamela,
            1 => Names::Bianca,
            2 => Names::Dominique,
            3 => Names::Stephany,
            _ => panic!("Invalid variant value for enum"),
        }
    }
}
pub struct Test {
    instance_id: Uuid,
}
impl Test {
    #[allow(unused_braces)]
    pub async fn test_int(&mut self, value: i64) -> i64 {
        let (mut _input, _response_event) = Self::_write_prefix().await;
        _input
            .write_i64::<::idl_internal::byteorder::BigEndian>(
                ::idl_internal::MethodType::MethodCall as i64,
            )
            .unwrap();
        _input
            .write_u128::<::idl_internal::byteorder::BigEndian>(self.instance_id.as_u128())
            .unwrap();
        const _METHOD_HASH: [u8; 0x10] = [
            0x59, 0x70, 0xaa, 0x1a, 0x89, 0xd1, 0x70, 0x8d, 0x98, 0xca, 0x6a, 0xef, 0x85, 0x5a,
            0xc2, 0x9c,
        ];
        _input.write_all(&_METHOD_HASH[..]).unwrap();
        _input
            .write_i64::<::idl_internal::byteorder::BigEndian>(value)
            .unwrap();
        let mut _write_ref = crate::ws::WS_INSTANCE.write.write().await;
        let _write = _write_ref.as_mut().expect("Invalid locked value");
        _write
            .send(::idl_internal::tungstenite::Message::binary(
                _input.into_boxed_slice(),
            ))
            .await
            .unwrap();
        let _response = _response_event.await.unwrap();
        let mut _response_data = &_response[..];
        {
            _response_data
                .read_i64::<::idl_internal::byteorder::BigEndian>()
                .unwrap()
        }
    }
    #[allow(unused_braces)]
    pub async fn test_bool(&mut self, value: bool) -> bool {
        let (mut _input, _response_event) = Self::_write_prefix().await;
        _input
            .write_i64::<::idl_internal::byteorder::BigEndian>(
                ::idl_internal::MethodType::MethodCall as i64,
            )
            .unwrap();
        _input
            .write_u128::<::idl_internal::byteorder::BigEndian>(self.instance_id.as_u128())
            .unwrap();
        const _METHOD_HASH: [u8; 0x10] = [
            0xfa, 0x10, 0x2e, 0x9a, 0x94, 0x7a, 0xbb, 0xcd, 0x95, 0xd4, 0x14, 0x9e, 0x50, 0x45,
            0x75, 0xee,
        ];
        _input.write_all(&_METHOD_HASH[..]).unwrap();
        _input
            .write_i64::<::idl_internal::byteorder::BigEndian>(value as i64)
            .unwrap();
        let mut _write_ref = crate::ws::WS_INSTANCE.write.write().await;
        let _write = _write_ref.as_mut().expect("Invalid locked value");
        _write
            .send(::idl_internal::tungstenite::Message::binary(
                _input.into_boxed_slice(),
            ))
            .await
            .unwrap();
        let _response = _response_event.await.unwrap();
        let mut _response_data = &_response[..];
        {
            _response_data
                .read_i64::<::idl_internal::byteorder::BigEndian>()
                .unwrap()
                == 1
        }
    }
    #[allow(unused_braces)]
    pub async fn test_float(&mut self, value: f64) -> f64 {
        let (mut _input, _response_event) = Self::_write_prefix().await;
        _input
            .write_i64::<::idl_internal::byteorder::BigEndian>(
                ::idl_internal::MethodType::MethodCall as i64,
            )
            .unwrap();
        _input
            .write_u128::<::idl_internal::byteorder::BigEndian>(self.instance_id.as_u128())
            .unwrap();
        const _METHOD_HASH: [u8; 0x10] = [
            0x9c, 0xc0, 0x26, 0xde, 0xad, 0x44, 0x7c, 0xe0, 0xb8, 0xd5, 0x58, 0x9b, 0x1a, 0x0f,
            0xec, 0x66,
        ];
        _input.write_all(&_METHOD_HASH[..]).unwrap();
        _input
            .write_f64::<::idl_internal::byteorder::BigEndian>(value)
            .unwrap();
        let mut _write_ref = crate::ws::WS_INSTANCE.write.write().await;
        let _write = _write_ref.as_mut().expect("Invalid locked value");
        _write
            .send(::idl_internal::tungstenite::Message::binary(
                _input.into_boxed_slice(),
            ))
            .await
            .unwrap();
        let _response = _response_event.await.unwrap();
        let mut _response_data = &_response[..];
        {
            _response_data
                .read_f64::<::idl_internal::byteorder::BigEndian>()
                .unwrap()
        }
    }
    #[allow(unused_braces)]
    pub async fn test_string(&mut self, value: String) -> String {
        let (mut _input, _response_event) = Self::_write_prefix().await;
        _input
            .write_i64::<::idl_internal::byteorder::BigEndian>(
                ::idl_internal::MethodType::MethodCall as i64,
            )
            .unwrap();
        _input
            .write_u128::<::idl_internal::byteorder::BigEndian>(self.instance_id.as_u128())
            .unwrap();
        const _METHOD_HASH: [u8; 0x10] = [
            0x27, 0x30, 0xb4, 0x59, 0x4d, 0xe3, 0xa1, 0x4c, 0x62, 0xb6, 0x16, 0x3e, 0x48, 0x37,
            0x28, 0x67,
        ];
        _input.write_all(&_METHOD_HASH[..]).unwrap();
        let _length = value.len() as i64;
        let mut _data = value.as_bytes();
        _input
            .write_i64::<::idl_internal::byteorder::BigEndian>(_length)
            .unwrap();
        _input.write_all(&_data[..]).unwrap();
        let mut _write_ref = crate::ws::WS_INSTANCE.write.write().await;
        let _write = _write_ref.as_mut().expect("Invalid locked value");
        _write
            .send(::idl_internal::tungstenite::Message::binary(
                _input.into_boxed_slice(),
            ))
            .await
            .unwrap();
        let _response = _response_event.await.unwrap();
        let mut _response_data = &_response[..];
        {
            let _length = _response_data
                .read_i64::<::idl_internal::byteorder::BigEndian>()
                .unwrap();
            let mut _data = vec![0; _length as usize];
            _response_data.read_exact(&mut _data[..]).unwrap();
            unsafe { String::from_utf8_unchecked(_data) }
        }
    }
    #[allow(unused_braces)]
    pub async fn test_bytes(&mut self, value: Vec<u8>) -> Vec<u8> {
        let (mut _input, _response_event) = Self::_write_prefix().await;
        _input
            .write_i64::<::idl_internal::byteorder::BigEndian>(
                ::idl_internal::MethodType::MethodCall as i64,
            )
            .unwrap();
        _input
            .write_u128::<::idl_internal::byteorder::BigEndian>(self.instance_id.as_u128())
            .unwrap();
        const _METHOD_HASH: [u8; 0x10] = [
            0xd1, 0xd9, 0x32, 0x6d, 0xb2, 0xf3, 0x52, 0x41, 0x78, 0x7b, 0xa2, 0xd2, 0x36, 0xa9,
            0xdd, 0x24,
        ];
        _input.write_all(&_METHOD_HASH[..]).unwrap();
        let _length = value.len() as i64;
        _input
            .write_i64::<::idl_internal::byteorder::BigEndian>(_length)
            .unwrap();
        _input.write_all(&value[..]).unwrap();
        let mut _write_ref = crate::ws::WS_INSTANCE.write.write().await;
        let _write = _write_ref.as_mut().expect("Invalid locked value");
        _write
            .send(::idl_internal::tungstenite::Message::binary(
                _input.into_boxed_slice(),
            ))
            .await
            .unwrap();
        let _response = _response_event.await.unwrap();
        let mut _response_data = &_response[..];
        {
            let _length = _response_data
                .read_i64::<::idl_internal::byteorder::BigEndian>()
                .unwrap();
            let mut _data = vec![0; _length as usize];
            _response_data.read_exact(&mut _data[..]).unwrap();
            _data
        }
    }
    #[allow(unused_braces)]
    pub async fn test_uuid(&mut self, value: Uuid) -> Uuid {
        let (mut _input, _response_event) = Self::_write_prefix().await;
        _input
            .write_i64::<::idl_internal::byteorder::BigEndian>(
                ::idl_internal::MethodType::MethodCall as i64,
            )
            .unwrap();
        _input
            .write_u128::<::idl_internal::byteorder::BigEndian>(self.instance_id.as_u128())
            .unwrap();
        const _METHOD_HASH: [u8; 0x10] = [
            0xcd, 0xa7, 0x5e, 0x5f, 0x2b, 0xdd, 0xb4, 0x8a, 0xcf, 0x8c, 0x65, 0x37, 0x4f, 0xbe,
            0xcd, 0xdf,
        ];
        _input.write_all(&_METHOD_HASH[..]).unwrap();
        let mut _data = value.as_bytes().to_vec();
        _input.write_all(&_data[..]).unwrap();
        let mut _write_ref = crate::ws::WS_INSTANCE.write.write().await;
        let _write = _write_ref.as_mut().expect("Invalid locked value");
        _write
            .send(::idl_internal::tungstenite::Message::binary(
                _input.into_boxed_slice(),
            ))
            .await
            .unwrap();
        let _response = _response_event.await.unwrap();
        let mut _response_data = &_response[..];
        {
            let mut _data: [u8; 0x10] = [0x0; 0x10];
            _response_data.read_exact(&mut _data[..]).unwrap();
            Uuid::from_bytes(_data)
        }
    }
    #[allow(unused_braces)]
    pub async fn test_int_array(&mut self, value: Vec<i64>) -> Vec<i64> {
        let (mut _input, _response_event) = Self::_write_prefix().await;
        _input
            .write_i64::<::idl_internal::byteorder::BigEndian>(
                ::idl_internal::MethodType::MethodCall as i64,
            )
            .unwrap();
        _input
            .write_u128::<::idl_internal::byteorder::BigEndian>(self.instance_id.as_u128())
            .unwrap();
        const _METHOD_HASH: [u8; 0x10] = [
            0xb1, 0xd9, 0xaf, 0xc3, 0xcf, 0x41, 0x7f, 0xe2, 0xab, 0xd2, 0xb4, 0x6e, 0x3a, 0xce,
            0xec, 0x38,
        ];
        _input.write_all(&_METHOD_HASH[..]).unwrap();
        let _length = value.len() as i64;
        _input
            .write_i64::<::idl_internal::byteorder::BigEndian>(_length)
            .unwrap();
        for _array_item in value {
            _input
                .write_i64::<::idl_internal::byteorder::BigEndian>(_array_item)
                .unwrap();
        }
        let mut _write_ref = crate::ws::WS_INSTANCE.write.write().await;
        let _write = _write_ref.as_mut().expect("Invalid locked value");
        _write
            .send(::idl_internal::tungstenite::Message::binary(
                _input.into_boxed_slice(),
            ))
            .await
            .unwrap();
        let _response = _response_event.await.unwrap();
        let mut _response_data = &_response[..];
        {
            let _length = _response_data
                .read_i64::<::idl_internal::byteorder::BigEndian>()
                .unwrap();
            let mut _result = vec![];
            for _intex in 0.._length {
                _result.push({
                    _response_data
                        .read_i64::<::idl_internal::byteorder::BigEndian>()
                        .unwrap()
                });
            }
            _result
        }
    }
    #[allow(unused_braces)]
    pub async fn test_map(
        &mut self,
        value: ::std::collections::HashMap<String, f64>,
    ) -> ::std::collections::HashMap<String, f64> {
        let (mut _input, _response_event) = Self::_write_prefix().await;
        _input
            .write_i64::<::idl_internal::byteorder::BigEndian>(
                ::idl_internal::MethodType::MethodCall as i64,
            )
            .unwrap();
        _input
            .write_u128::<::idl_internal::byteorder::BigEndian>(self.instance_id.as_u128())
            .unwrap();
        const _METHOD_HASH: [u8; 0x10] = [
            0xb5, 0xf2, 0x8a, 0x24, 0xd9, 0x90, 0xd7, 0x3a, 0x6e, 0xfc, 0x61, 0x5c, 0x59, 0x91,
            0x0e, 0x7f,
        ];
        _input.write_all(&_METHOD_HASH[..]).unwrap();
        let _length = value.len() as i64;
        _input
            .write_i64::<::idl_internal::byteorder::BigEndian>(_length)
            .unwrap();
        for (_map_key, _map_data) in value {
            let _length = _map_key.len() as i64;
            let mut _data = _map_key.as_bytes();
            _input
                .write_i64::<::idl_internal::byteorder::BigEndian>(_length)
                .unwrap();
            _input.write_all(&_data[..]).unwrap();
            _input
                .write_f64::<::idl_internal::byteorder::BigEndian>(_map_data)
                .unwrap();
        }
        let mut _write_ref = crate::ws::WS_INSTANCE.write.write().await;
        let _write = _write_ref.as_mut().expect("Invalid locked value");
        _write
            .send(::idl_internal::tungstenite::Message::binary(
                _input.into_boxed_slice(),
            ))
            .await
            .unwrap();
        let _response = _response_event.await.unwrap();
        let mut _response_data = &_response[..];
        {
            let _length = _response_data
                .read_i64::<::idl_internal::byteorder::BigEndian>()
                .unwrap();
            let mut _result = ::std::collections::HashMap::new();
            for _intex in 0.._length {
                if let Some(_) = _result.insert(
                    {
                        let _length = _response_data
                            .read_i64::<::idl_internal::byteorder::BigEndian>()
                            .unwrap();
                        let mut _data = vec![0; _length as usize];
                        _response_data.read_exact(&mut _data[..]).unwrap();
                        unsafe { String::from_utf8_unchecked(_data) }
                    },
                    {
                        _response_data
                            .read_f64::<::idl_internal::byteorder::BigEndian>()
                            .unwrap()
                    },
                ) {
                    panic!()
                }
            }
            _result
        }
    }
    #[allow(unused_braces)]
    pub async fn test_static(value: i64) -> i64 {
        let (mut _input, _response_event) = Self::_write_prefix().await;
        _input
            .write_i64::<::idl_internal::byteorder::BigEndian>(
                ::idl_internal::MethodType::MethodCall as i64,
            )
            .unwrap();
        const _METHOD_HASH: [u8; 0x10] = [
            0x9f, 0x6a, 0x0f, 0x85, 0xf9, 0x4b, 0xcf, 0x83, 0x68, 0x9d, 0x21, 0x8f, 0x3f, 0x8c,
            0x70, 0xba,
        ];
        _input.write_all(&_METHOD_HASH[..]).unwrap();
        _input
            .write_i64::<::idl_internal::byteorder::BigEndian>(value)
            .unwrap();
        let mut _write_ref = crate::ws::WS_INSTANCE.write.write().await;
        let _write = _write_ref.as_mut().expect("Invalid locked value");
        _write
            .send(::idl_internal::tungstenite::Message::binary(
                _input.into_boxed_slice(),
            ))
            .await
            .unwrap();
        let _response = _response_event.await.unwrap();
        let mut _response_data = &_response[..];
        {
            _response_data
                .read_i64::<::idl_internal::byteorder::BigEndian>()
                .unwrap()
        }
    }
    #[allow(unused_braces)]
    pub async fn test_struct(&mut self, value: Point) -> Point {
        let (mut _input, _response_event) = Self::_write_prefix().await;
        _input
            .write_i64::<::idl_internal::byteorder::BigEndian>(
                ::idl_internal::MethodType::MethodCall as i64,
            )
            .unwrap();
        _input
            .write_u128::<::idl_internal::byteorder::BigEndian>(self.instance_id.as_u128())
            .unwrap();
        const _METHOD_HASH: [u8; 0x10] = [
            0xff, 0x21, 0xed, 0xba, 0x64, 0xfc, 0x05, 0x46, 0xee, 0xb9, 0x35, 0xf2, 0x8d, 0x7f,
            0x79, 0x7f,
        ];
        _input.write_all(&_METHOD_HASH[..]).unwrap();
        {
            value.into_bytes(&mut _input);
        }
        let mut _write_ref = crate::ws::WS_INSTANCE.write.write().await;
        let _write = _write_ref.as_mut().expect("Invalid locked value");
        _write
            .send(::idl_internal::tungstenite::Message::binary(
                _input.into_boxed_slice(),
            ))
            .await
            .unwrap();
        let _response = _response_event.await.unwrap();
        let mut _response_data = &_response[..];
        {
            Point::from_bytes(&mut _response_data)
        }
    }
    #[allow(unused_braces)]
    pub async fn test_types(&mut self, value: Types) -> Types {
        let (mut _input, _response_event) = Self::_write_prefix().await;
        _input
            .write_i64::<::idl_internal::byteorder::BigEndian>(
                ::idl_internal::MethodType::MethodCall as i64,
            )
            .unwrap();
        _input
            .write_u128::<::idl_internal::byteorder::BigEndian>(self.instance_id.as_u128())
            .unwrap();
        const _METHOD_HASH: [u8; 0x10] = [
            0xb0, 0xec, 0xd9, 0xc8, 0xe5, 0x92, 0x39, 0x43, 0xeb, 0x24, 0xd6, 0x12, 0x36, 0xc8,
            0x25, 0xe3,
        ];
        _input.write_all(&_METHOD_HASH[..]).unwrap();
        {
            value.into_bytes(&mut _input);
        }
        let mut _write_ref = crate::ws::WS_INSTANCE.write.write().await;
        let _write = _write_ref.as_mut().expect("Invalid locked value");
        _write
            .send(::idl_internal::tungstenite::Message::binary(
                _input.into_boxed_slice(),
            ))
            .await
            .unwrap();
        let _response = _response_event.await.unwrap();
        let mut _response_data = &_response[..];
        {
            Types::from_bytes(&mut _response_data)
        }
    }
    #[allow(unused_braces)]
    pub async fn test_enum(&mut self, value: Names) -> Names {
        let (mut _input, _response_event) = Self::_write_prefix().await;
        _input
            .write_i64::<::idl_internal::byteorder::BigEndian>(
                ::idl_internal::MethodType::MethodCall as i64,
            )
            .unwrap();
        _input
            .write_u128::<::idl_internal::byteorder::BigEndian>(self.instance_id.as_u128())
            .unwrap();
        const _METHOD_HASH: [u8; 0x10] = [
            0xcd, 0x89, 0x00, 0xb3, 0x2f, 0x24, 0xba, 0x07, 0xfd, 0x52, 0x50, 0x71, 0x32, 0xb1,
            0x69, 0x79,
        ];
        _input.write_all(&_METHOD_HASH[..]).unwrap();
        {
            value.into_bytes(&mut _input);
        }
        let mut _write_ref = crate::ws::WS_INSTANCE.write.write().await;
        let _write = _write_ref.as_mut().expect("Invalid locked value");
        _write
            .send(::idl_internal::tungstenite::Message::binary(
                _input.into_boxed_slice(),
            ))
            .await
            .unwrap();
        let _response = _response_event.await.unwrap();
        let mut _response_data = &_response[..];
        {
            Names::from_bytes(&mut _response_data)
        }
    }
    pub async fn new() -> Self {
        crate::ws::WS_INSTANCE.wait_context().await;
        let (mut input, response_event) = Self::_write_prefix().await;
        input
            .write_i64::<::idl_internal::byteorder::BigEndian>(
                ::idl_internal::MethodType::CreateInstance as i64,
            )
            .unwrap();
        let mut write_ref = crate::ws::WS_INSTANCE.write.write().await;
        let write = write_ref.as_mut().expect("Invalid locked value");
        write
            .send(::idl_internal::tungstenite::Message::binary(
                input.into_boxed_slice(),
            ))
            .await
            .unwrap();
        let response = response_event.await.unwrap();
        let mut response_data = &response[..];
        let call_id = Uuid::from_u128(
            response_data
                .read_u128::<::idl_internal::byteorder::BigEndian>()
                .unwrap(),
        );
        Self {
            instance_id: call_id,
        }
    }
    async fn _write_prefix() -> (
        Vec<u8>,
        ::idl_internal::tokio::sync::oneshot::Receiver<Box<[u8]>>,
    ) {
        let mut input: Vec<u8> = Vec::with_capacity(0x1000);
        const PACKAGE_HASH: [u8; 0x10] = [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00,
        ];
        input.write_all(&PACKAGE_HASH[..]).unwrap();
        const LIBRARY_HASH: [u8; 0x10] = [
            0x51, 0xd4, 0x78, 0x33, 0xc8, 0xde, 0xfa, 0x41, 0xb0, 0xbd, 0xf0, 0xf3, 0x2c, 0x0c,
            0x24, 0xff,
        ];
        input.write_all(&LIBRARY_HASH[..]).unwrap();
        const INTERFACE_HASH: [u8; 0x10] = [
            0x18, 0x24, 0x29, 0x1b, 0xd4, 0xfc, 0x67, 0x7a, 0xd5, 0x05, 0xb9, 0x11, 0x66, 0xb5,
            0x51, 0xca,
        ];
        input.write_all(&INTERFACE_HASH[..]).unwrap();
        let call_id = Uuid::new_v4();
        input
            .write_u128::<::idl_internal::byteorder::BigEndian>(call_id.as_u128())
            .unwrap();
        let (sender, response_event) = ::idl_internal::tokio::sync::oneshot::channel();
        let dispatch = crate::ws::WS_INSTANCE.dispatch.clone();
        dispatch.write().await.insert(call_id, sender);
        (input, response_event)
    }
}
pub struct TetSt {}
impl TetSt {
    #[allow(unused_braces)]
    pub async fn test_int(value: i64) -> i64 {
        let (mut _input, _response_event) = Self::_write_prefix().await;
        _input
            .write_i64::<::idl_internal::byteorder::BigEndian>(
                ::idl_internal::MethodType::MethodCall as i64,
            )
            .unwrap();
        const _METHOD_HASH: [u8; 0x10] = [
            0x35, 0xd1, 0xe3, 0x7c, 0xe0, 0x7e, 0x75, 0xe0, 0x1b, 0xe4, 0x34, 0xbc, 0xdd, 0x2e,
            0x45, 0xa1,
        ];
        _input.write_all(&_METHOD_HASH[..]).unwrap();
        _input
            .write_i64::<::idl_internal::byteorder::BigEndian>(value)
            .unwrap();
        let mut _write_ref = crate::ws::WS_INSTANCE.write.write().await;
        let _write = _write_ref.as_mut().expect("Invalid locked value");
        _write
            .send(::idl_internal::tungstenite::Message::binary(
                _input.into_boxed_slice(),
            ))
            .await
            .unwrap();
        let _response = _response_event.await.unwrap();
        let mut _response_data = &_response[..];
        {
            _response_data
                .read_i64::<::idl_internal::byteorder::BigEndian>()
                .unwrap()
        }
    }
    async fn _write_prefix() -> (
        Vec<u8>,
        ::idl_internal::tokio::sync::oneshot::Receiver<Box<[u8]>>,
    ) {
        let mut input: Vec<u8> = Vec::with_capacity(0x1000);
        const PACKAGE_HASH: [u8; 0x10] = [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00,
        ];
        input.write_all(&PACKAGE_HASH[..]).unwrap();
        const LIBRARY_HASH: [u8; 0x10] = [
            0x51, 0xd4, 0x78, 0x33, 0xc8, 0xde, 0xfa, 0x41, 0xb0, 0xbd, 0xf0, 0xf3, 0x2c, 0x0c,
            0x24, 0xff,
        ];
        input.write_all(&LIBRARY_HASH[..]).unwrap();
        const INTERFACE_HASH: [u8; 0x10] = [
            0xca, 0x1d, 0x7f, 0x90, 0xbb, 0x1f, 0x8e, 0x73, 0x7c, 0x9a, 0xa2, 0x0b, 0x2e, 0xeb,
            0x32, 0xd9,
        ];
        input.write_all(&INTERFACE_HASH[..]).unwrap();
        let call_id = Uuid::new_v4();
        input
            .write_u128::<::idl_internal::byteorder::BigEndian>(call_id.as_u128())
            .unwrap();
        let (sender, response_event) = ::idl_internal::tokio::sync::oneshot::channel();
        let dispatch = crate::ws::WS_INSTANCE.dispatch.clone();
        dispatch.write().await.insert(call_id, sender);
        (input, response_event)
    }
}
