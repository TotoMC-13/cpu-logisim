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
