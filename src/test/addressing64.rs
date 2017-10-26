use ::{Mnemonic, Operand, Reg, OperandSize, SegmentReg, MaskReg, BroadcastMode, MergeMode, RoundingMode, RegScale};
use ::test::{encode64_helper2};

#[test]
fn encode_indirect() {
    encode64_helper2(Mnemonic::ADD, Operand::Direct(Reg::RAX), Operand::Indirect(Reg::RAX, Some(OperandSize::Qword), None), &vec![0x48, 0x03, 0x00]); // ADD RAX, QWORD PTR [RAX]
    encode64_helper2(Mnemonic::ADD, Operand::Direct(Reg::RAX), Operand::Indirect(Reg::RBX, Some(OperandSize::Qword), None), &vec![0x48, 0x03, 0x03]); // ADD RAX, QWORD PTR [RBX]
    encode64_helper2(Mnemonic::ADD, Operand::Direct(Reg::RAX), Operand::Indirect(Reg::RCX, Some(OperandSize::Qword), None), &vec![0x48, 0x03, 0x01]); // ADD RAX, QWORD PTR [RCX]
    encode64_helper2(Mnemonic::ADD, Operand::Direct(Reg::RAX), Operand::Indirect(Reg::RDX, Some(OperandSize::Qword), None), &vec![0x48, 0x03, 0x02]); // ADD RAX, QWORD PTR [RDX]
    encode64_helper2(Mnemonic::ADD, Operand::Direct(Reg::RAX), Operand::Indirect(Reg::RDI, Some(OperandSize::Qword), None), &vec![0x48, 0x03, 0x07]); // ADD RAX, QWORD PTR [RDI]
    encode64_helper2(Mnemonic::ADD, Operand::Direct(Reg::RAX), Operand::Indirect(Reg::RSI, Some(OperandSize::Qword), None), &vec![0x48, 0x03, 0x06]); // ADD RAX, QWORD PTR [RSI]
    encode64_helper2(Mnemonic::ADD, Operand::Direct(Reg::RAX), Operand::Indirect(Reg::RSP, Some(OperandSize::Qword), None), &vec![0x48, 0x03, 0x04, 0x24]); // ADD RAX, QWORD PTR [RSP]
    encode64_helper2(Mnemonic::ADD, Operand::Direct(Reg::RAX), Operand::Indirect(Reg::RBP, Some(OperandSize::Qword), None), &vec![0x48, 0x03, 0x45, 0x00]); // ADD RAX, QWORD PTR [RBP]
}
