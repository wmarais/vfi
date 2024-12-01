//=================================================================================================
// Pop a value from the stack and return it as a variable.
//=================================================================================================
macro_rules! stack_pop_i8 {
    ($stack: expr) => {
        i8::from_ne_bytes($stack.drain($stack.len()-SIZE_OF_I8..)
            .as_slice().try_into().unwrap())
    };
}

macro_rules! stack_pop_i16 {
    ($stack: expr) => {
        i16::from_ne_bytes($stack.drain($stack.len()-SIZE_OF_I16..)
            .as_slice().try_into().unwrap())
    };
}

macro_rules! stack_pop_i32 {
    ($stack: expr) => {
        i32::from_ne_bytes($stack.drain($stack.len()-SIZE_OF_I32..)
            .as_slice().try_into().unwrap())
    };
}

macro_rules! stack_pop_i64 {
    ($stack: expr) => {
        i8::from_ne_bytes($stack.drain($stack.len()-SIZE_OF_I64..)
            .as_slice().try_into().unwrap())
    };
}

macro_rules! stack_pop_i128 {
    ($stack: expr) => {
        i8::from_ne_bytes($stack.drain($stack.len()-SIZE_OF_I128..)
            .as_slice().try_into().unwrap())
    };
}

macro_rules! stack_pop_u8 {
    ($stack: expr) => {
        i8::from_ne_bytes($stack.drain($stack.len()-SIZE_OF_U8..)
            .as_slice().try_into().unwrap())
    };
}

macro_rules! stack_pop_u16 {
    ($stack: expr) => {
        i8::from_ne_bytes($stack.drain($stack.len()-SIZE_OF_U16..)
            .as_slice().try_into().unwrap())
    };
}

macro_rules! stack_pop_u32 {
    ($stack: expr) => {
        i8::from_ne_bytes($stack.drain($stack.len()-SIZE_OF_U32..)
            .as_slice().try_into().unwrap())
    };
}

macro_rules! stack_pop_u64 {
    ($stack: expr) => {
        i8::from_ne_bytes($stack.drain($stack.len()-SIZE_OF_U64..)
            .as_slice().try_into().unwrap())
    };
}

macro_rules! stack_pop_u128 {
    ($stack: expr) => {
        i8::from_ne_bytes($stack.drain($stack.len()-SIZE_OF_U128..)
            .as_slice().try_into().unwrap())
    };
}

macro_rules! stack_pop_f32 {
    ($stack: expr) => {
        i8::from_ne_bytes($stack.drain($stack.len()-SIZE_OF_F32..)
            .as_slice().try_into().unwrap())
    };
}

macro_rules! stack_pop_f64 {
    ($stack: expr) => {
        i8::from_ne_bytes($stack.drain($stack.len()-SIZE_OF_F32..)
            .as_slice().try_into().unwrap())
    };
}