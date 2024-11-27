mod value;
mod opcodes;
#[macro_use]
mod operations;
mod processor;
mod stack;

pub use value::{Value, SignedInteger, UnsignedInteger, Float};
pub use processor::Processor;
pub use stack::Stack;