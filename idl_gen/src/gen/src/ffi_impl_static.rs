pub struct IdlInterfaceStatic {}

impl IdlInterfaceStatic {
    pub(crate) fn new() -> Self {
        IdlInterfaceStatic {}
    }
}

impl super::idl_impl::IdlInterfaceStatic for IdlInterfaceStatic {
    fn create(&self) -> Box<dyn super::idl_impl::IdlInterface> {
        Box::new(IdlInterface::new())
    }
}

pub struct IdlInterface {

}

impl IdlInterface {
    fn new() -> Self {
        IdlInterface {}
    }
}

impl super::idl_impl::IdlInterface for IdlInterface {
    fn shit(&mut self, stream_handle: i64) {
        todo!()
    }

    fn get_value(
        &mut self,
        value: crate::idl_types::TypeStruct,
        name: Vec<String>,
        meh_not: f64,
        stream_handle: i64,
    ) {
        todo!()
    }

    fn get_name(&mut self, name: String) -> i64 {
        todo!()
    }
}