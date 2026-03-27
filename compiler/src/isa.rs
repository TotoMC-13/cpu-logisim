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
