use super::ffi_types::*;

pub type Types = i64;
impl From<super::idl_types::Types> for Types {
    fn from(value: super::idl_types::Types) -> Self {
        match value {
            super::idl_types::Types::NatInt => 0,
            super::idl_types::Types::NatFloat => 1,
            super::idl_types::Types::NatString => 2,
            super::idl_types::Types::NatBytes => 3,
            super::idl_types::Types::NatBool => 4,
            super::idl_types::Types::NatNone => 5,
        }
    }
}
impl From<Types> for super::idl_types::Types {
    fn from(value: Types) -> Self {
        match value {
            0 => super::idl_types::Types::NatInt,
            1 => super::idl_types::Types::NatFloat,
            2 => super::idl_types::Types::NatString,
            3 => super::idl_types::Types::NatBytes,
            4 => super::idl_types::Types::NatBool,
            5 => super::idl_types::Types::NatNone,
            _ => panic!(),
        }
    }
}

pub type TypeNode = AbiVariant;
impl From<super::idl_types::TypeNode> for TypeNode {
    #[allow(unused_braces)]
    fn from(value: super::idl_types::TypeNode) -> Self {
        let (_name, _data) = match value {
            super::idl_types::TypeNode::LibraryName(_value_field) => (0 as i64, {
                Box::into_raw(Box::new({ AbiString::from(_value_field) })) as *const AbiString
            }
                as *const ::core::ffi::c_void),
            super::idl_types::TypeNode::Imports(_value_field) => (1 as i64, {
                Box::into_raw(Box::new({
                    let mut _array_value_items = vec![];
                    for _array_item in _value_field {
                        _array_value_items.push({ AbiString::from(_array_item) });
                    }
                    let mut _array = _array_value_items.into_boxed_slice();
                    let _inn_array = AbiArray {
                        length: _array.len() as i64,
                        data: _array.as_mut_ptr() as *const ::core::ffi::c_void,
                    };
                    std::mem::forget(_array);
                    _inn_array
                })) as *const AbiArray
            }
                as *const ::core::ffi::c_void),
            super::idl_types::TypeNode::Comment(_value_field) => (2 as i64, {
                Box::into_raw(Box::new({
                    let mut _array_value_items = vec![];
                    for _array_item in _value_field {
                        _array_value_items.push({ AbiString::from(_array_item) });
                    }
                    let mut _array = _array_value_items.into_boxed_slice();
                    let _inn_array = AbiArray {
                        length: _array.len() as i64,
                        data: _array.as_mut_ptr() as *const ::core::ffi::c_void,
                    };
                    std::mem::forget(_array);
                    _inn_array
                })) as *const AbiArray
            }
                as *const ::core::ffi::c_void),
            super::idl_types::TypeNode::InterfaceComment(_value_field) => (3 as i64, {
                Box::into_raw(Box::new({
                    let mut _array_value_items = vec![];
                    for _array_item in _value_field {
                        _array_value_items.push({ AbiString::from(_array_item) });
                    }
                    let mut _array = _array_value_items.into_boxed_slice();
                    let _inn_array = AbiArray {
                        length: _array.len() as i64,
                        data: _array.as_mut_ptr() as *const ::core::ffi::c_void,
                    };
                    std::mem::forget(_array);
                    _inn_array
                })) as *const AbiArray
            }
                as *const ::core::ffi::c_void),
            super::idl_types::TypeNode::StructComment(_value_field) => (4 as i64, {
                Box::into_raw(Box::new({
                    let mut _array_value_items = vec![];
                    for _array_item in _value_field {
                        _array_value_items.push({ AbiString::from(_array_item) });
                    }
                    let mut _array = _array_value_items.into_boxed_slice();
                    let _inn_array = AbiArray {
                        length: _array.len() as i64,
                        data: _array.as_mut_ptr() as *const ::core::ffi::c_void,
                    };
                    std::mem::forget(_array);
                    _inn_array
                })) as *const AbiArray
            }
                as *const ::core::ffi::c_void),
            super::idl_types::TypeNode::EnumComment(_value_field) => (5 as i64, {
                Box::into_raw(Box::new({
                    let mut _array_value_items = vec![];
                    for _array_item in _value_field {
                        _array_value_items.push({ AbiString::from(_array_item) });
                    }
                    let mut _array = _array_value_items.into_boxed_slice();
                    let _inn_array = AbiArray {
                        length: _array.len() as i64,
                        data: _array.as_mut_ptr() as *const ::core::ffi::c_void,
                    };
                    std::mem::forget(_array);
                    _inn_array
                })) as *const AbiArray
            }
                as *const ::core::ffi::c_void),
            super::idl_types::TypeNode::ConstComment(_value_field) => (6 as i64, {
                Box::into_raw(Box::new({
                    let mut _array_value_items = vec![];
                    for _array_item in _value_field {
                        _array_value_items.push({ AbiString::from(_array_item) });
                    }
                    let mut _array = _array_value_items.into_boxed_slice();
                    let _inn_array = AbiArray {
                        length: _array.len() as i64,
                        data: _array.as_mut_ptr() as *const ::core::ffi::c_void,
                    };
                    std::mem::forget(_array);
                    _inn_array
                })) as *const AbiArray
            }
                as *const ::core::ffi::c_void),
            super::idl_types::TypeNode::StreamComment(_value_field) => (7 as i64, {
                Box::into_raw(Box::new({
                    let mut _array_value_items = vec![];
                    for _array_item in _value_field {
                        _array_value_items.push({ AbiString::from(_array_item) });
                    }
                    let mut _array = _array_value_items.into_boxed_slice();
                    let _inn_array = AbiArray {
                        length: _array.len() as i64,
                        data: _array.as_mut_ptr() as *const ::core::ffi::c_void,
                    };
                    std::mem::forget(_array);
                    _inn_array
                })) as *const AbiArray
            }
                as *const ::core::ffi::c_void),
            super::idl_types::TypeNode::TypeListComment(_value_field) => (8 as i64, {
                Box::into_raw(Box::new({
                    let mut _array_value_items = vec![];
                    for _array_item in _value_field {
                        _array_value_items.push({ AbiString::from(_array_item) });
                    }
                    let mut _array = _array_value_items.into_boxed_slice();
                    let _inn_array = AbiArray {
                        length: _array.len() as i64,
                        data: _array.as_mut_ptr() as *const ::core::ffi::c_void,
                    };
                    std::mem::forget(_array);
                    _inn_array
                })) as *const AbiArray
            }
                as *const ::core::ffi::c_void),
            super::idl_types::TypeNode::TypeStruct(_value_field) => (9 as i64, {
                Box::into_raw(Box::new({ TypeStruct::from(*_value_field) })) as *const TypeStruct
            }
                as *const ::core::ffi::c_void),
            super::idl_types::TypeNode::TypeStream(_value_field) => (10 as i64, {
                Box::into_raw(Box::new({ TypeStream::from(*_value_field) })) as *const TypeStream
            }
                as *const ::core::ffi::c_void),
            super::idl_types::TypeNode::TypeEnum(_value_field) => (11 as i64, {
                Box::into_raw(Box::new({ TypeEnum::from(*_value_field) })) as *const TypeEnum
            }
                as *const ::core::ffi::c_void),
            super::idl_types::TypeNode::TypeList(_value_field) => (12 as i64, {
                Box::into_raw(Box::new({ TypeList::from(*_value_field) })) as *const TypeList
            }
                as *const ::core::ffi::c_void),
            super::idl_types::TypeNode::TypeConst(_value_field) => (13 as i64, {
                Box::into_raw(Box::new({ TypeConst::from(*_value_field) })) as *const TypeConst
            }
                as *const ::core::ffi::c_void),
            super::idl_types::TypeNode::TypeInterface(_value_field) => (14 as i64, {
                Box::into_raw(Box::new({ TypeInterface::from(*_value_field) }))
                    as *const TypeInterface
            }
                as *const ::core::ffi::c_void),
        };
        AbiVariant {
            index: _name,
            data: _data,
        }
    }
}
impl From<TypeNode> for super::idl_types::TypeNode {
    #[allow(unused_braces)]
    fn from(value: TypeNode) -> Self {
        match value.index {
            0 => super::idl_types::TypeNode::LibraryName({
                unsafe { (value.data as *const AbiString).read() }.to_string()
            }),
            1 => super::idl_types::TypeNode::Imports({
                let _array = unsafe { (value.data as *const AbiArray).read() };
                let _data = _array.data as *const AbiString;
                let _length = _array.length as isize;
                let mut _array_vec = vec![];
                for _index in 0.._length {
                    let _array_item = unsafe { _data.offset(_index).read() };
                    _array_vec.push({ _array_item.to_string() });
                }
                _array_vec
            }),
            2 => super::idl_types::TypeNode::Comment({
                let _array = unsafe { (value.data as *const AbiArray).read() };
                let _data = _array.data as *const AbiString;
                let _length = _array.length as isize;
                let mut _array_vec = vec![];
                for _index in 0.._length {
                    let _array_item = unsafe { _data.offset(_index).read() };
                    _array_vec.push({ _array_item.to_string() });
                }
                _array_vec
            }),
            3 => super::idl_types::TypeNode::InterfaceComment({
                let _array = unsafe { (value.data as *const AbiArray).read() };
                let _data = _array.data as *const AbiString;
                let _length = _array.length as isize;
                let mut _array_vec = vec![];
                for _index in 0.._length {
                    let _array_item = unsafe { _data.offset(_index).read() };
                    _array_vec.push({ _array_item.to_string() });
                }
                _array_vec
            }),
            4 => super::idl_types::TypeNode::StructComment({
                let _array = unsafe { (value.data as *const AbiArray).read() };
                let _data = _array.data as *const AbiString;
                let _length = _array.length as isize;
                let mut _array_vec = vec![];
                for _index in 0.._length {
                    let _array_item = unsafe { _data.offset(_index).read() };
                    _array_vec.push({ _array_item.to_string() });
                }
                _array_vec
            }),
            5 => super::idl_types::TypeNode::EnumComment({
                let _array = unsafe { (value.data as *const AbiArray).read() };
                let _data = _array.data as *const AbiString;
                let _length = _array.length as isize;
                let mut _array_vec = vec![];
                for _index in 0.._length {
                    let _array_item = unsafe { _data.offset(_index).read() };
                    _array_vec.push({ _array_item.to_string() });
                }
                _array_vec
            }),
            6 => super::idl_types::TypeNode::ConstComment({
                let _array = unsafe { (value.data as *const AbiArray).read() };
                let _data = _array.data as *const AbiString;
                let _length = _array.length as isize;
                let mut _array_vec = vec![];
                for _index in 0.._length {
                    let _array_item = unsafe { _data.offset(_index).read() };
                    _array_vec.push({ _array_item.to_string() });
                }
                _array_vec
            }),
            7 => super::idl_types::TypeNode::StreamComment({
                let _array = unsafe { (value.data as *const AbiArray).read() };
                let _data = _array.data as *const AbiString;
                let _length = _array.length as isize;
                let mut _array_vec = vec![];
                for _index in 0.._length {
                    let _array_item = unsafe { _data.offset(_index).read() };
                    _array_vec.push({ _array_item.to_string() });
                }
                _array_vec
            }),
            8 => super::idl_types::TypeNode::TypeListComment({
                let _array = unsafe { (value.data as *const AbiArray).read() };
                let _data = _array.data as *const AbiString;
                let _length = _array.length as isize;
                let mut _array_vec = vec![];
                for _index in 0.._length {
                    let _array_item = unsafe { _data.offset(_index).read() };
                    _array_vec.push({ _array_item.to_string() });
                }
                _array_vec
            }),
            9 => super::idl_types::TypeNode::TypeStruct(Box::new({
                super::idl_types::TypeStruct::from(unsafe {
                    (value.data as *const TypeStruct).read()
                })
            })),
            10 => super::idl_types::TypeNode::TypeStream(Box::new({
                super::idl_types::TypeStream::from(unsafe {
                    (value.data as *const TypeStream).read()
                })
            })),
            11 => super::idl_types::TypeNode::TypeEnum(Box::new({
                super::idl_types::TypeEnum::from(unsafe { (value.data as *const TypeEnum).read() })
            })),
            12 => super::idl_types::TypeNode::TypeList(Box::new({
                super::idl_types::TypeList::from(unsafe { (value.data as *const TypeList).read() })
            })),
            13 => super::idl_types::TypeNode::TypeConst(Box::new({
                super::idl_types::TypeConst::from(unsafe {
                    (value.data as *const TypeConst).read()
                })
            })),
            14 => super::idl_types::TypeNode::TypeInterface(Box::new({
                super::idl_types::TypeInterface::from(unsafe {
                    (value.data as *const TypeInterface).read()
                })
            })),
            _ => panic!(),
        }
    }
}

#[repr(C)]
pub struct TypeInterface {
    pub ident: *const AbiString,
    pub fields: *const AbiArray,
}
impl From<super::idl_types::TypeInterface> for TypeInterface {
    #[allow(unused_braces)]
    fn from(value: super::idl_types::TypeInterface) -> Self {
        Self {
            ident: {
                Box::into_raw(Box::new({ AbiString::from(value.ident) })) as *const AbiString
            },
            fields: {
                Box::into_raw(Box::new({
                    let mut _array_value_items = vec![];
                    for _array_item in value.fields {
                        _array_value_items.push({ InterfaceNode::from(_array_item) });
                    }
                    let mut _array = _array_value_items.into_boxed_slice();
                    let _inn_array = AbiArray {
                        length: _array.len() as i64,
                        data: _array.as_mut_ptr() as *const ::core::ffi::c_void,
                    };
                    std::mem::forget(_array);
                    _inn_array
                })) as *const AbiArray
            },
        }
    }
}
impl From<TypeInterface> for super::idl_types::TypeInterface {
    #[allow(unused_braces)]
    fn from(value: TypeInterface) -> Self {
        Self {
            ident: { unsafe { (value.ident as *const AbiString).read() }.to_string() },
            fields: {
                let _array = unsafe { (value.fields as *const AbiArray).read() };
                let _data = _array.data as *const InterfaceNode;
                let _length = _array.length as isize;
                let mut _array_vec = vec![];
                for _index in 0.._length {
                    let _array_item = unsafe { _data.offset(_index).read() };
                    _array_vec.push({ super::idl_types::InterfaceNode::from(_array_item) });
                }
                _array_vec
            },
        }
    }
}

pub type InterfaceNode = AbiVariant;
impl From<super::idl_types::InterfaceNode> for InterfaceNode {
    #[allow(unused_braces)]
    fn from(value: super::idl_types::InterfaceNode) -> Self {
        let (_name, _data) = match value {
            super::idl_types::InterfaceNode::InterfaceField(_value_field) => (0 as i64, {
                Box::into_raw(Box::new({ InterfaceField::from(*_value_field) }))
                    as *const InterfaceField
            }
                as *const ::core::ffi::c_void),
            super::idl_types::InterfaceNode::Comment(_value_field) => (1 as i64, {
                Box::into_raw(Box::new({
                    let mut _array_value_items = vec![];
                    for _array_item in _value_field {
                        _array_value_items.push({ AbiString::from(_array_item) });
                    }
                    let mut _array = _array_value_items.into_boxed_slice();
                    let _inn_array = AbiArray {
                        length: _array.len() as i64,
                        data: _array.as_mut_ptr() as *const ::core::ffi::c_void,
                    };
                    std::mem::forget(_array);
                    _inn_array
                })) as *const AbiArray
            }
                as *const ::core::ffi::c_void),
        };
        AbiVariant {
            index: _name,
            data: _data,
        }
    }
}
impl From<InterfaceNode> for super::idl_types::InterfaceNode {
    #[allow(unused_braces)]
    fn from(value: InterfaceNode) -> Self {
        match value.index {
            0 => super::idl_types::InterfaceNode::InterfaceField(Box::new({
                super::idl_types::InterfaceField::from(unsafe {
                    (value.data as *const InterfaceField).read()
                })
            })),
            1 => super::idl_types::InterfaceNode::Comment({
                let _array = unsafe { (value.data as *const AbiArray).read() };
                let _data = _array.data as *const AbiString;
                let _length = _array.length as isize;
                let mut _array_vec = vec![];
                for _index in 0.._length {
                    let _array_item = unsafe { _data.offset(_index).read() };
                    _array_vec.push({ _array_item.to_string() });
                }
                _array_vec
            }),
            _ => panic!(),
        }
    }
}

#[repr(C)]
pub struct InterfaceField {
    pub attributes: *const AbiArray,
    pub ident: *const AbiString,
    pub ty: *const TypeName,
}
impl From<super::idl_types::InterfaceField> for InterfaceField {
    #[allow(unused_braces)]
    fn from(value: super::idl_types::InterfaceField) -> Self {
        Self {
            attributes: {
                Box::into_raw(Box::new({
                    let mut _array_value_items = vec![];
                    for _array_item in value.attributes {
                        _array_value_items.push({ Attributes::from(_array_item) });
                    }
                    let mut _array = _array_value_items.into_boxed_slice();
                    let _inn_array = AbiArray {
                        length: _array.len() as i64,
                        data: _array.as_mut_ptr() as *const ::core::ffi::c_void,
                    };
                    std::mem::forget(_array);
                    _inn_array
                })) as *const AbiArray
            },
            ident: {
                Box::into_raw(Box::new({ AbiString::from(value.ident) })) as *const AbiString
            },
            ty: { Box::into_raw(Box::new({ TypeName::from(value.ty) })) as *const TypeName },
        }
    }
}
impl From<InterfaceField> for super::idl_types::InterfaceField {
    #[allow(unused_braces)]
    fn from(value: InterfaceField) -> Self {
        Self {
            attributes: {
                let _array = unsafe { (value.attributes as *const AbiArray).read() };
                let _data = _array.data as *const Attributes;
                let _length = _array.length as isize;
                let mut _array_vec = vec![];
                for _index in 0.._length {
                    let _array_item = unsafe { _data.offset(_index).read() };
                    _array_vec.push({ super::idl_types::Attributes::from(_array_item) });
                }
                _array_vec
            },
            ident: { unsafe { (value.ident as *const AbiString).read() }.to_string() },
            ty: {
                super::idl_types::TypeName::from(unsafe { (value.ty as *const TypeName).read() })
            },
        }
    }
}

#[repr(C)]
pub struct TypeStruct {
    pub ident: *const AbiString,
    pub fields: *const AbiArray,
}
impl From<super::idl_types::TypeStruct> for TypeStruct {
    #[allow(unused_braces)]
    fn from(value: super::idl_types::TypeStruct) -> Self {
        Self {
            ident: {
                Box::into_raw(Box::new({ AbiString::from(value.ident) })) as *const AbiString
            },
            fields: {
                Box::into_raw(Box::new({
                    let mut _array_value_items = vec![];
                    for _array_item in value.fields {
                        _array_value_items.push({ StructNode::from(_array_item) });
                    }
                    let mut _array = _array_value_items.into_boxed_slice();
                    let _inn_array = AbiArray {
                        length: _array.len() as i64,
                        data: _array.as_mut_ptr() as *const ::core::ffi::c_void,
                    };
                    std::mem::forget(_array);
                    _inn_array
                })) as *const AbiArray
            },
        }
    }
}
impl From<TypeStruct> for super::idl_types::TypeStruct {
    #[allow(unused_braces)]
    fn from(value: TypeStruct) -> Self {
        Self {
            ident: { unsafe { (value.ident as *const AbiString).read() }.to_string() },
            fields: {
                let _array = unsafe { (value.fields as *const AbiArray).read() };
                let _data = _array.data as *const StructNode;
                let _length = _array.length as isize;
                let mut _array_vec = vec![];
                for _index in 0.._length {
                    let _array_item = unsafe { _data.offset(_index).read() };
                    _array_vec.push({ super::idl_types::StructNode::from(_array_item) });
                }
                _array_vec
            },
        }
    }
}

pub type StructNode = AbiVariant;
impl From<super::idl_types::StructNode> for StructNode {
    #[allow(unused_braces)]
    fn from(value: super::idl_types::StructNode) -> Self {
        let (_name, _data) = match value {
            super::idl_types::StructNode::StructField(_value_field) => (0 as i64, {
                Box::into_raw(Box::new({ StructField::from(*_value_field) })) as *const StructField
            }
                as *const ::core::ffi::c_void),
            super::idl_types::StructNode::Comment(_value_field) => (1 as i64, {
                Box::into_raw(Box::new({
                    let mut _array_value_items = vec![];
                    for _array_item in _value_field {
                        _array_value_items.push({ AbiString::from(_array_item) });
                    }
                    let mut _array = _array_value_items.into_boxed_slice();
                    let _inn_array = AbiArray {
                        length: _array.len() as i64,
                        data: _array.as_mut_ptr() as *const ::core::ffi::c_void,
                    };
                    std::mem::forget(_array);
                    _inn_array
                })) as *const AbiArray
            }
                as *const ::core::ffi::c_void),
        };
        AbiVariant {
            index: _name,
            data: _data,
        }
    }
}
impl From<StructNode> for super::idl_types::StructNode {
    #[allow(unused_braces)]
    fn from(value: StructNode) -> Self {
        match value.index {
            0 => super::idl_types::StructNode::StructField(Box::new({
                super::idl_types::StructField::from(unsafe {
                    (value.data as *const StructField).read()
                })
            })),
            1 => super::idl_types::StructNode::Comment({
                let _array = unsafe { (value.data as *const AbiArray).read() };
                let _data = _array.data as *const AbiString;
                let _length = _array.length as isize;
                let mut _array_vec = vec![];
                for _index in 0.._length {
                    let _array_item = unsafe { _data.offset(_index).read() };
                    _array_vec.push({ _array_item.to_string() });
                }
                _array_vec
            }),
            _ => panic!(),
        }
    }
}

#[repr(C)]
pub struct StructField {
    pub ident: *const AbiString,
    pub ty: *const TypeName,
}
impl From<super::idl_types::StructField> for StructField {
    #[allow(unused_braces)]
    fn from(value: super::idl_types::StructField) -> Self {
        Self {
            ident: {
                Box::into_raw(Box::new({ AbiString::from(value.ident) })) as *const AbiString
            },
            ty: { Box::into_raw(Box::new({ TypeName::from(value.ty) })) as *const TypeName },
        }
    }
}
impl From<StructField> for super::idl_types::StructField {
    #[allow(unused_braces)]
    fn from(value: StructField) -> Self {
        Self {
            ident: { unsafe { (value.ident as *const AbiString).read() }.to_string() },
            ty: {
                super::idl_types::TypeName::from(unsafe { (value.ty as *const TypeName).read() })
            },
        }
    }
}

#[repr(C)]
pub struct TypeStream {
    pub ident: *const AbiString,
    pub fields: *const AbiArray,
}
impl From<super::idl_types::TypeStream> for TypeStream {
    #[allow(unused_braces)]
    fn from(value: super::idl_types::TypeStream) -> Self {
        Self {
            ident: {
                Box::into_raw(Box::new({ AbiString::from(value.ident) })) as *const AbiString
            },
            fields: {
                Box::into_raw(Box::new({
                    let mut _array_value_items = vec![];
                    for _array_item in value.fields {
                        _array_value_items.push({ StreamNode::from(_array_item) });
                    }
                    let mut _array = _array_value_items.into_boxed_slice();
                    let _inn_array = AbiArray {
                        length: _array.len() as i64,
                        data: _array.as_mut_ptr() as *const ::core::ffi::c_void,
                    };
                    std::mem::forget(_array);
                    _inn_array
                })) as *const AbiArray
            },
        }
    }
}
impl From<TypeStream> for super::idl_types::TypeStream {
    #[allow(unused_braces)]
    fn from(value: TypeStream) -> Self {
        Self {
            ident: { unsafe { (value.ident as *const AbiString).read() }.to_string() },
            fields: {
                let _array = unsafe { (value.fields as *const AbiArray).read() };
                let _data = _array.data as *const StreamNode;
                let _length = _array.length as isize;
                let mut _array_vec = vec![];
                for _index in 0.._length {
                    let _array_item = unsafe { _data.offset(_index).read() };
                    _array_vec.push({ super::idl_types::StreamNode::from(_array_item) });
                }
                _array_vec
            },
        }
    }
}

pub type StreamNode = AbiVariant;
impl From<super::idl_types::StreamNode> for StreamNode {
    #[allow(unused_braces)]
    fn from(value: super::idl_types::StreamNode) -> Self {
        let (_name, _data) = match value {
            super::idl_types::StreamNode::StreamField(_value_field) => (0 as i64, {
                Box::into_raw(Box::new({ StreamField::from(*_value_field) })) as *const StreamField
            }
                as *const ::core::ffi::c_void),
            super::idl_types::StreamNode::Comment(_value_field) => (1 as i64, {
                Box::into_raw(Box::new({
                    let mut _array_value_items = vec![];
                    for _array_item in _value_field {
                        _array_value_items.push({ AbiString::from(_array_item) });
                    }
                    let mut _array = _array_value_items.into_boxed_slice();
                    let _inn_array = AbiArray {
                        length: _array.len() as i64,
                        data: _array.as_mut_ptr() as *const ::core::ffi::c_void,
                    };
                    std::mem::forget(_array);
                    _inn_array
                })) as *const AbiArray
            }
                as *const ::core::ffi::c_void),
        };
        AbiVariant {
            index: _name,
            data: _data,
        }
    }
}
impl From<StreamNode> for super::idl_types::StreamNode {
    #[allow(unused_braces)]
    fn from(value: StreamNode) -> Self {
        match value.index {
            0 => super::idl_types::StreamNode::StreamField(Box::new({
                super::idl_types::StreamField::from(unsafe {
                    (value.data as *const StreamField).read()
                })
            })),
            1 => super::idl_types::StreamNode::Comment({
                let _array = unsafe { (value.data as *const AbiArray).read() };
                let _data = _array.data as *const AbiString;
                let _length = _array.length as isize;
                let mut _array_vec = vec![];
                for _index in 0.._length {
                    let _array_item = unsafe { _data.offset(_index).read() };
                    _array_vec.push({ _array_item.to_string() });
                }
                _array_vec
            }),
            _ => panic!(),
        }
    }
}

#[repr(C)]
pub struct StreamField {
    pub ident: *const AbiString,
    pub ty: *const TypeName,
}
impl From<super::idl_types::StreamField> for StreamField {
    #[allow(unused_braces)]
    fn from(value: super::idl_types::StreamField) -> Self {
        Self {
            ident: {
                Box::into_raw(Box::new({ AbiString::from(value.ident) })) as *const AbiString
            },
            ty: { Box::into_raw(Box::new({ TypeName::from(value.ty) })) as *const TypeName },
        }
    }
}
impl From<StreamField> for super::idl_types::StreamField {
    #[allow(unused_braces)]
    fn from(value: StreamField) -> Self {
        Self {
            ident: { unsafe { (value.ident as *const AbiString).read() }.to_string() },
            ty: {
                super::idl_types::TypeName::from(unsafe { (value.ty as *const TypeName).read() })
            },
        }
    }
}

#[repr(C)]
pub struct TypeList {
    pub ident: *const AbiString,
    pub fields: *const AbiArray,
}
impl From<super::idl_types::TypeList> for TypeList {
    #[allow(unused_braces)]
    fn from(value: super::idl_types::TypeList) -> Self {
        Self {
            ident: {
                Box::into_raw(Box::new({ AbiString::from(value.ident) })) as *const AbiString
            },
            fields: {
                Box::into_raw(Box::new({
                    let mut _array_value_items = vec![];
                    for _array_item in value.fields {
                        _array_value_items.push({ TypeListNode::from(_array_item) });
                    }
                    let mut _array = _array_value_items.into_boxed_slice();
                    let _inn_array = AbiArray {
                        length: _array.len() as i64,
                        data: _array.as_mut_ptr() as *const ::core::ffi::c_void,
                    };
                    std::mem::forget(_array);
                    _inn_array
                })) as *const AbiArray
            },
        }
    }
}
impl From<TypeList> for super::idl_types::TypeList {
    #[allow(unused_braces)]
    fn from(value: TypeList) -> Self {
        Self {
            ident: { unsafe { (value.ident as *const AbiString).read() }.to_string() },
            fields: {
                let _array = unsafe { (value.fields as *const AbiArray).read() };
                let _data = _array.data as *const TypeListNode;
                let _length = _array.length as isize;
                let mut _array_vec = vec![];
                for _index in 0.._length {
                    let _array_item = unsafe { _data.offset(_index).read() };
                    _array_vec.push({ super::idl_types::TypeListNode::from(_array_item) });
                }
                _array_vec
            },
        }
    }
}

pub type TypeListNode = AbiVariant;
impl From<super::idl_types::TypeListNode> for TypeListNode {
    #[allow(unused_braces)]
    fn from(value: super::idl_types::TypeListNode) -> Self {
        let (_name, _data) = match value {
            super::idl_types::TypeListNode::TypeListField(_value_field) => (0 as i64, {
                Box::into_raw(Box::new({ TypeListField::from(*_value_field) }))
                    as *const TypeListField
            }
                as *const ::core::ffi::c_void),
            super::idl_types::TypeListNode::Comment(_value_field) => (1 as i64, {
                Box::into_raw(Box::new({
                    let mut _array_value_items = vec![];
                    for _array_item in _value_field {
                        _array_value_items.push({ AbiString::from(_array_item) });
                    }
                    let mut _array = _array_value_items.into_boxed_slice();
                    let _inn_array = AbiArray {
                        length: _array.len() as i64,
                        data: _array.as_mut_ptr() as *const ::core::ffi::c_void,
                    };
                    std::mem::forget(_array);
                    _inn_array
                })) as *const AbiArray
            }
                as *const ::core::ffi::c_void),
        };
        AbiVariant {
            index: _name,
            data: _data,
        }
    }
}
impl From<TypeListNode> for super::idl_types::TypeListNode {
    #[allow(unused_braces)]
    fn from(value: TypeListNode) -> Self {
        match value.index {
            0 => super::idl_types::TypeListNode::TypeListField(Box::new({
                super::idl_types::TypeListField::from(unsafe {
                    (value.data as *const TypeListField).read()
                })
            })),
            1 => super::idl_types::TypeListNode::Comment({
                let _array = unsafe { (value.data as *const AbiArray).read() };
                let _data = _array.data as *const AbiString;
                let _length = _array.length as isize;
                let mut _array_vec = vec![];
                for _index in 0.._length {
                    let _array_item = unsafe { _data.offset(_index).read() };
                    _array_vec.push({ _array_item.to_string() });
                }
                _array_vec
            }),
            _ => panic!(),
        }
    }
}

#[repr(C)]
pub struct TypeListField {
    pub ident: *const AbiString,
    pub ty: *const TypeName,
}
impl From<super::idl_types::TypeListField> for TypeListField {
    #[allow(unused_braces)]
    fn from(value: super::idl_types::TypeListField) -> Self {
        Self {
            ident: {
                Box::into_raw(Box::new({ AbiString::from(value.ident) })) as *const AbiString
            },
            ty: { Box::into_raw(Box::new({ TypeName::from(value.ty) })) as *const TypeName },
        }
    }
}
impl From<TypeListField> for super::idl_types::TypeListField {
    #[allow(unused_braces)]
    fn from(value: TypeListField) -> Self {
        Self {
            ident: { unsafe { (value.ident as *const AbiString).read() }.to_string() },
            ty: {
                super::idl_types::TypeName::from(unsafe { (value.ty as *const TypeName).read() })
            },
        }
    }
}

#[repr(C)]
pub struct TypeEnum {
    pub ident: *const AbiString,
    pub fields: *const AbiArray,
}
impl From<super::idl_types::TypeEnum> for TypeEnum {
    #[allow(unused_braces)]
    fn from(value: super::idl_types::TypeEnum) -> Self {
        Self {
            ident: {
                Box::into_raw(Box::new({ AbiString::from(value.ident) })) as *const AbiString
            },
            fields: {
                Box::into_raw(Box::new({
                    let mut _array_value_items = vec![];
                    for _array_item in value.fields {
                        _array_value_items.push({ EnumNode::from(_array_item) });
                    }
                    let mut _array = _array_value_items.into_boxed_slice();
                    let _inn_array = AbiArray {
                        length: _array.len() as i64,
                        data: _array.as_mut_ptr() as *const ::core::ffi::c_void,
                    };
                    std::mem::forget(_array);
                    _inn_array
                })) as *const AbiArray
            },
        }
    }
}
impl From<TypeEnum> for super::idl_types::TypeEnum {
    #[allow(unused_braces)]
    fn from(value: TypeEnum) -> Self {
        Self {
            ident: { unsafe { (value.ident as *const AbiString).read() }.to_string() },
            fields: {
                let _array = unsafe { (value.fields as *const AbiArray).read() };
                let _data = _array.data as *const EnumNode;
                let _length = _array.length as isize;
                let mut _array_vec = vec![];
                for _index in 0.._length {
                    let _array_item = unsafe { _data.offset(_index).read() };
                    _array_vec.push({ super::idl_types::EnumNode::from(_array_item) });
                }
                _array_vec
            },
        }
    }
}

pub type EnumNode = AbiVariant;
impl From<super::idl_types::EnumNode> for EnumNode {
    #[allow(unused_braces)]
    fn from(value: super::idl_types::EnumNode) -> Self {
        let (_name, _data) = match value {
            super::idl_types::EnumNode::EnumField(_value_field) => (0 as i64, {
                Box::into_raw(Box::new({ EnumField::from(*_value_field) })) as *const EnumField
            }
                as *const ::core::ffi::c_void),
            super::idl_types::EnumNode::Comment(_value_field) => (1 as i64, {
                Box::into_raw(Box::new({
                    let mut _array_value_items = vec![];
                    for _array_item in _value_field {
                        _array_value_items.push({ AbiString::from(_array_item) });
                    }
                    let mut _array = _array_value_items.into_boxed_slice();
                    let _inn_array = AbiArray {
                        length: _array.len() as i64,
                        data: _array.as_mut_ptr() as *const ::core::ffi::c_void,
                    };
                    std::mem::forget(_array);
                    _inn_array
                })) as *const AbiArray
            }
                as *const ::core::ffi::c_void),
        };
        AbiVariant {
            index: _name,
            data: _data,
        }
    }
}
impl From<EnumNode> for super::idl_types::EnumNode {
    #[allow(unused_braces)]
    fn from(value: EnumNode) -> Self {
        match value.index {
            0 => super::idl_types::EnumNode::EnumField(Box::new({
                super::idl_types::EnumField::from(unsafe {
                    (value.data as *const EnumField).read()
                })
            })),
            1 => super::idl_types::EnumNode::Comment({
                let _array = unsafe { (value.data as *const AbiArray).read() };
                let _data = _array.data as *const AbiString;
                let _length = _array.length as isize;
                let mut _array_vec = vec![];
                for _index in 0.._length {
                    let _array_item = unsafe { _data.offset(_index).read() };
                    _array_vec.push({ _array_item.to_string() });
                }
                _array_vec
            }),
            _ => panic!(),
        }
    }
}

#[repr(C)]
pub struct EnumField {
    pub ident: *const AbiString,
}
impl From<super::idl_types::EnumField> for EnumField {
    #[allow(unused_braces)]
    fn from(value: super::idl_types::EnumField) -> Self {
        Self {
            ident: {
                Box::into_raw(Box::new({ AbiString::from(value.ident) })) as *const AbiString
            },
        }
    }
}
impl From<EnumField> for super::idl_types::EnumField {
    #[allow(unused_braces)]
    fn from(value: EnumField) -> Self {
        Self {
            ident: { unsafe { (value.ident as *const AbiString).read() }.to_string() },
        }
    }
}

#[repr(C)]
pub struct TypeConst {
    pub ident: *const AbiString,
    pub fields: *const AbiArray,
    pub const_type: ConstTypes,
}
impl From<super::idl_types::TypeConst> for TypeConst {
    #[allow(unused_braces)]
    fn from(value: super::idl_types::TypeConst) -> Self {
        Self {
            ident: {
                Box::into_raw(Box::new({ AbiString::from(value.ident) })) as *const AbiString
            },
            fields: {
                Box::into_raw(Box::new({
                    let mut _array_value_items = vec![];
                    for _array_item in value.fields {
                        _array_value_items.push({ ConstNode::from(_array_item) });
                    }
                    let mut _array = _array_value_items.into_boxed_slice();
                    let _inn_array = AbiArray {
                        length: _array.len() as i64,
                        data: _array.as_mut_ptr() as *const ::core::ffi::c_void,
                    };
                    std::mem::forget(_array);
                    _inn_array
                })) as *const AbiArray
            },
            const_type: { ConstTypes::from(value.const_type) },
        }
    }
}
impl From<TypeConst> for super::idl_types::TypeConst {
    #[allow(unused_braces)]
    fn from(value: TypeConst) -> Self {
        Self {
            ident: { unsafe { (value.ident as *const AbiString).read() }.to_string() },
            fields: {
                let _array = unsafe { (value.fields as *const AbiArray).read() };
                let _data = _array.data as *const ConstNode;
                let _length = _array.length as isize;
                let mut _array_vec = vec![];
                for _index in 0.._length {
                    let _array_item = unsafe { _data.offset(_index).read() };
                    _array_vec.push({ super::idl_types::ConstNode::from(_array_item) });
                }
                _array_vec
            },
            const_type: { super::idl_types::ConstTypes::from(value.const_type) },
        }
    }
}

pub type ConstTypes = i64;
impl From<super::idl_types::ConstTypes> for ConstTypes {
    fn from(value: super::idl_types::ConstTypes) -> Self {
        match value {
            super::idl_types::ConstTypes::NatInt => 0,
            super::idl_types::ConstTypes::NatFloat => 1,
            super::idl_types::ConstTypes::NatString => 2,
        }
    }
}
impl From<ConstTypes> for super::idl_types::ConstTypes {
    fn from(value: ConstTypes) -> Self {
        match value {
            0 => super::idl_types::ConstTypes::NatInt,
            1 => super::idl_types::ConstTypes::NatFloat,
            2 => super::idl_types::ConstTypes::NatString,
            _ => panic!(),
        }
    }
}

pub type ConstNode = AbiVariant;
impl From<super::idl_types::ConstNode> for ConstNode {
    #[allow(unused_braces)]
    fn from(value: super::idl_types::ConstNode) -> Self {
        let (_name, _data) = match value {
            super::idl_types::ConstNode::ConstField(_value_field) => (0 as i64, {
                Box::into_raw(Box::new({ ConstField::from(*_value_field) })) as *const ConstField
            }
                as *const ::core::ffi::c_void),
            super::idl_types::ConstNode::Comment(_value_field) => (1 as i64, {
                Box::into_raw(Box::new({
                    let mut _array_value_items = vec![];
                    for _array_item in _value_field {
                        _array_value_items.push({ AbiString::from(_array_item) });
                    }
                    let mut _array = _array_value_items.into_boxed_slice();
                    let _inn_array = AbiArray {
                        length: _array.len() as i64,
                        data: _array.as_mut_ptr() as *const ::core::ffi::c_void,
                    };
                    std::mem::forget(_array);
                    _inn_array
                })) as *const AbiArray
            }
                as *const ::core::ffi::c_void),
        };
        AbiVariant {
            index: _name,
            data: _data,
        }
    }
}
impl From<ConstNode> for super::idl_types::ConstNode {
    #[allow(unused_braces)]
    fn from(value: ConstNode) -> Self {
        match value.index {
            0 => super::idl_types::ConstNode::ConstField(Box::new({
                super::idl_types::ConstField::from(unsafe {
                    (value.data as *const ConstField).read()
                })
            })),
            1 => super::idl_types::ConstNode::Comment({
                let _array = unsafe { (value.data as *const AbiArray).read() };
                let _data = _array.data as *const AbiString;
                let _length = _array.length as isize;
                let mut _array_vec = vec![];
                for _index in 0.._length {
                    let _array_item = unsafe { _data.offset(_index).read() };
                    _array_vec.push({ _array_item.to_string() });
                }
                _array_vec
            }),
            _ => panic!(),
        }
    }
}

#[repr(C)]
pub struct ConstField {
    pub ident: *const AbiString,
    pub value: *const AbiString,
}
impl From<super::idl_types::ConstField> for ConstField {
    #[allow(unused_braces)]
    fn from(value: super::idl_types::ConstField) -> Self {
        Self {
            ident: {
                Box::into_raw(Box::new({ AbiString::from(value.ident) })) as *const AbiString
            },
            value: {
                Box::into_raw(Box::new({ AbiString::from(value.value) })) as *const AbiString
            },
        }
    }
}
impl From<ConstField> for super::idl_types::ConstField {
    #[allow(unused_braces)]
    fn from(value: ConstField) -> Self {
        Self {
            ident: { unsafe { (value.ident as *const AbiString).read() }.to_string() },
            value: { unsafe { (value.value as *const AbiString).read() }.to_string() },
        }
    }
}

#[repr(C)]
pub struct Attributes {
    pub field: *const AbiArray,
}
impl From<super::idl_types::Attributes> for Attributes {
    #[allow(unused_braces)]
    fn from(value: super::idl_types::Attributes) -> Self {
        Self {
            field: {
                Box::into_raw(Box::new({
                    let mut _array_value_items = vec![];
                    for _array_item in value.field {
                        _array_value_items.push({ AttributeNode::from(_array_item) });
                    }
                    let mut _array = _array_value_items.into_boxed_slice();
                    let _inn_array = AbiArray {
                        length: _array.len() as i64,
                        data: _array.as_mut_ptr() as *const ::core::ffi::c_void,
                    };
                    std::mem::forget(_array);
                    _inn_array
                })) as *const AbiArray
            },
        }
    }
}
impl From<Attributes> for super::idl_types::Attributes {
    #[allow(unused_braces)]
    fn from(value: Attributes) -> Self {
        Self {
            field: {
                let _array = unsafe { (value.field as *const AbiArray).read() };
                let _data = _array.data as *const AttributeNode;
                let _length = _array.length as isize;
                let mut _array_vec = vec![];
                for _index in 0.._length {
                    let _array_item = unsafe { _data.offset(_index).read() };
                    _array_vec.push({ super::idl_types::AttributeNode::from(_array_item) });
                }
                _array_vec
            },
        }
    }
}

pub type AttributeNode = AbiVariant;
impl From<super::idl_types::AttributeNode> for AttributeNode {
    #[allow(unused_braces)]
    fn from(value: super::idl_types::AttributeNode) -> Self {
        let (_name, _data) = match value {
            super::idl_types::AttributeNode::Name(_value_field) => (0 as i64, {
                Box::into_raw(Box::new({ AttributeNames::from(_value_field) }))
                    as *const AttributeNames
            }
                as *const ::core::ffi::c_void),
            super::idl_types::AttributeNode::UnknownName(_value_field) => (1 as i64, {
                Box::into_raw(Box::new({ AbiString::from(_value_field) })) as *const AbiString
            }
                as *const ::core::ffi::c_void),
            super::idl_types::AttributeNode::StringField(_value_field) => (2 as i64, {
                Box::into_raw(Box::new({ AbiString::from(_value_field) })) as *const AbiString
            }
                as *const ::core::ffi::c_void),
        };
        AbiVariant {
            index: _name,
            data: _data,
        }
    }
}
impl From<AttributeNode> for super::idl_types::AttributeNode {
    #[allow(unused_braces)]
    fn from(value: AttributeNode) -> Self {
        match value.index {
            0 => super::idl_types::AttributeNode::Name({
                super::idl_types::AttributeNames::from(unsafe {
                    (value.data as *const AttributeNames).read()
                })
            }),
            1 => super::idl_types::AttributeNode::UnknownName({
                unsafe { (value.data as *const AbiString).read() }.to_string()
            }),
            2 => super::idl_types::AttributeNode::StringField({
                unsafe { (value.data as *const AbiString).read() }.to_string()
            }),
            _ => panic!(),
        }
    }
}

pub type AttributeNames = i64;
impl From<super::idl_types::AttributeNames> for AttributeNames {
    fn from(value: super::idl_types::AttributeNames) -> Self {
        match value {
            super::idl_types::AttributeNames::DeprecatedName => 0,
        }
    }
}
impl From<AttributeNames> for super::idl_types::AttributeNames {
    fn from(value: AttributeNames) -> Self {
        match value {
            0 => super::idl_types::AttributeNames::DeprecatedName,
            _ => panic!(),
        }
    }
}

pub type TypeName = AbiVariant;
impl From<super::idl_types::TypeName> for TypeName {
    #[allow(unused_braces)]
    fn from(value: super::idl_types::TypeName) -> Self {
        let (_name, _data) = match value {
            super::idl_types::TypeName::Types(_value_field) => (0 as i64, {
                Box::into_raw(Box::new({ Types::from(_value_field) })) as *const Types
            }
                as *const ::core::ffi::c_void),
            super::idl_types::TypeName::TypeFunction(_value_field) => (1 as i64, {
                Box::into_raw(Box::new({ TypeFunction::from(*_value_field) }))
                    as *const TypeFunction
            }
                as *const ::core::ffi::c_void),
            super::idl_types::TypeName::TypeTuple(_value_field) => (2 as i64, {
                Box::into_raw(Box::new({ TypeTuple::from(*_value_field) })) as *const TypeTuple
            }
                as *const ::core::ffi::c_void),
            super::idl_types::TypeName::TypeArray(_value_field) => (3 as i64, {
                Box::into_raw(Box::new({ TypeArray::from(*_value_field) })) as *const TypeArray
            }
                as *const ::core::ffi::c_void),
            super::idl_types::TypeName::TypeMap(_value_field) => (4 as i64, {
                Box::into_raw(Box::new({ TypeMap::from(*_value_field) })) as *const TypeMap
            }
                as *const ::core::ffi::c_void),
            super::idl_types::TypeName::TypeOption(_value_field) => (5 as i64, {
                Box::into_raw(Box::new({ TypeOption::from(*_value_field) })) as *const TypeOption
            }
                as *const ::core::ffi::c_void),
            super::idl_types::TypeName::TypeResult(_value_field) => (6 as i64, {
                Box::into_raw(Box::new({ TypeResult::from(*_value_field) })) as *const TypeResult
            }
                as *const ::core::ffi::c_void),
            super::idl_types::TypeName::ListTypeName(_value_field) => (7 as i64, {
                Box::into_raw(Box::new({ AbiString::from(_value_field) })) as *const AbiString
            }
                as *const ::core::ffi::c_void),
            super::idl_types::TypeName::EnumTypeName(_value_field) => (8 as i64, {
                Box::into_raw(Box::new({ AbiString::from(_value_field) })) as *const AbiString
            }
                as *const ::core::ffi::c_void),
            super::idl_types::TypeName::StructTypeName(_value_field) => (9 as i64, {
                Box::into_raw(Box::new({ AbiString::from(_value_field) })) as *const AbiString
            }
                as *const ::core::ffi::c_void),
            super::idl_types::TypeName::InterfaceTypeName(_value_field) => (10 as i64, {
                Box::into_raw(Box::new({ AbiString::from(_value_field) })) as *const AbiString
            }
                as *const ::core::ffi::c_void),
            super::idl_types::TypeName::ConstTypeName(_value_field) => (11 as i64, {
                Box::into_raw(Box::new({ AbiString::from(_value_field) })) as *const AbiString
            }
                as *const ::core::ffi::c_void),
            super::idl_types::TypeName::StreamTypeName(_value_field) => (12 as i64, {
                Box::into_raw(Box::new({ AbiString::from(_value_field) })) as *const AbiString
            }
                as *const ::core::ffi::c_void),
        };
        AbiVariant {
            index: _name,
            data: _data,
        }
    }
}
impl From<TypeName> for super::idl_types::TypeName {
    #[allow(unused_braces)]
    fn from(value: TypeName) -> Self {
        match value.index {
            0 => super::idl_types::TypeName::Types({
                super::idl_types::Types::from(unsafe { (value.data as *const Types).read() })
            }),
            1 => super::idl_types::TypeName::TypeFunction(Box::new({
                super::idl_types::TypeFunction::from(unsafe {
                    (value.data as *const TypeFunction).read()
                })
            })),
            2 => super::idl_types::TypeName::TypeTuple(Box::new({
                super::idl_types::TypeTuple::from(unsafe {
                    (value.data as *const TypeTuple).read()
                })
            })),
            3 => super::idl_types::TypeName::TypeArray(Box::new({
                super::idl_types::TypeArray::from(unsafe {
                    (value.data as *const TypeArray).read()
                })
            })),
            4 => super::idl_types::TypeName::TypeMap(Box::new({
                super::idl_types::TypeMap::from(unsafe { (value.data as *const TypeMap).read() })
            })),
            5 => super::idl_types::TypeName::TypeOption(Box::new({
                super::idl_types::TypeOption::from(unsafe {
                    (value.data as *const TypeOption).read()
                })
            })),
            6 => super::idl_types::TypeName::TypeResult(Box::new({
                super::idl_types::TypeResult::from(unsafe {
                    (value.data as *const TypeResult).read()
                })
            })),
            7 => super::idl_types::TypeName::ListTypeName({
                unsafe { (value.data as *const AbiString).read() }.to_string()
            }),
            8 => super::idl_types::TypeName::EnumTypeName({
                unsafe { (value.data as *const AbiString).read() }.to_string()
            }),
            9 => super::idl_types::TypeName::StructTypeName({
                unsafe { (value.data as *const AbiString).read() }.to_string()
            }),
            10 => super::idl_types::TypeName::InterfaceTypeName({
                unsafe { (value.data as *const AbiString).read() }.to_string()
            }),
            11 => super::idl_types::TypeName::ConstTypeName({
                unsafe { (value.data as *const AbiString).read() }.to_string()
            }),
            12 => super::idl_types::TypeName::StreamTypeName({
                unsafe { (value.data as *const AbiString).read() }.to_string()
            }),
            _ => panic!(),
        }
    }
}

#[repr(C)]
pub struct TypeFunction {
    pub args: *const TypeName,
    pub return_ty: *const TypeName,
}
impl From<super::idl_types::TypeFunction> for TypeFunction {
    #[allow(unused_braces)]
    fn from(value: super::idl_types::TypeFunction) -> Self {
        Self {
            args: { Box::into_raw(Box::new({ TypeName::from(value.args) })) as *const TypeName },
            return_ty: {
                Box::into_raw(Box::new({ TypeName::from(value.return_ty) })) as *const TypeName
            },
        }
    }
}
impl From<TypeFunction> for super::idl_types::TypeFunction {
    #[allow(unused_braces)]
    fn from(value: TypeFunction) -> Self {
        Self {
            args: {
                super::idl_types::TypeName::from(unsafe { (value.args as *const TypeName).read() })
            },
            return_ty: {
                super::idl_types::TypeName::from(unsafe {
                    (value.return_ty as *const TypeName).read()
                })
            },
        }
    }
}

#[repr(C)]
pub struct TypeArray {
    pub ty: *const TypeName,
}
impl From<super::idl_types::TypeArray> for TypeArray {
    #[allow(unused_braces)]
    fn from(value: super::idl_types::TypeArray) -> Self {
        Self {
            ty: { Box::into_raw(Box::new({ TypeName::from(value.ty) })) as *const TypeName },
        }
    }
}
impl From<TypeArray> for super::idl_types::TypeArray {
    #[allow(unused_braces)]
    fn from(value: TypeArray) -> Self {
        Self {
            ty: {
                super::idl_types::TypeName::from(unsafe { (value.ty as *const TypeName).read() })
            },
        }
    }
}

#[repr(C)]
pub struct TypeMap {
    pub map_ty: *const TypeName,
    pub index_ty: *const TypeName,
}
impl From<super::idl_types::TypeMap> for TypeMap {
    #[allow(unused_braces)]
    fn from(value: super::idl_types::TypeMap) -> Self {
        Self {
            map_ty: {
                Box::into_raw(Box::new({ TypeName::from(value.map_ty) })) as *const TypeName
            },
            index_ty: {
                Box::into_raw(Box::new({ TypeName::from(value.index_ty) })) as *const TypeName
            },
        }
    }
}
impl From<TypeMap> for super::idl_types::TypeMap {
    #[allow(unused_braces)]
    fn from(value: TypeMap) -> Self {
        Self {
            map_ty: {
                super::idl_types::TypeName::from(unsafe {
                    (value.map_ty as *const TypeName).read()
                })
            },
            index_ty: {
                super::idl_types::TypeName::from(unsafe {
                    (value.index_ty as *const TypeName).read()
                })
            },
        }
    }
}

#[repr(C)]
pub struct TypeTuple {
    pub fields: *const AbiArray,
}
impl From<super::idl_types::TypeTuple> for TypeTuple {
    #[allow(unused_braces)]
    fn from(value: super::idl_types::TypeTuple) -> Self {
        Self {
            fields: {
                Box::into_raw(Box::new({
                    let mut _array_value_items = vec![];
                    for _array_item in value.fields {
                        _array_value_items.push({ TupleEntry::from(_array_item) });
                    }
                    let mut _array = _array_value_items.into_boxed_slice();
                    let _inn_array = AbiArray {
                        length: _array.len() as i64,
                        data: _array.as_mut_ptr() as *const ::core::ffi::c_void,
                    };
                    std::mem::forget(_array);
                    _inn_array
                })) as *const AbiArray
            },
        }
    }
}
impl From<TypeTuple> for super::idl_types::TypeTuple {
    #[allow(unused_braces)]
    fn from(value: TypeTuple) -> Self {
        Self {
            fields: {
                let _array = unsafe { (value.fields as *const AbiArray).read() };
                let _data = _array.data as *const TupleEntry;
                let _length = _array.length as isize;
                let mut _array_vec = vec![];
                for _index in 0.._length {
                    let _array_item = unsafe { _data.offset(_index).read() };
                    _array_vec.push({ super::idl_types::TupleEntry::from(_array_item) });
                }
                _array_vec
            },
        }
    }
}

#[repr(C)]
pub struct TupleEntry {
    pub ident: *const AbiString,
    pub ty: *const TypeName,
}
impl From<super::idl_types::TupleEntry> for TupleEntry {
    #[allow(unused_braces)]
    fn from(value: super::idl_types::TupleEntry) -> Self {
        Self {
            ident: {
                Box::into_raw(Box::new({ AbiString::from(value.ident) })) as *const AbiString
            },
            ty: { Box::into_raw(Box::new({ TypeName::from(value.ty) })) as *const TypeName },
        }
    }
}
impl From<TupleEntry> for super::idl_types::TupleEntry {
    #[allow(unused_braces)]
    fn from(value: TupleEntry) -> Self {
        Self {
            ident: { unsafe { (value.ident as *const AbiString).read() }.to_string() },
            ty: {
                super::idl_types::TypeName::from(unsafe { (value.ty as *const TypeName).read() })
            },
        }
    }
}

#[repr(C)]
pub struct TypeResult {
    pub ok_ty: *const TypeName,
    pub err_ty: *const TypeName,
}
impl From<super::idl_types::TypeResult> for TypeResult {
    #[allow(unused_braces)]
    fn from(value: super::idl_types::TypeResult) -> Self {
        Self {
            ok_ty: { Box::into_raw(Box::new({ TypeName::from(value.ok_ty) })) as *const TypeName },
            err_ty: {
                Box::into_raw(Box::new({ TypeName::from(value.err_ty) })) as *const TypeName
            },
        }
    }
}
impl From<TypeResult> for super::idl_types::TypeResult {
    #[allow(unused_braces)]
    fn from(value: TypeResult) -> Self {
        Self {
            ok_ty: {
                super::idl_types::TypeName::from(unsafe { (value.ok_ty as *const TypeName).read() })
            },
            err_ty: {
                super::idl_types::TypeName::from(unsafe {
                    (value.err_ty as *const TypeName).read()
                })
            },
        }
    }
}

#[repr(C)]
pub struct TypeOption {
    pub some_ty: *const TypeName,
}
impl From<super::idl_types::TypeOption> for TypeOption {
    #[allow(unused_braces)]
    fn from(value: super::idl_types::TypeOption) -> Self {
        Self {
            some_ty: {
                Box::into_raw(Box::new({ TypeName::from(value.some_ty) })) as *const TypeName
            },
        }
    }
}
impl From<TypeOption> for super::idl_types::TypeOption {
    #[allow(unused_braces)]
    fn from(value: TypeOption) -> Self {
        Self {
            some_ty: {
                super::idl_types::TypeName::from(unsafe {
                    (value.some_ty as *const TypeName).read()
                })
            },
        }
    }
}
