//=================================================================================================
// Load primitive types from the the application byte stream into the stack. These macros assume
// the byte stream is in native byte order and does not require reordering. This is done
// specifically for performance. The interpreter need to check if the byte order match before
// executing the application, or perform some JIT to reorder.
//=================================================================================================
macro_rules! stack_load_i8 {
    ($buffer: expr, $counter: expr, $stack: expr) => {
        $stack.extend(&$buffer[$counter..$counter+SIZE_OF_I8]);
        $counter += SIZE_OF_I8;
    }
}

macro_rules! stack_load_u8 {
    ($buffer: expr, $counter: expr, $stack: expr) => {
        $stack.extend(&$buffer[$counter..$counter+SIZE_OF_U8]);
        $counter += SIZE_OF_U8;
    }
}

macro_rules! stack_load_i16 {
    ($buffer: expr, $counter: expr, $stack: expr) => {
        $stack.extend(&$buffer[$counter..$counter+SIZE_OF_I16]);
        $counter += SIZE_OF_I16;
    }
}

macro_rules! stack_load_u16 {
    ($buffer: expr, $counter: expr, $stack: expr) => {
        $stack.extend(&$buffer[$counter..$counter+SIZE_OF_U16]);
        $counter += SIZE_OF_U16;
    }
}

macro_rules! stack_load_i32 {
    ($buffer: expr, $counter: expr, $stack: expr) => {
        $stack.extend(&$buffer[$counter..$counter+SIZE_OF_I32]);
        $counter += SIZE_OF_I32;
    }
}

macro_rules! stack_load_u32 {
    ($buffer: expr, $counter: expr, $stack: expr) => {
        $stack.extend(&$buffer[$counter..$counter+SIZE_OF_U32]);
        $counter += SIZE_OF_U32;
    }
}

macro_rules! stack_load_i64 {
    ($buffer: expr, $counter: expr, $stack: expr) => {
        $stack.extend(&$buffer[$counter..$counter+SIZE_OF_I64]);
        $counter += SIZE_OF_I64;
    }
}

macro_rules! stack_load_u64 {
    ($buffer: expr, $counter: expr, $stack: expr) => {
        $stack.extend(&$buffer[$counter..$counter+SIZE_OF_U64]);
        $counter += SIZE_OF_U64;
    }
}

macro_rules! stack_load_i128 {
    ($buffer: expr, $counter: expr, $stack: expr) => {
        $stack.extend(&$buffer[$counter..$counter+SIZE_OF_I128]);
        $counter += SIZE_OF_I128;
    }
}

macro_rules! stack_load_u128 {
    ($buffer: expr, $counter: expr, $stack: expr) => {
        $stack.extend(&$buffer[$counter..$counter+SIZE_OF_U128]);
        $counter += SIZE_OF_U128;
    }
}

macro_rules! stack_load_f32 {
    ($buffer: expr, $counter: expr, $stack: expr) => {
        $stack.extend(&$buffer[$counter..$counter+SIZE_OF_F32]);
        $counter += SIZE_OF_F32;
    }
}

macro_rules! stack_load_f64 {
    ($buffer: expr, $counter: expr, $stack: expr) => {
        $stack.extend(&$buffer[$counter..$counter+SIZE_OF_F64]);
        $counter += SIZE_OF_F64;
    }
}

macro_rules! stack_load_addr {
    ($buffer: expr, $counter: expr, $stack: expr) => {
        $stack.extend(&$buffer[$counter..$counter+SIZE_OF_ADDR]);
        $counter += SIZE_OF_ADDR;
    }
}