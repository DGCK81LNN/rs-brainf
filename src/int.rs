pub enum BfInt {
    I8(i8),
    U8(u8),
    I16(i16),
    U16(u16),
    I32(i32),
    U32(u32),
    I64(i64),
    U64(u64),
    //I128(i128),
    //U128(u128),
}

pub trait BfValue:
    From<BfInt>
    + num::PrimInt
    + num::traits::WrappingAdd
    + num::traits::WrappingSub
    + num::traits::WrappingMul
{
}

macro_rules! bf_int_impl {
    ($variant:ident, $itype:ty) => {
        impl From<$itype> for BfInt {
            fn from(value: $itype) -> Self {
                BfInt::$variant(value)
            }
        }
        impl From<BfInt> for $itype {
            fn from(value: BfInt) -> Self {
                match value {
                    BfInt::I8(n) => n as $itype,
                    BfInt::U8(n) => n as $itype,
                    BfInt::I16(n) => n as $itype,
                    BfInt::U16(n) => n as $itype,
                    BfInt::I32(n) => n as $itype,
                    BfInt::U32(n) => n as $itype,
                    BfInt::I64(n) => n as $itype,
                    BfInt::U64(n) => n as $itype,
                    //BfInt::I128(n) => n as $itype,
                    //BfInt::U128(n) => n as $itype,
                }
            }
        }
        impl BfValue for $itype {}
    };
}
bf_int_impl!(I8, i8);
bf_int_impl!(U8, u8);
bf_int_impl!(I16, i16);
bf_int_impl!(U16, u16);
bf_int_impl!(I32, i32);
bf_int_impl!(U32, u32);
bf_int_impl!(I64, i64);
bf_int_impl!(U64, u64);
//bf_int_impl!(I128, i128);
//bf_int_impl!(U128, u128);

//impl BfInt {
//    fn as_i8(&self) -> i8 { i8::from(*self) }
//    fn as_u8(&self) -> u8 { u8::from(*self) }
//    fn as_i16(&self) -> i16 { i16::from(*self) }
//    fn as_u16(&self) -> u16 { u16::from(*self) }
//    fn as_i32(&self) -> i32 { i32::from(*self) }
//    fn as_u32(&self) -> u32 { u32::from(*self) }
//    fn as_i64(&self) -> i64 { i64::from(*self) }
//    fn as_u64(&self) -> u64 { u64::from(*self) }
//    //fn as_i128(&self) -> i128 { i128::from(*self) }
//    //fn as_u128(&self) -> u128 { u128::from(*self) }
//}

#[macro_export]
macro_rules! int_from {
    ($val: expr) => {
        crate::int::BfInt::from($val).into()
    };
}
