//=================================================================================================
// Push a value from a variable into the stack.
//=================================================================================================
macro_rules! stack_push_i8 {
    ($stack: expr, $value: expr) => {
        $stack.extend($value.to_ne_bytes());
    }
}

macro_rules! stack_push_i16 {
    ($stack: expr, $value: expr) => {
        $stack.extend($value.to_ne_bytes());
    }
}

macro_rules! stack_push_i32 {
    ($stack: expr, $value: expr) => {
        $stack.extend($value.to_ne_bytes());
    }
}

macro_rules! stack_push_i64 {
    ($stack: expr, $value: expr) => {
        $stack.extend($value.to_ne_bytes());
    }
}

macro_rules! stack_push_i128 {
    ($stack: expr, $value: expr) => {
        $stack.extend($value.to_ne_bytes());
    }
}

macro_rules! stack_push_u8 {
    ($stack: expr, $value: expr) => {
        $stack.extend($value.to_ne_bytes());
    }
}

macro_rules! stack_push_u16 {
    ($stack: expr, $value: expr) => {
        $stack.extend($value.to_ne_bytes());
    }
}

macro_rules! stack_push_u32 {
    ($stack: expr, $value: expr) => {
        $stack.extend($value.to_ne_bytes());
    }
}

macro_rules! stack_push_u64 {
    ($stack: expr, $value: expr) => {
        $stack.extend($value.to_ne_bytes());
    }
}

macro_rules! stack_push_u128 {
    ($stack: expr, $value: expr) => {
        $stack.extend($value.to_ne_bytes());
    }
}

macro_rules! stack_push_f32 {
    ($stack: expr, $value: expr) => {
        $stack.extend($value.to_ne_bytes());
    }
}

macro_rules! stack_push_f64 {
    ($stack: expr, $value: expr) => {
        $stack.extend($value.to_ne_bytes());
    }
}

// macro_rules! stack_push_addr {
//     ($stack: expr, $value: expr) => {
//         $stack.extend($value.to_ne_bytes());
//     }
// }