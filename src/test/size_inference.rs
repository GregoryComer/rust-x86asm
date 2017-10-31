use ::{Mnemonic, Operand, Reg, OperandSize, SegmentReg, MaskReg, BroadcastMode, MergeMode, RoundingMode, RegScale};
use ::test::{encode32_helper2, encode32_helper3, encode32_assert_ambiguous};

#[test]
fn infer_size_indirect_8bit_instr() {
    encode32_helper2(Mnemonic::ADD, Operand::Direct(Reg::AL), Operand::Indirect(Reg::EAX, None, None), &vec![0x02, 0x00]);
}

#[test]
fn infer_operand_size_16_32_instr() {
    encode32_helper2(Mnemonic::ADD, Operand::Direct(Reg::AX), Operand::Indirect(Reg::EBX, None, None), &vec![0x66, 0x03, 0x03]);
    encode32_helper2(Mnemonic::ADD, Operand::Direct(Reg::EAX), Operand::Indirect(Reg::EBX, None, None), &vec![0x03, 0x03]);
}

#[test]
fn infer_vector_size() {
    encode32_helper3(Mnemonic::VADDPD, Operand::Direct(Reg::XMM1), Operand::Direct(Reg::XMM2), Operand::Indirect(Reg::EAX, None, None), &vec![0xC5, 0xE9, 0x58, 0x08]); // VADDPD XMM1, XMM2, XMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VADDPD, Operand::Direct(Reg::YMM1), Operand::Direct(Reg::YMM2), Operand::Indirect(Reg::EAX, None, None), &vec![0xC5, 0xED, 0x58, 0x08]); // VADDPD YMM1, YMM2, YMMWORD PTR [EAX]
    encode32_helper3(Mnemonic::VADDPD, Operand::Direct(Reg::ZMM1), Operand::Direct(Reg::ZMM2), Operand::Indirect(Reg::EAX, None, None), &vec![0x62, 0xF1, 0xED, 0x48, 0x58, 0x08]); // VADDPD ZMM1, ZMM2, ZMMWORD PTR [EAX]
}

#[test]
fn notify_ambiguous_size_16_32() {
    encode32_assert_ambiguous(Mnemonic::NOT, Some(Operand::Indirect(Reg::EAX, None, None)), None, None, None);
}
