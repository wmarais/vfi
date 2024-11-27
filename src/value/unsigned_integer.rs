use super::Value;

use core::ops::{Add, Sub, Mul, Div, Rem, Shl, Shr, BitAnd, BitOr, BitXor};

pub trait UnsignedInteger<const K: usize>: 
    Value<K> +
    Sized + 
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
    Rem<Output = Self>  {}

pub const SIZE_OF_U8:   usize = size_of::<u8>();
pub const SIZE_OF_U16:  usize = size_of::<u16>();
pub const SIZE_OF_U32:  usize = size_of::<u32>();
pub const SIZE_OF_U64:  usize = size_of::<u64>();
pub const SIZE_OF_U128: usize = size_of::<u128>();

impl UnsignedInteger<SIZE_OF_U8> for u8   {}
impl UnsignedInteger<SIZE_OF_U16> for u16  {}
impl UnsignedInteger<SIZE_OF_U32> for u32  {}
impl UnsignedInteger<SIZE_OF_U64> for u64  {}
impl UnsignedInteger<SIZE_OF_U128> for u128 {}

impl Value<SIZE_OF_U8> for u8 {
    #[inline(always)]
    fn from_ne_bytes(buffer: [u8; SIZE_OF_U8]) -> Self {
        Self::from_ne_bytes(buffer)
    }

    #[inline(always)]
    fn to_ne_bytes(&self) -> [u8; SIZE_OF_U8] {
        (*self).to_ne_bytes()
    }
}

impl Value<SIZE_OF_U16> for u16 {
    #[inline(always)]
    fn from_ne_bytes(buffer: [u8; SIZE_OF_U16]) -> Self {
        Self::from_ne_bytes(buffer)
    }

    #[inline(always)]
    fn to_ne_bytes(&self) -> [u8; SIZE_OF_U16] {
        (*self).to_ne_bytes()
    }
}

impl Value<SIZE_OF_U32> for u32 {
    #[inline(always)]
    fn from_ne_bytes(buffer: [u8; SIZE_OF_U32]) -> Self {
        Self::from_ne_bytes(buffer)
    }

    #[inline(always)]
    fn to_ne_bytes(&self) -> [u8; SIZE_OF_U32] {
        (*self).to_ne_bytes()
    }
}

impl Value<SIZE_OF_U64> for u64 {
    #[inline(always)]
    fn from_ne_bytes(buffer: [u8; SIZE_OF_U64]) -> Self {
        Self::from_ne_bytes(buffer)
    }

    #[inline(always)]
    fn to_ne_bytes(&self) -> [u8; SIZE_OF_U64] {
        (*self).to_ne_bytes()
    }
}

impl Value<SIZE_OF_U128> for u128 {
    #[inline(always)]
    fn from_ne_bytes(buffer: [u8; SIZE_OF_U128]) -> Self {
        Self::from_ne_bytes(buffer)
    }

    #[inline(always)]
    fn to_ne_bytes(&self) -> [u8; SIZE_OF_U128] {
        (*self).to_ne_bytes()
    }
}
