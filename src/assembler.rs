use std::collections::HashMap;

use crate::{
    ast::{AsmLine, Instruction, Opcode, Operand},
    gen_instruction,
};

pub fn remove_labels(ast: Vec<AsmLine>) -> Vec<Instruction> {
    let mut label_map = HashMap::new();
    let mut pc = 0;
    for line in ast.clone() {
        match line {
            AsmLine::Label(l) => {
                label_map.insert(l, pc);
            }
            AsmLine::Instruction(Instruction { operands, .. }) => {
                pc += 2;
                for operand in operands {
                    match operand {
                        Operand::Immediate(_) => pc += 1,
                        Operand::Label(_) => pc += 1,
                        _ => (),
                    }
                }
            }
        }
    }

    let mut ret = Vec::new();
    for line in ast {
        match line {
            AsmLine::Instruction(instr) => {
                let mut new_instr = gen_instruction!(instr.opcode);
                for (pos, op) in instr.operands.into_iter().enumerate() {
                    match op {
                        Operand::Label(l) => {
                            new_instr.operands[pos] = Operand::Immediate(label_map[&l])
                        }
                        _ => new_instr.operands[pos] = op,
                    }
                }
                ret.push(new_instr);
            }
            _ => (),
        }
    }
    ret
}

pub fn assemble(instrs: Vec<Instruction>) -> Vec<u8> {
    let mut ret = Vec::new();
    for instruction in instrs {
        let mut opcode_word = instruction.opcode as u8;
        let mut register_word = 0_u8;
        let mut append = Vec::new();
        if instruction.opcode != Opcode::Cmp && instruction.opcode != Opcode::Str && instruction.opcode != Opcode::Pst {
            if instruction.operands[0].is_reg() && instruction.operands[0] != Operand::None {
                opcode_word |= instruction.operands[0].unwrap_u8() << 5;
            } else if instruction.operands[0].is_imm() {
                register_word |= 0b10000000;
                append.push(instruction.operands[0].unwrap_u8());
            }

            if instruction.operands[1].is_imm() {
                if 0b10000000 & register_word != 0 {
                    register_word |= 0b01000000;
                } else {
                    register_word |= 0b10000000;
                }
                append.push(instruction.operands[1].unwrap_u8())
            } else if instruction.operands[1] != Operand::None {
                register_word |= instruction.operands[1].unwrap_u8() << 3;
            }
            if instruction.operands[2].is_imm() {
                register_word |= 0b01000000;
                append.push(instruction.operands[2].unwrap_u8());
            } else if instruction.operands[2] != Operand::None {
                register_word |= instruction.operands[2].unwrap_u8();
            }
        } else if instruction.opcode == Opcode::Pst {
            if instruction.operands[0].is_imm() {
                if 0b10000000 & register_word != 0 {
                    register_word |= 0b01000000;
                } else {
                    register_word |= 0b10000000;
                }
                append.push(instruction.operands[0].unwrap_u8());
            } else if instruction.operands[0] != Operand::None {
                register_word |= instruction.operands[0].unwrap_u8() << 3;
            }
            if instruction.operands[1].is_imm() {
                register_word |= 0b01000000;
                append.push(instruction.operands[1].unwrap_u8());
            } else if instruction.operands[1] != Operand::None {
                register_word |= instruction.operands[1].unwrap_u8();
            }
        } else {
            if instruction.operands[0].is_imm() {
                if 0b10000000 & register_word != 0 {
                    register_word |= 0b01000000;
                } else {
                    register_word |= 0b10000000;
                }
                append.push(instruction.operands[0].unwrap_u8());
            } else if instruction.operands[0] != Operand::None {
                register_word |= instruction.operands[0].unwrap_u8() << 3;
            }
            if instruction.operands[1].is_imm() {
                register_word |= 0b01000000;
                append.push(instruction.operands[1].unwrap_u8());
            } else if instruction.operands[1] != Operand::None {
                register_word |= instruction.operands[1].unwrap_u8();
            }
        }
        ret.push(opcode_word);
        ret.push(register_word);
        ret.append(&mut append);
    }
    ret
}
