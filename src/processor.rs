use super::opcodes::*;
use super::value::*;
use super::Stack;

pub enum TrapCode {
    Unreachable,
    ReservedOpCode
}

pub struct Processor {
    stack: Stack
}

impl Processor {
    pub fn new() -> Self {
        Processor {
            stack: Stack::new(2*1024*1024*1024)
        }
    }

    pub fn execute(&mut self, program: &[u8]) -> Result<(), TrapCode>{
        let mut counter: usize = 0;

        let stack = &mut self.stack;

        while counter < program.len() {
            let op_code = program[counter];
            counter += 1;
            
            match op_code {
                OP_STACK_PUSH_I8 => {
                    stack.push_bytes(&program[counter..counter+SIZE_OF_I8]);
                    counter += SIZE_OF_I8;
                },

                OP_STACK_PUSH_I16 => {
                    stack.push_bytes(&program[counter..counter+SIZE_OF_I16]);
                    counter += SIZE_OF_I16;
                },

                OP_STACK_PUSH_I32 => {
                    stack.push_bytes(&program[counter..counter+SIZE_OF_I32]);
                    counter += SIZE_OF_I32;
                },

                OP_STACK_PUSH_I64 => {
                    stack.push_bytes(&program[counter..counter+SIZE_OF_I64]);
                    counter += SIZE_OF_I64;
                },

                OP_STACK_PUSH_I128 => {
                    stack.push_bytes(&program[counter..counter+SIZE_OF_I128]);
                    counter += SIZE_OF_I128;
                },

                OP_STACK_PUSH_F32 => {
                    stack.push_bytes(&program[counter..counter+SIZE_OF_F32]);
                    counter += SIZE_OF_F32;
                },

                OP_STACK_PUSH_F64 => {
                    stack.push_bytes(&program[counter..counter+SIZE_OF_F64]);
                    counter += SIZE_OF_F64;
                },

                OP_STACK_PUSH_ADDR => { 
                    stack.push_bytes(&program[counter .. counter + SIZE_OF_ADDR]);
                    counter += SIZE_OF_ADDR;
                },

                // Addition
                OP_NUM_ADD_I8           => { op_add!(stack, i8); },
                OP_NUM_ADD_I16          => { op_add!(stack, i16); },
                OP_NUM_ADD_I32          => { op_add!(stack, i32); },
                OP_NUM_ADD_I64          => { op_add!(stack, i64); },
                OP_NUM_ADD_I128         => { op_add!(stack, i128); },
                OP_NUM_ADD_U8           => { op_add!(stack, u8); },
                OP_NUM_ADD_U16          => { op_add!(stack, u16); },
                OP_NUM_ADD_U32          => { op_add!(stack, u32); },
                OP_NUM_ADD_U64          => { op_add!(stack, u64); },
                OP_NUM_ADD_U128         => { op_add!(stack, u128); },
                OP_NUM_ADD_F32          => { op_add!(stack, f32); },
                OP_NUM_ADD_F64          => { op_add!(stack, f64); },

                // Subtraction
                OP_NUM_SUB_I8           => { op_sub!(stack, i8); },
                OP_NUM_SUB_I16          => { op_sub!(stack, i16); },
                OP_NUM_SUB_I32          => { op_sub!(stack, i32); },
                OP_NUM_SUB_I64          => { op_sub!(stack, i64); },
                OP_NUM_SUB_I128         => { op_sub!(stack, i128); },
                OP_NUM_SUB_U8           => { op_sub!(stack, u8); },
                OP_NUM_SUB_U16          => { op_sub!(stack, u16); },
                OP_NUM_SUB_U32          => { op_sub!(stack, u32); },
                OP_NUM_SUB_U64          => { op_sub!(stack, u64); },
                OP_NUM_SUB_U128         => { op_sub!(stack, u128); },
                OP_NUM_SUB_F32          => { op_sub!(stack, f32); },
                OP_NUM_SUB_F64          => { op_sub!(stack, f64); },

                // Multiplication
                OP_NUM_MUL_I8           => { op_mul!(stack, i8); },
                OP_NUM_MUL_I16          => { op_mul!(stack, i16); },
                OP_NUM_MUL_I32          => { op_mul!(stack, i32); },
                OP_NUM_MUL_I64          => { op_mul!(stack, i64); },
                OP_NUM_MUL_I128         => { op_mul!(stack, i128); },
                OP_NUM_MUL_U8           => { op_mul!(stack, u8); },
                OP_NUM_MUL_U16          => { op_mul!(stack, u16); },
                OP_NUM_MUL_U32          => { op_mul!(stack, u32); },
                OP_NUM_MUL_U64          => { op_mul!(stack, u64);  },
                OP_NUM_MUL_U128         => { op_mul!(stack, u128); },
                OP_NUM_MUL_F32          => { op_mul!(stack, f32); },
                OP_NUM_MUL_F64          => { op_mul!(stack, f64); },

                // Division
                OP_NUM_DIV_I8           => { op_div!(stack, i8); },
                OP_NUM_DIV_I16          => { op_div!(stack, i16); },
                OP_NUM_DIV_I32          => { op_div!(stack, i32); },
                OP_NUM_DIV_I64          => { op_div!(stack, i64); },
                OP_NUM_DIV_I128         => { op_div!(stack, i128); },
                OP_NUM_DIV_U8           => { op_div!(stack, u8); },
                OP_NUM_DIV_U16          => { op_div!(stack, u16); },
                OP_NUM_DIV_U32          => { op_div!(stack, u32); },
                OP_NUM_DIV_U64          => { op_div!(stack, u64);  },
                OP_NUM_DIV_U128         => { op_div!(stack, u128); },
                OP_NUM_DIV_F32          => { op_div!(stack, f32); },
                OP_NUM_DIV_F64          => { op_div!(stack, f64); },

                // Remainder
                OP_NUM_REM_I8           => { op_rem!(stack, i8); },
                OP_NUM_REM_I16          => { op_rem!(stack, i16); },
                OP_NUM_REM_I32          => { op_rem!(stack, i32); },
                OP_NUM_REM_I64          => { op_rem!(stack, i64); },
                OP_NUM_REM_I128         => { op_rem!(stack, i128); },
                OP_NUM_REM_U8           => { op_rem!(stack, u8); },
                OP_NUM_REM_U16          => { op_rem!(stack, u16); },
                OP_NUM_REM_U32          => { op_rem!(stack, u32); },
                OP_NUM_REM_U64          => { op_rem!(stack, u64);  },
                OP_NUM_REM_U128         => { op_rem!(stack, u128); },

                // Equal
                OP_NUM_EQ_I8            => { op_eq!(stack, i8); },
                OP_NUM_EQ_I16           => { op_eq!(stack, i16); },
                OP_NUM_EQ_I32           => { op_eq!(stack, i32); },
                OP_NUM_EQ_I64           => { op_eq!(stack, i64); },
                OP_NUM_EQ_I128          => { op_eq!(stack, i128); },
                OP_NUM_EQ_U8            => { op_eq!(stack, u8); },
                OP_NUM_EQ_U16           => { op_eq!(stack, u16); },
                OP_NUM_EQ_U32           => { op_eq!(stack, u32); },
                OP_NUM_EQ_U64           => { op_eq!(stack, u64);  },
                OP_NUM_EQ_U128          => { op_eq!(stack, u128); },
                OP_NUM_EQ_F32           => { op_eq!(stack, f32); },
                OP_NUM_EQ_F64           => { op_eq!(stack, f64); },

                // Not Equal
                OP_NUM_NEQ_I8           => { op_neq!(stack, i8); },
                OP_NUM_NEQ_I16          => { op_neq!(stack, i16); },
                OP_NUM_NEQ_I32          => { op_neq!(stack, i32); },
                OP_NUM_NEQ_I64          => { op_neq!(stack, i64); },
                OP_NUM_NEQ_I128         => { op_neq!(stack, i128); },
                OP_NUM_NEQ_U8           => { op_neq!(stack, u8); },
                OP_NUM_NEQ_U16          => { op_neq!(stack, u16); },
                OP_NUM_NEQ_U32          => { op_neq!(stack, u32); },
                OP_NUM_NEQ_U64          => { op_neq!(stack, u64);  },
                OP_NUM_NEQ_U128         => { op_neq!(stack, u128); },
                OP_NUM_NEQ_F32          => { op_neq!(stack, f32); },
                OP_NUM_NEQ_F64          => { op_neq!(stack, f64); },

                // Less Than
                OP_NUM_LES_I8           => { op_les!(stack, i8); },
                OP_NUM_LES_I16          => { op_les!(stack, i16); },
                OP_NUM_LES_I32          => { op_les!(stack, i32); },
                OP_NUM_LES_I64          => { op_les!(stack, i64); },
                OP_NUM_LES_I128         => { op_les!(stack, i128); },
                OP_NUM_LES_U8           => { op_les!(stack, u8); },
                OP_NUM_LES_U16          => { op_les!(stack, u16); },
                OP_NUM_LES_U32          => { op_les!(stack, u32); },
                OP_NUM_LES_U64          => { op_les!(stack, u64);  },
                OP_NUM_LES_U128         => { op_les!(stack, u128); },
                OP_NUM_LES_F32          => { op_les!(stack, f32); },
                OP_NUM_LES_F64          => { op_les!(stack, f64); },

                // Greater Than
                OP_NUM_GRT_I8           => { op_grt!(stack, i8); },
                OP_NUM_GRT_I16          => { op_grt!(stack, i16); },
                OP_NUM_GRT_I32          => { op_grt!(stack, i32); },
                OP_NUM_GRT_I64          => { op_grt!(stack, i64); },
                OP_NUM_GRT_I128         => { op_grt!(stack, i128); },
                OP_NUM_GRT_U8           => { op_grt!(stack, u8); },
                OP_NUM_GRT_U16          => { op_grt!(stack, u16); },
                OP_NUM_GRT_U32          => { op_grt!(stack, u32); },
                OP_NUM_GRT_U64          => { op_grt!(stack, u64);  },
                OP_NUM_GRT_U128         => { op_grt!(stack, u128); },
                OP_NUM_GRT_F32          => { op_grt!(stack, f32); },
                OP_NUM_GRT_F64          => { op_grt!(stack, f64); },
               
                // Less Than or Equal
                OP_NUM_LEQ_I8           => { op_leq!(stack, i8); },
                OP_NUM_LEQ_I16          => { op_leq!(stack, i16); },
                OP_NUM_LEQ_I32          => { op_leq!(stack, i32); },
                OP_NUM_LEQ_I64          => { op_leq!(stack, i64); },
                OP_NUM_LEQ_I128         => { op_leq!(stack, i128); },
                OP_NUM_LEQ_U8           => { op_leq!(stack, u8); },
                OP_NUM_LEQ_U16          => { op_leq!(stack, u16); },
                OP_NUM_LEQ_U32          => { op_leq!(stack, u32); },
                OP_NUM_LEQ_U64          => { op_leq!(stack, u64);  },
                OP_NUM_LEQ_U128         => { op_leq!(stack, u128); },
                OP_NUM_LEQ_F32          => { op_leq!(stack, f32); },
                OP_NUM_LEQ_F64          => { op_leq!(stack, f64); },

                // Greater Than or Equal
                OP_NUM_GEQ_I8           => { op_geq!(stack, i8); },
                OP_NUM_GEQ_I16          => { op_geq!(stack, i16); },
                OP_NUM_GEQ_I32          => { op_geq!(stack, i32); },
                OP_NUM_GEQ_I64          => { op_geq!(stack, i64); },
                OP_NUM_GEQ_I128         => { op_geq!(stack, i128); },
                OP_NUM_GEQ_U8           => { op_geq!(stack, u8); },
                OP_NUM_GEQ_U16          => { op_geq!(stack, u16); },
                OP_NUM_GEQ_U32          => { op_geq!(stack, u32); },
                OP_NUM_GEQ_U64          => { op_geq!(stack, u64);  },
                OP_NUM_GEQ_U128         => { op_geq!(stack, u128); },
                OP_NUM_GEQ_F32          => { op_geq!(stack, f32); },
                OP_NUM_GEQ_F64          => { op_geq!(stack, f64); },
                
                // Squareroot
                OP_NUM_SQRT_F32         => { op_sqrt!(stack, f32); },
                OP_NUM_SQRT_F64         => { op_sqrt!(stack, f64); },

                // Left Shift
                OP_BIT_SHL_I8           => { op_shl!(stack, i8); },
                OP_BIT_SHL_I16          => { op_shl!(stack, i16); },
                OP_BIT_SHL_I32          => { op_shl!(stack, i32); },
                OP_BIT_SHL_I64          => { op_shl!(stack, i64); },
                OP_BIT_SHL_I128         => { op_shl!(stack, i128); },
                OP_BIT_SHL_U8           => { op_shl!(stack, u8); },
                OP_BIT_SHL_U16          => { op_shl!(stack, u16); },
                OP_BIT_SHL_U32          => { op_shl!(stack, u32); },
                OP_BIT_SHL_U64          => { op_shl!(stack, u64);  },
                OP_BIT_SHL_U128         => { op_shl!(stack, u128); },

                // Right Shift
                OP_BIT_SHR_I8           => { op_shr!(stack, i8); },
                OP_BIT_SHR_I16          => { op_shr!(stack, i16); },
                OP_BIT_SHR_I32          => { op_shr!(stack, i32); },
                OP_BIT_SHR_I64          => { op_shr!(stack, i64); },
                OP_BIT_SHR_I128         => { op_shr!(stack, i128); },
                OP_BIT_SHR_U8           => { op_shr!(stack, u8); },
                OP_BIT_SHR_U16          => { op_shr!(stack, u16); },
                OP_BIT_SHR_U32          => { op_shr!(stack, u32); },
                OP_BIT_SHR_U64          => { op_shr!(stack, u64);  },
                OP_BIT_SHR_U128         => { op_shr!(stack, u128); },

                // And
                OP_BIT_AND_I8           => { op_bit_and!(stack, i8); },
                OP_BIT_AND_I16          => { op_bit_and!(stack, i16); },
                OP_BIT_AND_I32          => { op_bit_and!(stack, i32); },
                OP_BIT_AND_I64          => { op_bit_and!(stack, i64); },
                OP_BIT_AND_I128         => { op_bit_and!(stack, i128); },
                OP_BIT_AND_U8           => { op_bit_and!(stack, u8); },
                OP_BIT_AND_U16          => { op_bit_and!(stack, u16); },
                OP_BIT_AND_U32          => { op_bit_and!(stack, u32); },
                OP_BIT_AND_U64          => { op_bit_and!(stack, u64);  },
                OP_BIT_AND_U128         => { op_bit_and!(stack, u128); },

                // Or
                OP_BIT_OR_I8            => { op_bit_or!(stack, i8); },
                OP_BIT_OR_I16           => { op_bit_or!(stack, i16); },
                OP_BIT_OR_I32           => { op_bit_or!(stack, i32); },
                OP_BIT_OR_I64           => { op_bit_or!(stack, i64); },
                OP_BIT_OR_I128          => { op_bit_or!(stack, i128); },
                OP_BIT_OR_U8            => { op_bit_or!(stack, u8); },
                OP_BIT_OR_U16           => { op_bit_or!(stack, u16); },
                OP_BIT_OR_U32           => { op_bit_or!(stack, u32); },
                OP_BIT_OR_U64           => { op_bit_or!(stack, u64);  },
                OP_BIT_OR_U128          => { op_bit_or!(stack, u128); },

                // Xor
                OP_BIT_XOR_I8           => { op_bit_xor!(stack, i8); },
                OP_BIT_XOR_I16          => { op_bit_xor!(stack, i16); },
                OP_BIT_XOR_I32          => { op_bit_xor!(stack, i32); },
                OP_BIT_XOR_I64          => { op_bit_xor!(stack, i64); },
                OP_BIT_XOR_I128         => { op_bit_xor!(stack, i128); },
                OP_BIT_XOR_U8           => { op_bit_xor!(stack, u8); },
                OP_BIT_XOR_U16          => { op_bit_xor!(stack, u16); },
                OP_BIT_XOR_U32          => { op_bit_xor!(stack, u32); },
                OP_BIT_XOR_U64          => { op_bit_xor!(stack, u64);  },
                OP_BIT_XOR_U128         => { op_bit_xor!(stack, u128); },

                _ => { return Err(TrapCode::ReservedOpCode); }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut proc = Processor::new();
        let program: Vec<u8> = Vec::new();

        proc.execute(program.as_slice());
        proc.execute(program.as_slice());
    }
}



