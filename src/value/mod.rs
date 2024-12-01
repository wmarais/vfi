mod address;
mod float;
mod signed_integer;
mod string;
mod unsigned_integer;

use crate::Stack;

pub use address::{
    Address,
};

pub use float::{
    Float,
};

pub use signed_integer::{
    SignedInteger,
};

pub use unsigned_integer::{
    UnsignedInteger
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
