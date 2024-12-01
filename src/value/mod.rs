mod address;
mod float;
mod signed_integer;
mod string;
mod unsigned_integer;

use crate::Stack;

pub use address::{
    Address,
    SIZE_OF_ADDR
};

pub use float::{
    Float,
    SIZE_OF_F32,
    SIZE_OF_F64
};

pub use signed_integer::{
    SignedInteger,
    SIZE_OF_I8,
    SIZE_OF_I16,
    SIZE_OF_I32,
    SIZE_OF_I64,
    SIZE_OF_I128
};

pub use unsigned_integer::{
    UnsignedInteger,
    SIZE_OF_U8,
    SIZE_OF_U16,
    SIZE_OF_U32,
    SIZE_OF_U64,
    SIZE_OF_U128
};

pub trait Value <const K: usize>: Sized {
    fn from_ne_bytes(buffer: [u8; K]) -> Self; 
    fn to_ne_bytes(&self) -> [u8; K];
}

pub trait Binding {
    fn invoke(&self, stack: &mut Stack);
}


pub struct FunctionBinding<F> {
    handle: F
}

impl<F> Binding for FunctionBinding<F> {
    fn invoke(&self, stack: &mut Stack) {
        self.execute(stack);
    }
}

impl<F> FunctionBinding<F> {
    fn execute(&self, stack: &mut Stack) {
        
    }
}
