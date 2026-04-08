pub enum Instruction {
    RType {
        opcode: OpCode,
        rs1: Register,
        rd: Register,
        rs2: Register,
    },
    IType {
        opcode: OpCode,
        rd: Register,
        rs1: Register,
        imm: i8,
    },
    SType {
        opcode: OpCode,
        imm: i8,
        rs1: Register,
        rs2: Register,
    },
    BType {
        opcode: OpCode,
        imm: i8,
        rs1: Register,
        rs2: Register,
    },
    JType {
        opcode: OpCode,
        imm: i16,
    },
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum Register {
    R0 = 0,
    Ra = 1,
    Sp = 2,
    Gp = 3,
    A0 = 4,
    A1 = 5,
    A2 = 6,
    A3 = 7,
    T0 = 8,
    T1 = 9,
    T2 = 10,
    T3 = 11,
    S0 = 12,
    S1 = 13,
    S2 = 14,
    S3 = 15,
}

impl Register {
    pub fn from_str(s: &str) -> Option<Register> {
        match s.to_uppercase().as_str() {
            "R0" | "ZERO" => Some(Register::R0),
            "R1" | "RA" => Some(Register::Ra),
            "R2" | "SP" => Some(Register::Sp),
            "R3" | "GP" => Some(Register::Gp),
            "R4" | "A0" => Some(Register::A0),
            "R5" | "A1" => Some(Register::A1),
            "R6" | "A2" => Some(Register::A2),
            "R7" | "A3" => Some(Register::A3),
            "R8" | "T0" => Some(Register::T0),
            "R9" | "T1" => Some(Register::T1),
            "R10" | "T2" => Some(Register::T2),
            "R11" | "T3" => Some(Register::T3),
            "R12" | "S0" => Some(Register::S0),
            "R13" | "S1" => Some(Register::S1),
            "R14" | "S2" => Some(Register::S2),
            "R15" | "S3" => Some(Register::S3),
            _ => None,
        }
    }
}

#[derive(Copy, Clone)]
#[repr(u16)]
pub enum OpCode {
    // R-type
    Add = 0b0000,
    Sub = 0b0001,
    And = 0b0010,
    Or = 0b0011,
    Xor = 0b0100,
    Sll = 0b0101,
    Srl = 0b0110,
    Sra = 0b0111,
    Slt = 0b1000,

    // I-type
    Addi = 0b1001,
    Andi = 0b1010,
    Lw = 0b1011,
    Jalr = 0b1111,

    // S-type
    Sw = 0b1100,

    // B-type
    Beq = 0b1101,

    // J-type
    J = 0b1110,
}

impl OpCode {
    pub fn from_str(s: &str) -> Option<OpCode> {
        match s.to_uppercase().as_str() {
            "ADD" => Some(OpCode::Add),
            "SUB" => Some(OpCode::Sub),
            "AND" => Some(OpCode::And),
            "OR" => Some(OpCode::Or),
            "XOR" => Some(OpCode::Xor),
            "SLL" => Some(OpCode::Sll),
            "SRL" => Some(OpCode::Srl),
            "SRA" => Some(OpCode::Sra),
            "SLT" => Some(OpCode::Slt),
            "ADDI" => Some(OpCode::Addi),
            "ANDI" => Some(OpCode::Andi),
            "LW" => Some(OpCode::Lw),
            "SW" => Some(OpCode::Sw),
            "BEQ" => Some(OpCode::Beq),
            "J" => Some(OpCode::J),
            "JALR" => Some(OpCode::Jalr),
            _ => None,
        }
    }
}

#[derive(Copy, Clone)]
pub enum Pseudo {
    Nop,
    Mv { rd: Register, rs1: Register },
    Neg { rd: Register, rs1: Register },
    Beqz { rs1: Register, offset: i8 },
    Bnez { rs1: Register, offset: i8 },
    Li { rd: Register, imm: i16 }, // Para constantes de 4b o 16b
    Jr { rs1: Register },
    Ret,
}

impl Pseudo {
    pub fn from_str(s: &str) -> Option<Pseudo> {
        match s.to_uppercase().as_str() {
            "NOP" => Some(Pseudo::Nop),
            "MV" => Some(Pseudo::Mv {
                rd: Register::R0,
                rs1: Register::R0,
            }),
            "NEG" => Some(Pseudo::Neg {
                rd: Register::R0,
                rs1: Register::R0,
            }),
            "BEQZ" => Some(Pseudo::Beqz {
                rs1: Register::R0,
                offset: 0,
            }),
            "BNEZ" => Some(Pseudo::Bnez {
                rs1: Register::R0,
                offset: 0,
            }),
            "LI" => Some(Pseudo::Li {
                rd: Register::R0,
                imm: 0,
            }),
            "JR" => Some(Pseudo::Jr { rs1: Register::R0 }),
            "RET" => Some(Pseudo::Ret),
            _ => None,
        }
    }

    pub fn expand(&self) -> Vec<Instruction> {
        match self {
            Pseudo::Nop => vec![Instruction::RType {
                opcode: OpCode::Add,
                rd: Register::R0,
                rs1: Register::R0,
                rs2: Register::R0,
            }],
            Pseudo::Mv { rd, rs1 } => vec![Instruction::RType {
                opcode: OpCode::Add,
                rd: *rd,
                rs1: *rs1,
                rs2: Register::R0,
            }],
            Pseudo::Neg { rd, rs1 } => vec![Instruction::RType {
                opcode: OpCode::Sub,
                rd: *rd,
                rs1: *rs1,
                rs2: Register::R0,
            }],
            Pseudo::Beqz { rs1, offset } => vec![Instruction::BType {
                opcode: OpCode::Beq,
                imm: *offset,
                rs1: *rs1,
                rs2: Register::R0,
            }],
            Pseudo::Bnez { rs1, offset } => vec![
                Instruction::BType {
                    opcode: OpCode::Beq,
                    imm: 1,
                    rs1: *rs1,
                    rs2: Register::R0,
                },
                Instruction::JType {
                    opcode: OpCode::J,
                    imm: *offset as i16,
                },
            ],
            Pseudo::Li { rd, imm } => {
                let mut res: Vec<Instruction> = Vec::new();

                if *imm >= -8 && *imm <= 7 {
                    res.push(Instruction::IType {
                        opcode: OpCode::Addi,
                        rd: *rd,
                        rs1: Register::R0,
                        imm: *imm as i8,
                    });
                } else {
                    let n1 = ((*imm >> 12) & 0xF) as i8;
                    let n2 = ((*imm >> 8) & 0xF) as i8;
                    let n3 = ((*imm >> 4) & 0xF) as i8;
                    let n4 = (*imm & 0xF) as i8;

                    res.push(Instruction::IType {
                        opcode: OpCode::Addi,
                        rd: Register::T3,
                        rs1: Register::R0,
                        imm: 4,
                    });

                    res.push(Instruction::IType {
                        opcode: OpCode::Addi,
                        rd: *rd,
                        rs1: Register::R0,
                        imm: n1,
                    });
                    for nibble in [n2, n3, n4] {
                        res.push(Instruction::RType {
                            opcode: OpCode::Sll,
                            rd: *rd,
                            rs1: *rd,
                            rs2: Register::T3,
                        });

                        res.push(Instruction::IType {
                            opcode: OpCode::Addi,
                            rd: Register::T0,
                            rs1: Register::R0,
                            imm: nibble,
                        });

                        res.push(Instruction::IType {
                            opcode: OpCode::Andi,
                            rd: Register::T0,
                            rs1: Register::T0,
                            imm: 0xF,
                        });

                        res.push(Instruction::RType {
                            opcode: OpCode::Or,
                            rd: *rd,
                            rs1: *rd,
                            rs2: Register::T0,
                        });
                    }
                }

                res
            }
            Pseudo::Jr { rs1 } => vec![Instruction::IType {
                opcode: OpCode::Jalr,
                rd: Register::R0,
                rs1: *rs1,
                imm: 0,
            }],
            Pseudo::Ret => vec![Instruction::IType {
                opcode: OpCode::Jalr,
                rd: Register::R0,
                rs1: Register::Ra,
                imm: 0,
            }],
        }
    }
}
