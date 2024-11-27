use super::Value;
use core::ops::{Add, Sub, Mul, Div, Rem};

pub trait Float<const K: usize>: 
    Value<K> +
    Sized + 
    PartialEq +
    PartialOrd +
    Add<Output = Self> + 
    Sub<Output = Self> + 
    Mul<Output = Self> + 
    Div<Output = Self> + 
    Rem<Output = Self>  {
        fn sqrt(&self) -> Self;
    }


pub const SIZE_OF_F32: usize = size_of::<f32>();
pub const SIZE_OF_F64: usize = size_of::<f64>();


impl Float<SIZE_OF_F32> for f32 {
    fn sqrt(&self) -> Self {
        (*self).sqrt()
    }
}

impl Float<SIZE_OF_F64> for f64 {
    fn sqrt(&self) -> Self {
        (*self).sqrt()
    }
}

impl Value<SIZE_OF_F32> for f32 {
    #[inline(always)]
    fn from_ne_bytes(buffer: [u8; SIZE_OF_F32]) -> Self {
        f32::from_ne_bytes(buffer)
    }

    #[inline(always)]
    fn to_ne_bytes(&self) -> [u8; SIZE_OF_F32] {
        (*self).to_ne_bytes()
    }
}

impl Value<SIZE_OF_F64> for f64 {
    #[inline(always)]
    fn from_ne_bytes(buffer: [u8; SIZE_OF_F64]) -> Self {
        Self::from_ne_bytes(buffer)
    }

    #[inline(always)]
    fn to_ne_bytes(&self) -> [u8; SIZE_OF_F64] {
        (*self).to_ne_bytes()
    }
}