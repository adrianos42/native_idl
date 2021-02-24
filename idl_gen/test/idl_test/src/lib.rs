mod ffi;
mod ffi_types;
#[derive(Debug, Clone)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}
#[derive(Debug, Clone)]
pub enum Types {
    IntType(i64),
    StringType(String),
}
pub struct Names(String);
impl Names {
    pub const NAME: Names = Names("Adriano");
    pub const SURNAME: Names = Names("Souza");
}
pub struct ValuesFloat(f64);
impl ValuesFloat {
    pub const PI: ValuesFloat = ValuesFloat(3.14);
    pub const SQRT: ValuesFloat = ValuesFloat(2.55);
    pub const ONEDW: ValuesFloat = ValuesFloat(122332.0);
}
pub struct ValuesInteger(i64);
impl ValuesInteger {
    pub const ONE: ValuesInteger = ValuesInteger(1);
    pub const TWO: ValuesInteger = ValuesInteger(2);
    pub const THREE: ValuesInteger = ValuesInteger(3);
}
pub struct ValuesUuid(Uuid);
impl ValuesUuid {
    pub const FIRST: ValuesUuid =
        ValuesUuid(Uuid::from_u128(296289971956447042072627609082762517043));
    pub const SECOND: ValuesUuid =
        ValuesUuid(Uuid::from_u128(260977251990978308441640609507304342989));
    pub const THIRD: ValuesUuid =
        ValuesUuid(Uuid::from_u128(292033564790391402541401596494284923513));
    pub const FOURTH: ValuesUuid =
        ValuesUuid(Uuid::from_u128(60699404614046739753320738030883304109));
}
