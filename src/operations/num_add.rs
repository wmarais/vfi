macro_rules! op_add_i8 {
    ($stack: expr) => {
        {
            let rhs = stack_pop_i8!($stack);
            let lhs = stack_pop_i8!($stack);
            stack_push_i8!($stack,  lhs + rhs);
        }
    };
}

macro_rules! op_add_i16 {
    ($stack: expr) => {
        {
            let rhs = stack_pop_i16!($stack);
            let lhs = stack_pop_i16!($stack);
            stack_push_i16!($stack,  lhs + rhs);
        }
    };
}

macro_rules! op_add_i32 {
    ($stack: expr) => {
        {
            let rhs = stack_pop_i32!($stack);
            let lhs = stack_pop_i32!($stack);
            stack_push_i32!($stack,  lhs + rhs);
        }
    };
}

macro_rules! op_add_i64 {
    ($stack: expr) => {
        {
            let rhs = stack_pop_i64!($stack);
            let lhs = stack_pop_i64!($stack);
            stack_push_i64!($stack,  lhs + rhs);
        }
    };
}

macro_rules! op_add_i128 {
    ($stack: expr) => {
        {
            let rhs = stack_pop_i128!($stack);
            let lhs = stack_pop_i128!($stack);
            stack_push_i128!($stack,  lhs + rhs);
        }
    };
}

macro_rules! op_add_u8 {
    ($stack: expr) => {
        {
            let rhs = stack_pop_u8!($stack);
            let lhs = stack_pop_u8!($stack);
            stack_push_u8!($stack,  lhs + rhs);
        }
    };
}

macro_rules! op_add_u16 {
    ($stack: expr) => {
        {
            let rhs = stack_pop_u16!($stack);
            let lhs = stack_pop_u16!($stack);
            stack_push_u16!($stack,  lhs + rhs);
        }
    };
}

macro_rules! op_add_u32 {
    ($stack: expr) => {
        {
            let rhs = stack_pop_u32!($stack);
            let lhs = stack_pop_u32!($stack);
            stack_push_u32!($stack,  lhs + rhs);
        }
    };
}

macro_rules! op_add_u64 {
    ($stack: expr) => {
        {
            let rhs = stack_pop_u64!($stack);
            let lhs = stack_pop_u64!($stack);
            stack_push_u64!($stack,  lhs + rhs);
        }
    };
}

macro_rules! op_add_u128 {
    ($stack: expr) => {
        {
            let rhs = stack_pop_u128!($stack);
            let lhs = stack_pop_u128!($stack);
            stack_push_u128!($stack,  lhs + rhs);
        }
    };
}

macro_rules! op_add_f32 {
    ($stack: expr) => {
        {
            let rhs = stack_pop_f32!($stack);
            let lhs = stack_pop_f32!($stack);
            stack_push_f32!($stack,  lhs + rhs);
        }
    };
}

macro_rules! op_add_f64 {
    ($stack: expr) => {
        {
            let rhs = stack_pop_f64!($stack);
            let lhs = stack_pop_f64!($stack);
            stack_push_f64!($stack,  lhs + rhs);
        }
    };
}




















































































// macro_rules! stack_push_bytes {
//     ($buffer: expr, $counter: expr, $stack: expr, $size: expr) => {
//         $stack.extend(&$buffer[$counter..$counter+$size]);
//         $counter += $size;
//     }
// }




// // macro_rules! stack_pop {
// //     ($stack: expr, $type) => {
// //         V::from_ne_bytes($stack[beg..end].try_into().unwrap());
// //     };
// // }


// macro_rules! op_add {
//     ($stack: expr, $type:ty) => {
//         let rhs: $type = $stack.pop();
//         let lhs: $type = $stack.pop();
//         let result = lhs + rhs;
//         $stack.push(result);
//     }
// }

// macro_rules! op_sub {
//     ($stack: expr, $type:ty) => {
//         let rhs: $type = $stack.pop();
//         let lhs: $type = $stack.pop();
//         let result = lhs - rhs;
//         $stack.push(result);
//     }
// }

// macro_rules! op_mul {
//     ($stack: expr, $type:ty) => {
//         let rhs: $type = $stack.pop();
//         let lhs: $type = $stack.pop();
//         let result = lhs * rhs;
//         $stack.push(result);
//     }
// }

// macro_rules! op_div {
//     ($stack: expr, $type:ty) => {
//         let rhs: $type = $stack.pop();
//         let lhs: $type = $stack.pop();
//         let result = lhs / rhs;
//         $stack.push(result);
//     }
// }

// macro_rules! op_rem {
//     ($stack: expr, $type:ty) => {
//         let rhs: $type = $stack.pop();
//         let lhs: $type = $stack.pop();
//         let result = lhs % rhs;
//         $stack.push(result);
//     }
// }

// macro_rules! op_eq {
//     ($stack: expr, $type:ty) => {
//         let rhs: $type = $stack.pop();
//         let lhs: $type = $stack.pop();
//         let result = i32::from_bool(lhs == rhs);
//         $stack.push(result);
//     }
// }

// macro_rules! op_neq {
//     ($stack: expr, $type:ty) => {
//         let rhs: $type = $stack.pop();
//         let lhs: $type = $stack.pop();
//         let result = i32::from_bool(lhs != rhs);
//         $stack.push(result);
//     }
// }

// macro_rules! op_les {
//     ($stack: expr, $type:ty) => {
//         let rhs: $type = $stack.pop();
//         let lhs: $type = $stack.pop();
//         let result = i32::from_bool(lhs < rhs);
//         $stack.push(result);
//     }
// }

// macro_rules! op_grt {
//     ($stack: expr, $type:ty) => {
//         let rhs: $type = $stack.pop();
//         let lhs: $type = $stack.pop();
//         let result = i32::from_bool(lhs > rhs);
//         $stack.push(result);
//     }
// }

// macro_rules! op_leq {
//     ($stack: expr, $type:ty) => {
//         let rhs: $type = $stack.pop();
//         let lhs: $type = $stack.pop();
//         let result = i32::from_bool(lhs <= rhs);
//         $stack.push(result);
//     }
// }

// macro_rules! op_geq {
//     ($stack: expr, $type:ty) => {
//         let rhs: $type = $stack.pop();
//         let lhs: $type = $stack.pop();
//         let result = i32::from_bool(lhs >= rhs);
//         $stack.push(result);
//     }
// }

// macro_rules! op_sqrt {
//     ($stack: expr, $type:ty) => {
//         let lhs: $type = $stack.pop();
//         let result = lhs.sqrt();
//         $stack.push(result);
//     }
// }

// macro_rules! op_shl {
//     ($stack: expr, $type:ty) => {
//         let lhs: $type = $stack.pop();
//         let rhs: $type = $stack.pop();
//         let result = lhs << rhs;
//         $stack.push(result);
//     }
// }

// macro_rules! op_shr {
//     ($stack: expr, $type:ty) => {
//         let lhs: $type = $stack.pop();
//         let rhs: $type = $stack.pop();
//         let result = lhs << rhs;
//         $stack.push(result);
//     }
// }

// macro_rules! op_bit_and {
//     ($stack: expr, $type:ty) => {
//         let lhs: $type = $stack.pop();
//         let rhs: $type = $stack.pop();
//         let result = lhs & rhs;
//         $stack.push(result);
//     }
// }

// macro_rules! op_bit_or {
//     ($stack: expr, $type:ty) => {
//         let lhs: $type = $stack.pop();
//         let rhs: $type = $stack.pop();
//         let result = lhs | rhs;
//         $stack.push(result);
//     }
// }

// macro_rules! op_bit_xor {
//     ($stack: expr, $type:ty) => {
//         let lhs: $type = $stack.pop();
//         let rhs: $type = $stack.pop();
//         let result = lhs ^ rhs;
//         $stack.push(result);
//     }
// }
