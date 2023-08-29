use lalrpop_util::lalrpop_mod;

pub mod assembler;
pub mod ast;
lalrpop_mod!(pub grammar);

#[macro_export]
macro_rules! gen_instruction {
    ($type:expr) => {
        Instruction {
            opcode: $type,
            operands: [Operand::None, Operand::None, Operand::None],
        }
    };
    ($type:expr; $op1:expr) => {
        Instruction {
            opcode: $type,
            operands: [$op1, Operand::None, Operand::None],
        }
    };
    ($type:expr; $op1:expr, $op2:expr) => {
        Instruction {
            opcode: $type,
            operands: [$op1, $op2, Operand::None],
        }
    };
    ($type:expr; $op1:expr, $op2:expr, $op3:expr) => {
        Instruction {
            opcode: $type,
            operands: [$op1, $op2, $op3],
        }
    };
}
