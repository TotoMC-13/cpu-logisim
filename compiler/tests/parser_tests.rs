use compiler::isa::{Instruction, OpCode, Register};
use compiler::parser::parser;

fn tokenizar(frase: &str) -> Vec<String> {
    frase.split_whitespace().map(|s| s.to_string()).collect()
}

#[test]
fn test_rtype_add() {
    let tokens = tokenizar("ADD R1 R2 R3");
    let binario_salida = parser(tokens);

    let binario_esperado = Instruction::RType {
        opcode: OpCode::Add,
        rd: Register::Ra,  // R1 mapped to Ra
        rs1: Register::Sp, // R2 mapped to Sp
        rs2: Register::Gp, // R3 mapped to Gp
    }
    .encode();

    assert_eq!(binario_salida, vec![binario_esperado]);
}

#[test]
fn test_itype_addi() {
    let tokens = tokenizar("ADDI R4 R5 12");
    let binario_salida = parser(tokens);

    let binario_esperado = Instruction::IType {
        opcode: OpCode::Addi,
        rd: Register::A0,  // R4
        rs1: Register::A1, // R5
        imm: 12,
    }
    .encode();

    assert_eq!(binario_salida, vec![binario_esperado]);
}

#[test]
fn test_stype_sw() {
    let tokens = tokenizar("SW 15 R8 R9");
    let binario_salida = parser(tokens);

    let binario_esperado = Instruction::SType {
        opcode: OpCode::Sw,
        imm: 15,
        rs1: Register::T0, // R8
        rs2: Register::T1, // R9
    }
    .encode();

    assert_eq!(binario_salida, vec![binario_esperado]);
}

#[test]
fn test_btype_beq_con_etiqueta() {
    // MAIN: ADD R1 R2 R3
    // BEQ R1 R2 MAIN (El BEQ debería saltar -1 instrucción al offset)
    let tokens = tokenizar("MAIN: ADD R1 R2 R3 BEQ R1 R2 MAIN");
    let binario_salida = parser(tokens);

    let instruccion_1 = Instruction::RType {
        opcode: OpCode::Add,
        rd: Register::Ra,
        rs1: Register::Sp,
        rs2: Register::Gp,
    }
    .encode();

    let instruccion_b = Instruction::BType {
        opcode: OpCode::Beq,
        rs1: Register::Ra,
        rs2: Register::Sp,
        imm: -1,
    }
    .encode();

    assert_eq!(binario_salida, vec![instruccion_1, instruccion_b]);
}

#[test]
fn test_jtype_absoluto() {
    let tokens = tokenizar("J 45");
    let binario_salida = parser(tokens);

    let binario_esperado = Instruction::JType {
        opcode: OpCode::J,
        imm: 45,
    }
    .encode();

    assert_eq!(binario_salida, vec![binario_esperado]);
}
