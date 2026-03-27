#[cfg(test)]
mod tests {
    use compiler::isa::{Instruction, OpCode, Register};

    // use super::*;

    #[test]
    fn test_encode_addi() {
        // addi r4, r0, 5 -> Opcode 1001 (9), rd 4, rs1 0, imm 5
        // Binario: 1001 0100 0000 0101 = 0x9405
        let inst = Instruction::IType {
            opcode: OpCode::Addi,
            rd: Register::A0, // r4 es a0
            rs1: Register::R0,
            imm: 5,
        };
        assert_eq!(inst.encode(), 0x9405);
    }

    #[test]
    fn test_encode_add_r_type() {
        // add r4, r0, r1 -> Opcode 0000, rs1 0, rd 4, rs2 1
        // Formato R: opcode[15:12], rs1[11:8], rd[7:4], rs2[3:0]
        // Binario: 0000 0000 0100 0001 = 0x0041
        let inst = Instruction::RType {
            opcode: OpCode::Add,
            rs1: Register::R0,
            rd: Register::A0,
            rs2: Register::Ra, // r1 es ra
        };
        assert_eq!(inst.encode(), 0x0041);
    }

    #[test]
    fn test_encode_jump_j_type() {
        // j 0xABC -> Opcode 1110 (E), imm 0xABC
        // Formato J: opcode[15:12], imm[11:0]
        // Binario: 1110 1010 1011 1100 = 0xEABC
        let inst = Instruction::JType {
            opcode: OpCode::J,
            imm: 0x0ABC,
        };
        assert_eq!(inst.encode(), 0xEABC);
    }

    #[test]
    fn test_encode_negative_imm_masking() {
        // Verificamos que un inmediato negativo no rompa el opcode
        // addi r1, r1, -1 (imm -1 es 0xF en 4 bits)
        // Opcode 9, rd 1, rs1 1, imm F -> 0x911F
        let inst = Instruction::IType {
            opcode: OpCode::Addi,
            rd: Register::Ra,
            rs1: Register::Ra,
            imm: -1,
        };
        assert_eq!(inst.encode(), 0x911F);
    }
}
