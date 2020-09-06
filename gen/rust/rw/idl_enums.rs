#[derive(Debug)]
pub enum IdlAbiError {
    Ok,
    InvalidArg,
    NullPtr,
    Abort,
    CallbackException,
    Unimplemented,
    NotAllowedOperatoin,
}

#[derive(Debug)]
pub enum IdlInterfaceAttributes {
    PGet,
    PSet,
    MStream,
    MFactory,
    MAsync,
    MSync,
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum IdlTypes {
    NInt,
    NFloat,
    NBool,
    NEnum,
    NString,
    NBytes,
    NStruct,
    NFunction,
    NInterface,
    NTypeList,
    NTuple,
    NVoid,
    NNone,
    NArray,
}

impl std::convert::TryFrom<i64> for IdlTypes {
    type Error = ();

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(IdlTypes::NInt),
            1 => Ok(IdlTypes::NFloat),
            2 => Ok(IdlTypes::NBool),
            3 => Ok(IdlTypes::NEnum),
            4 => Ok(IdlTypes::NString),
            5 => Ok(IdlTypes::NBytes),
            6 => Ok(IdlTypes::NStruct),
            7 => Ok(IdlTypes::NFunction),
            8 => Ok(IdlTypes::NInterface),
            9 => Ok(IdlTypes::NTypeList),
            10 => Ok(IdlTypes::NTuple),
            11 => Ok(IdlTypes::NVoid),
            12 => Ok(IdlTypes::NNone),
            13 => Ok(IdlTypes::NArray),
            _ => Err(())
        }
    }
}

