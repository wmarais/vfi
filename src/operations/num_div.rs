macro_rules! op_div_i8 {
    ($stack: expr) => {
        {
            let rhs = stack_pop_i8!($stack);
            let lhs = stack_pop_i8!($stack);
            stack_push_i8!($stack,  lhs / rhs);
        }
    };
}

macro_rules! op_div_i16 {
    ($stack: expr) => {
        {
            let rhs = stack_pop_i16!($stack);
            let lhs = stack_pop_i16!($stack);
            stack_push_i16!($stack,  lhs / rhs);
        }
    };
}

macro_rules! op_div_i32 {
    ($stack: expr) => {
        {
            let rhs = stack_pop_i32!($stack);
            let lhs = stack_pop_i32!($stack);
            stack_push_i32!($stack,  lhs / rhs);
        }
    };
}

macro_rules! op_div_i64 {
    ($stack: expr) => {
        {
            let rhs = stack_pop_i64!($stack);
            let lhs = stack_pop_i64!($stack);
            stack_push_i64!($stack,  lhs / rhs);
        }
    };
}

macro_rules! op_div_i128 {
    ($stack: expr) => {
        {
            let rhs = stack_pop_i128!($stack);
            let lhs = stack_pop_i128!($stack);
            stack_push_i128!($stack,  lhs / rhs);
        }
    };
}

macro_rules! op_div_u8 {
    ($stack: expr) => {
        {
            let rhs = stack_pop_u8!($stack);
            let lhs = stack_pop_u8!($stack);
            stack_push_u8!($stack,  lhs / rhs);
        }
    };
}

macro_rules! op_div_u16 {
    ($stack: expr) => {
        {
            let rhs = stack_pop_u16!($stack);
            let lhs = stack_pop_u16!($stack);
            stack_push_u16!($stack,  lhs / rhs);
        }
    };
}

macro_rules! op_div_u32 {
    ($stack: expr) => {
        {
            let rhs = stack_pop_u32!($stack);
            let lhs = stack_pop_u32!($stack);
            stack_push_u32!($stack,  lhs / rhs);
        }
    };
}

macro_rules! op_div_u64 {
    ($stack: expr) => {
        {
            let rhs = stack_pop_u64!($stack);
            let lhs = stack_pop_u64!($stack);
            stack_push_u64!($stack,  lhs / rhs);
        }
    };
}

macro_rules! op_div_u128 {
    ($stack: expr) => {
        {
            let rhs = stack_pop_u128!($stack);
            let lhs = stack_pop_u128!($stack);
            stack_push_u128!($stack,  lhs / rhs);
        }
    };
}

macro_rules! op_div_f32 {
    ($stack: expr) => {
        {
            let rhs = stack_pop_f32!($stack);
            let lhs = stack_pop_f32!($stack);
            stack_push_f32!($stack,  lhs / rhs);
        }
    };
}

macro_rules! op_div_f64 {
    ($stack: expr) => {
        {
            let rhs = stack_pop_f64!($stack);
            let lhs = stack_pop_f64!($stack);
            stack_push_f64!($stack,  lhs / rhs);
        }
    };
}
