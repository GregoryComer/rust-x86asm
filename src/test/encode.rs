use ::*;
use ::test::*;

#[test]
fn extended_regs() {
    run_test(&Instruction::new2(Mnemonic::ADD, Operand::Direct(Reg::RAX), Operand::Direct(Reg::R8)), &[0x4C, 0x01, 0xC0], OperandSize::Qword);
    run_test(&Instruction::new2(Mnemonic::ADD, Operand::Direct(Reg::AL), Operand::Direct(Reg::SPL)), &[0x40, 0x00, 0xE0], OperandSize::Qword);
    run_test(&Instruction::new2(Mnemonic::ADD, Operand::Direct(Reg::AL), Operand::Direct(Reg::BPL)), &[0x40, 0x00, 0xE8], OperandSize::Qword);
    run_test(&Instruction::new2(Mnemonic::ADD, Operand::Direct(Reg::AL), Operand::Direct(Reg::SIL)), &[0x40, 0x00, 0xF0], OperandSize::Qword);
    run_test(&Instruction::new2(Mnemonic::ADD, Operand::Direct(Reg::AL), Operand::Direct(Reg::DIL)), &[0x40, 0x00, 0xF8], OperandSize::Qword);
}
