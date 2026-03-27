use crate::isa::Instruction;

impl Instruction {
    pub fn encode(&self) -> u16 {
        match self {
            Instruction::RType {
                opcode,
                rs1,
                rd,
                rs2,
            } => {
                // Formato R: opcode[15:12], rs1[11:8], rd[7:4], rs2[3:0]
                (*opcode as u16) << 12 | (*rs1 as u16) << 8 | (*rd as u16) << 4 | (*rs2 as u16)
            }
            Instruction::IType {
                opcode,
                rd,
                rs1,
                imm,
            } => {
                // Formato I: opcode[15:12], rd[11:8], rs1[7:4], imm[3:0]
                // El & 0xF es para asegurarnos de usar solo 4 bits del inmediato (Mascara de bits)
                (*opcode as u16) << 12
                    | (*rd as u16) << 8
                    | (*rs1 as u16) << 4
                    | ((*imm as u16) & 0xF)
            }
            Instruction::SType {
                opcode,
                imm,
                rs1,
                rs2,
            } => {
                // Formato S: opcode[15:12], imm[11:8], rs1[7:4], rs2[3:0]
                (*opcode as u16) << 12
                    | ((*imm as u16) & 0xF) << 8
                    | (*rs1 as u16) << 4
                    | (*rs2 as u16)
            }
            Instruction::BType {
                opcode,
                imm,
                rs1,
                rs2,
            } => {
                // Formato B: opcode[15:12], imm[11:8], rs1[7:4], rs2[3:0]
                (*opcode as u16) << 12
                    | ((*imm as u16) & 0xF) << 8
                    | (*rs1 as u16) << 4
                    | (*rs2 as u16)
            }
            // Formato J: opcode[15:12], imm[11:0]
            Instruction::JType { opcode, imm } => (*opcode as u16) << 12 | (*imm as u16) & 0xFFF,
        }
    }
}
