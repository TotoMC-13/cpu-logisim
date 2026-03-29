use crate::isa::{Instruction, OpCode, Register};
use core::panic;
use std::collections::HashMap;

pub fn parser(tokens: Vec<String>) -> Vec<u16> {
    let mut tags: HashMap<String, u16> = HashMap::new();
    let mut pc: u16 = 0;

    for token in &tokens {
        if token.ends_with(':') {
            let tag = token.trim_end_matches(':').to_string();

            if tags.contains_key(&tag) {
                panic!("Error: La etiqueta {} esta duplicada", tag)
            }

            tags.insert(tag, pc);
        } else if OpCode::from_str(&token).is_some() {
            pc += 1;
        }
    }

    let mut res: Vec<u16> = Vec::new();
    let mut iter = tokens.iter();
    pc = 0;

    while let Some(token) = iter.next() {
        if token.ends_with(':') {
            continue;
        }

        if let Some(opcode) = OpCode::from_str(token) {
            match opcode {
                // ---- TIPO R ----
                OpCode::Add
                | OpCode::Sub
                | OpCode::And
                | OpCode::Or
                | OpCode::Xor
                | OpCode::Sll => {
                    let rd = Register::from_str(iter.next().unwrap()).expect("Error parseando Rd");
                    let rs1 =
                        Register::from_str(iter.next().unwrap()).expect("Error parseando Rs1");
                    let rs2 =
                        Register::from_str(iter.next().unwrap()).expect("Error parseando Rs2");

                    let instruccion = Instruction::RType {
                        opcode,
                        rs1,
                        rd,
                        rs2,
                    };
                    res.push(instruccion.encode());
                }

                // ---- TIPO I ----
                OpCode::Addi | OpCode::Andi | OpCode::Lw | OpCode::Jalr => {
                    let rd = Register::from_str(iter.next().unwrap()).expect("Error parseando Rd");
                    let rs1 =
                        Register::from_str(iter.next().unwrap()).expect("Error parseando Rs1");
                    let num_str = iter.next().unwrap();
                    let imm = num_str
                        .parse::<i8>()
                        .expect("El inmediato no es un numero valido");

                    let instruccion = Instruction::IType {
                        opcode,
                        rs1,
                        rd,
                        imm,
                    };
                    res.push(instruccion.encode());
                }

                // ---- TIPO S ----
                OpCode::Sw => {
                    let num_str = iter.next().unwrap();
                    let imm = num_str
                        .parse::<i8>()
                        .expect("El inmediato no es un numero valido");
                    let rs1 =
                        Register::from_str(iter.next().unwrap()).expect("Error parseando Rs1");
                    let rs2 =
                        Register::from_str(iter.next().unwrap()).expect("Error parseando Rs2");

                    let instruccion = Instruction::SType {
                        opcode,
                        imm,
                        rs1,
                        rs2,
                    };
                    res.push(instruccion.encode());
                }

                // ---- TIPO B ----
                OpCode::Beq => {
                    let rs1 =
                        Register::from_str(iter.next().unwrap()).expect("Error parseando Rs1");
                    let rs2 =
                        Register::from_str(iter.next().unwrap()).expect("Error parseando Rs2");

                    let destino_str = iter.next().unwrap();
                    let imm = if let Ok(num) = destino_str.parse::<i8>() {
                        num
                    } else {
                        let dir_destino = tags.get(destino_str).unwrap_or_else(|| {
                            panic!("Error: La etiqueta '{}' no existe", destino_str)
                        });

                        // Calculamos el Offset Relativo
                        let diferencia = (*dir_destino as i16) - (pc as i16);
                        diferencia as i8
                    };
                    let instruccion = Instruction::BType {
                        opcode,
                        imm,
                        rs1,
                        rs2,
                    };
                    res.push(instruccion.encode());
                }

                // ---- TIPO J ----
                OpCode::J => {
                    let destino_str = iter.next().unwrap();
                    let imm = if let Ok(num) = destino_str.parse::<i16>() {
                        num
                    } else {
                        let dir_destino = tags.get(destino_str).unwrap_or_else(|| {
                            panic!("Error: La etiqueta '{}' no existe", destino_str)
                        });

                        // Calculamos el Offset Relativo
                        (*dir_destino as i16) - (pc as i16)
                    };
                    let instruccion = Instruction::JType { opcode, imm };
                    res.push(instruccion.encode());
                }

                _ => continue,
            }
            pc += 1;
        }
    }

    res
}
