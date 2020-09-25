use super::ffi_types::*;

lazy_static! {
    static ref IDL_INTERFACE_STATIC: IdlInterfaceStatic = IdlInterfaceStatic::new();
}



pub struct IdlInterfaceStatic {
    pub(super) instance: Box<dyn super::idl_impl::IdlInterfaceStatic + Send + Sync>,
    pub(super) shit_stream: Option<extern "C" fn(i64, i64)>,
    pub(super) get_value_stream: Option<extern "C" fn(i64, *const super::idl_ffi_types::TypeNode)>,
}

impl IdlInterfaceStatic {
    fn new() -> Self {
        Self {
            instance: Box::new(super::ffi_impl_static::IdlInterfaceStatic::new()),
            shit_stream: None,
            get_value_stream: None,
        }
    }
}

pub struct IdlInterface {
    pub(super) instance: Box<dyn super::idl_impl::IdlInterface>,
}
impl From<Box<dyn super::idl_impl::IdlInterface>> for IdlInterface {
    fn from(value: Box<dyn super::idl_impl::IdlInterface>) -> Self {
        Self { instance: value }
    }
}
impl IdlInterfaceStatic {
    #[allow(unused_braces)]
    pub fn set_shit_stream(&self, handle: i64, value: i64) {
        let _result = { value } as i64;
        self.shit_stream.unwrap()(handle, _result);
    }
    #[allow(unused_braces)]
    pub fn set_get_value_stream(&self, handle: i64, value: super::idl_types::TypeNode) {
        let _result = {
            Box::into_raw(Box::new({ super::idl_ffi_types::TypeNode::from(value) }))
                as *const super::idl_ffi_types::TypeNode
        };
        self.get_value_stream.unwrap()(handle, _result);
    }
}
