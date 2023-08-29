#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum Opcode {
    Hlt = 0b11111111,
    Mov = 0,
    Add,
    Sub,
    Mul,
    And,
    Or,
    Xor,
    Not,
    Cmp,
    Jgr,
    Jlt,
    Jge,
    Jle,
    Jeq,
    Jnq,
    Jmp,
    Cal,
    Ret,
    Psh,
    Pop,
    Lod,
    Str,
    Rsh,
    Lsh,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operand {
    Register(u8),
    Immediate(u8),
    Label(String),
    None,
}

impl Operand {
    pub fn unwrap_u8(&self) -> u8 {
        match self {
            Operand::Immediate(i) => *i,
            Operand::Register(i) => *i,
            _ => unreachable!("unreachable executed in unwrap_u8"),
        }
    }
    pub fn is_imm(&self) -> bool {
        match self {
            Operand::Immediate(_) => true,
            _ => false,
        }
    }
    /// HACK: This function is only ever called in the assembler, and there are only ever imms or regs there
    pub fn is_reg(&self) -> bool {
        !self.is_imm()
    }
}

#[derive(Debug, Clone)]
pub struct Instruction {
    pub opcode: Opcode,
    pub operands: [Operand; 3],
}

#[derive(Debug, Clone)]
pub enum AsmLine {
    Instruction(Instruction),
    Label(String),
}
