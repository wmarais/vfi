use super::Value;

use core::ops::{Add, Sub, Mul, Div, Rem, Shl, Shr, BitAnd, BitOr, BitXor};

pub trait SignedInteger<const K: usize>: 
    Value<K> +
    PartialEq +
    PartialOrd +
    BitAnd<Output = Self> +
    BitOr<Output = Self> +
    BitXor<Output = Self> +
    Shl<Output = Self> +
    Shr<Output = Self> +
    Add<Output = Self> + 
    Sub<Output = Self> + 
    Mul<Output = Self> + 
    Div<Output = Self> + 
    Rem<Output = Self> {
        fn from_bool(value: bool) -> Self;
    }

pub const SIZE_OF_I8:   usize = size_of::<i8>();
pub const SIZE_OF_I16:  usize = size_of::<i16>();
pub const SIZE_OF_I32:  usize = size_of::<i32>();
pub const SIZE_OF_I64:  usize = size_of::<i64>();
pub const SIZE_OF_I128: usize = size_of::<i128>();

impl SignedInteger<SIZE_OF_I8> for i8   {
    fn from_bool(value: bool) -> Self {
        if value { 1 } else { 0 }
    }
}
impl SignedInteger<{size_of::<i16>()}> for i16  {
    fn from_bool(value: bool) -> Self {
        if value { 1 } else { 0 }
    }
}
impl SignedInteger<{size_of::<i32>()}> for i32  {
    fn from_bool(value: bool) -> Self {
        if value { 1 } else { 0 }
    }
}
impl SignedInteger<{size_of::<i64>()}> for i64  {
    fn from_bool(value: bool) -> Self {
        if value { 1 } else { 0 }
    }
}
impl SignedInteger<{size_of::<i128>()}> for i128 {
    fn from_bool(value: bool) -> Self {
        if value { 1 } else { 0 }
    }
}

impl Value<SIZE_OF_I8> for i8 {
    #[inline(always)]
    fn from_ne_bytes(buffer: [u8; SIZE_OF_I8]) -> Self {
        i8::from_ne_bytes(buffer)
    }

    #[inline(always)]
    fn to_ne_bytes(&self) -> [u8; SIZE_OF_I8] {
        (*self).to_ne_bytes()
    }
}

impl Value<SIZE_OF_I16> for i16 {
    #[inline(always)]
    fn from_ne_bytes(buffer: [u8; SIZE_OF_I16]) -> Self {
        i16::from_ne_bytes(buffer)
    }

    #[inline(always)]
    fn to_ne_bytes(&self) -> [u8; SIZE_OF_I16] {
        (*self).to_ne_bytes()
    }
}

impl Value<SIZE_OF_I32> for i32 {
    #[inline(always)]
    fn from_ne_bytes(buffer: [u8; SIZE_OF_I32]) -> Self {
        i32::from_ne_bytes(buffer)
    }

    #[inline(always)]
    fn to_ne_bytes(&self) -> [u8; SIZE_OF_I32] {
        (*self).to_ne_bytes()
    }
}

impl Value<SIZE_OF_I64> for i64 {
    #[inline(always)]
    fn from_ne_bytes(buffer: [u8; SIZE_OF_I64]) -> Self {
        i64::from_ne_bytes(buffer)
    }

    #[inline(always)]
    fn to_ne_bytes(&self) -> [u8; SIZE_OF_I64] {
        (*self).to_ne_bytes()
    }
}

impl Value<SIZE_OF_I128> for i128 {
    #[inline(always)]
    fn from_ne_bytes(buffer: [u8; SIZE_OF_I128]) -> Self {
        i128::from_ne_bytes(buffer)
    }

    #[inline(always)]
    fn to_ne_bytes(&self) -> [u8; SIZE_OF_I128] {
        (*self).to_ne_bytes()
    }
}
