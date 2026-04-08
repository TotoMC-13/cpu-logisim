use crate::isa::{Instruction, OpCode, Pseudo, Register};
use core::panic;
use std::collections::HashMap;

pub fn parser(tokens: Vec<String>) -> Vec<u16> {
    let mut tags: HashMap<String, u16> = HashMap::new();
    let mut pc: u16 = 0;

    let mut iter = tokens.iter();
    while let Some(token) = iter.next() {
        if token.ends_with(':') {
            let tag = token.trim_end_matches(':').to_string();
            if tags.contains_key(&tag) {
                panic!("Etiqueta duplicada");
            }
            tags.insert(tag, pc);
            continue;
        }

        if let Some(pseudo) = Pseudo::from_str(token) {
            match pseudo {
                Pseudo::Li { .. } => {
                    iter.next(); // Skipeamos el registro
                    let imm_s = iter.next().unwrap();
                    let imm = if imm_s.starts_with("0x") {
                        i16::from_str_radix(&imm_s[2..], 16).unwrap()
                    } else {
                        imm_s.parse::<i16>().unwrap()
                    };
                    pc += (Pseudo::Li {
                        rd: Register::R0,
                        imm,
                    })
                    .expand()
                    .len() as u16;
                }
                Pseudo::Mv { .. }
                | Pseudo::Neg { .. }
                | Pseudo::Beqz { .. }
                | Pseudo::Bnez { .. } => {
                    iter.next();
                    iter.next(); // Skipeamos 2 argumentos
                    pc += pseudo.expand().len() as u16;
                }
                Pseudo::Jr { .. } => {
                    iter.next(); // Skipeamos 1 argumento
                    pc += pseudo.expand().len() as u16;
                }
                Pseudo::Nop | Pseudo::Ret => pc += pseudo.expand().len() as u16, // NOP, RET
            }
        } else if let Some(opcode) = OpCode::from_str(token) {
            pc += 1;
            match opcode {
                OpCode::J => {
                    iter.next();
                }
                _ => {
                    iter.next();
                    iter.next();
                    iter.next();
                }
            }
        }
    }

    let mut res: Vec<u16> = Vec::new();
    let mut iter = tokens.iter();
    pc = 0;

    while let Some(token) = iter.next() {
        if token.ends_with(':') {
            continue;
        }

        if let Some(pseudo_template) = Pseudo::from_str(token) {
            match pseudo_template {
                Pseudo::Nop => {
                    for inst in pseudo_template.expand() {
                        res.push(inst.encode());
                        pc += 1;
                    }
                }
                Pseudo::Mv { .. } => {
                    let rd_s = iter.next().unwrap();
                    let rs1_s = iter.next().unwrap();

                    let rd = Register::from_str(rd_s).unwrap();
                    let rs1 = Register::from_str(rs1_s).unwrap();

                    for inst in (Pseudo::Mv { rd, rs1 }).expand() {
                        res.push(inst.encode());
                        pc += 1;
                    }
                }
                Pseudo::Neg { .. } => {
                    let rd_s = iter.next().unwrap();
                    let rs1_s = iter.next().unwrap();

                    let rd = Register::from_str(rd_s).unwrap();
                    let rs1 = Register::from_str(rs1_s).unwrap();

                    for inst in (Pseudo::Neg { rd, rs1 }).expand() {
                        res.push(inst.encode());
                        pc += 1;
                    }
                }
                Pseudo::Beqz { .. } => {
                    let rs1_s = iter.next().unwrap();
                    let target_s = iter.next().unwrap();

                    let rs1 = Register::from_str(rs1_s).unwrap();

                    let offset = if let Ok(num) = target_s.parse::<i8>() {
                        num
                    } else if let Ok(hex_num) =
                        i8::from_str_radix(target_s.trim_start_matches("0x"), 16)
                    {
                        hex_num
                    } else {
                        let target_pc = *tags.get(target_s).expect("Error: Etiqueta no definida");
                        (target_pc as i32 - pc as i32) as i8
                    };

                    for inst in (Pseudo::Beqz { rs1, offset }).expand() {
                        res.push(inst.encode());
                        pc += 1;
                    }
                }
                Pseudo::Bnez { .. } => {
                    let rs1_s = iter.next().unwrap();
                    let target_s = iter.next().unwrap();

                    let rs1 = Register::from_str(rs1_s).unwrap();

                    let offset = if let Ok(num) = target_s.parse::<i8>() {
                        num
                    } else if let Ok(hex_num) =
                        i8::from_str_radix(target_s.trim_start_matches("0x"), 16)
                    {
                        hex_num
                    } else {
                        let target_pc = *tags.get(target_s).expect("Error: Etiqueta no definida");
                        (target_pc as i32 - pc as i32 - 1) as i8
                    };

                    for inst in (Pseudo::Bnez { rs1, offset }).expand() {
                        res.push(inst.encode());
                        pc += 1;
                    }
                }
                Pseudo::Li { .. } => {
                    let rd_s = iter.next().unwrap();
                    let imm_s = iter.next().unwrap();

                    let rd = Register::from_str(rd_s).unwrap();

                    let imm = if imm_s.starts_with("0x") {
                        i16::from_str_radix(&imm_s[2..], 16).unwrap()
                    } else {
                        imm_s.parse::<i16>().unwrap()
                    };

                    for inst in (Pseudo::Li { rd, imm }).expand() {
                        res.push(inst.encode());
                        pc += 1;
                    }
                }
                Pseudo::Jr { .. } => {
                    let rs1_s = iter.next().unwrap();

                    let rs1 = Register::from_str(rs1_s).unwrap();

                    for inst in (Pseudo::Jr { rs1 }).expand() {
                        res.push(inst.encode());
                        pc += 1;
                    }
                }
                Pseudo::Ret => {
                    for inst in (Pseudo::Ret).expand() {
                        res.push(inst.encode());
                        pc += 1;
                    }
                }
            }
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
                | OpCode::Sll
                | OpCode::Srl
                | OpCode::Sra
                | OpCode::Slt => {
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
            }
            pc += 1;
        }
    }

    res
}
