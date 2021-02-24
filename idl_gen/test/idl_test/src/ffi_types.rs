use idl_internal::ffi::ffi_types::*;
use idl_test::idl_types;
#[repr(C)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}
impl Point {
    pub fn dispose() {}
}
impl From<idl_types::Point> for Point {
    #[allow(unused_braces)]
    fn from(value: idl_types::Point) -> Self {
        Self {
            x: { value.x } as i64,
            y: { value.y } as i64,
        }
    }
}
impl From<Point> for idl_types::Point {
    #[allow(unused_braces)]
    fn from(value: Point) -> Self {
        Self {
            x: { value.x } as i64,
            y: { value.y } as i64,
        }
    }
}
pub struct Types(AbiVariant);
impl Types {
    pub fn dispose(value: AbiVariant) {
        match value.index {
            0 => {}
            1 => {
                let mut _value_disps: Box<AbiString> =
                    unsafe { Box::from_raw(value.data as *mut AbiString) };
                _value_disps.dispose();
            }
            _ => panic!("Invalid variant value for type list"),
        }
    }
}
impl From<Types> for AbiVariant {
    fn from(value: Types) -> Self {
        value.0
    }
}
impl From<AbiVariant> for Types {
    fn from(value: AbiVariant) -> Self {
        Self(value)
    }
}
impl From<idl_types::Types> for Types {
    #[allow(unused_braces)]
    fn from(value: idl_types::Types) -> Self {
        let (_name, _data) = match value {
            idl_types::Types::IntType(_value_field) => {
                (
                    0 as i64,
                    { Box::into_raw(Box::new({ _value_field } as i64)) as *const i64 }
                        as *const ::core::ffi::c_void,
                )
            }
            idl_types::Types::StringType(_value_field) => (1 as i64, {
                Box::into_raw(Box::new({ AbiString::from(_value_field) })) as *const AbiString
            }
                as *const ::core::ffi::c_void),
        };
        AbiVariant {
            index: _name,
            data: _data,
        }
        .into()
    }
}
impl From<Types> for idl_types::Types {
    #[allow(unused_braces)]
    fn from(value: Types) -> Self {
        let value: AbiVariant = value.into();
        match value.index {
            0 => idl_types::Types::IntType({ unsafe { (value.data as *const i64).read() } } as i64),
            1 => idl_types::Types::StringType({
                unsafe { (value.data as *const AbiString).read() }.to_string()
            }),
            _ => panic!("Invalid variant value for type list"),
        }
    }
}
