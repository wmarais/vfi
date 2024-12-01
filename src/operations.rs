// macro_rules! stack_push_bytes {
//     ($stack: expr, $bytes: expr) => {
//         $stack.extend($bytes);
//     }
// }

// macro_rules! stack_pop {
//     ($stack: expr, $type) => {
//         V::from_ne_bytes($stack[beg..end].try_into().unwrap());
//     };
// }


macro_rules! op_add {
    ($stack: expr, $type:ty) => {
        let rhs: $type = $stack.pop();
        let lhs: $type = $stack.pop();
        let result = lhs + rhs;
        $stack.push(result);
    }
}

macro_rules! op_sub {
    ($stack: expr, $type:ty) => {
        let rhs: $type = $stack.pop();
        let lhs: $type = $stack.pop();
        let result = lhs - rhs;
        $stack.push(result);
    }
}

macro_rules! op_mul {
    ($stack: expr, $type:ty) => {
        let rhs: $type = $stack.pop();
        let lhs: $type = $stack.pop();
        let result = lhs * rhs;
        $stack.push(result);
    }
}

macro_rules! op_div {
    ($stack: expr, $type:ty) => {
        let rhs: $type = $stack.pop();
        let lhs: $type = $stack.pop();
        let result = lhs / rhs;
        $stack.push(result);
    }
}

macro_rules! op_rem {
    ($stack: expr, $type:ty) => {
        let rhs: $type = $stack.pop();
        let lhs: $type = $stack.pop();
        let result = lhs % rhs;
        $stack.push(result);
    }
}

macro_rules! op_eq {
    ($stack: expr, $type:ty) => {
        let rhs: $type = $stack.pop();
        let lhs: $type = $stack.pop();
        let result = i32::from_bool(lhs == rhs);
        $stack.push(result);
    }
}

macro_rules! op_neq {
    ($stack: expr, $type:ty) => {
        let rhs: $type = $stack.pop();
        let lhs: $type = $stack.pop();
        let result = i32::from_bool(lhs != rhs);
        $stack.push(result);
    }
}

macro_rules! op_les {
    ($stack: expr, $type:ty) => {
        let rhs: $type = $stack.pop();
        let lhs: $type = $stack.pop();
        let result = i32::from_bool(lhs < rhs);
        $stack.push(result);
    }
}

macro_rules! op_grt {
    ($stack: expr, $type:ty) => {
        let rhs: $type = $stack.pop();
        let lhs: $type = $stack.pop();
        let result = i32::from_bool(lhs > rhs);
        $stack.push(result);
    }
}

macro_rules! op_leq {
    ($stack: expr, $type:ty) => {
        let rhs: $type = $stack.pop();
        let lhs: $type = $stack.pop();
        let result = i32::from_bool(lhs <= rhs);
        $stack.push(result);
    }
}

macro_rules! op_geq {
    ($stack: expr, $type:ty) => {
        let rhs: $type = $stack.pop();
        let lhs: $type = $stack.pop();
        let result = i32::from_bool(lhs >= rhs);
        $stack.push(result);
    }
}

macro_rules! op_sqrt {
    ($stack: expr, $type:ty) => {
        let lhs: $type = $stack.pop();
        let result = lhs.sqrt();
        $stack.push(result);
    }
}

macro_rules! op_shl {
    ($stack: expr, $type:ty) => {
        let lhs: $type = $stack.pop();
        let rhs: $type = $stack.pop();
        let result = lhs << rhs;
        $stack.push(result);
    }
}

macro_rules! op_shr {
    ($stack: expr, $type:ty) => {
        let lhs: $type = $stack.pop();
        let rhs: $type = $stack.pop();
        let result = lhs << rhs;
        $stack.push(result);
    }
}

macro_rules! op_bit_and {
    ($stack: expr, $type:ty) => {
        let lhs: $type = $stack.pop();
        let rhs: $type = $stack.pop();
        let result = lhs & rhs;
        $stack.push(result);
    }
}

macro_rules! op_bit_or {
    ($stack: expr, $type:ty) => {
        let lhs: $type = $stack.pop();
        let rhs: $type = $stack.pop();
        let result = lhs | rhs;
        $stack.push(result);
    }
}

macro_rules! op_bit_xor {
    ($stack: expr, $type:ty) => {
        let lhs: $type = $stack.pop();
        let rhs: $type = $stack.pop();
        let result = lhs ^ rhs;
        $stack.push(result);
    }
}
