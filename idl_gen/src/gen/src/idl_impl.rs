pub trait IdlInterface {
    fn shit(&mut self, stream_handle: i64);
    fn get_value(
        &mut self,
        value: super::idl_types::TypeStruct,
        name: Vec<String>,
        meh_not: f64,
        stream_handle: i64,
    );
    fn get_name(&mut self, name: String) -> i64;
}

pub trait IdlInterfaceStatic : Send {
    fn create(&self) -> Box<dyn super::idl_impl::IdlInterface>;
}
