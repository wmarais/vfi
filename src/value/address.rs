use super::Value;

pub const SIZE_OF_ADDR: usize = size_of::<usize>();

pub trait Address: 
    Value<SIZE_OF_ADDR> +
    Sized + 
    PartialEq +
    PartialOrd {}