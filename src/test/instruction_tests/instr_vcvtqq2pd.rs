use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvtqq2pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTQQ2PD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 254, 143, 230, 237], OperandSize::Dword)
}

#[test]
fn vcvtqq2pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTQQ2PD, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexed(EDI, EBX, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 254, 142, 230, 20, 95], OperandSize::Dword)
}

#[test]
fn vcvtqq2pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTQQ2PD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM19)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 177, 254, 140, 230, 243], OperandSize::Qword)
}

#[test]
fn vcvtqq2pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTQQ2PD, operand1: Some(Direct(XMM3)), operand2: Some(Indirect(RSI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 254, 141, 230, 30], OperandSize::Qword)
}

#[test]
fn vcvtqq2pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTQQ2PD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 254, 175, 230, 239], OperandSize::Dword)
}

#[test]
fn vcvtqq2pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTQQ2PD, operand1: Some(Direct(YMM5)), operand2: Some(Indirect(ESI, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 254, 174, 230, 46], OperandSize::Dword)
}

#[test]
fn vcvtqq2pd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTQQ2PD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM9)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 209, 254, 175, 230, 241], OperandSize::Qword)
}

#[test]
fn vcvtqq2pd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTQQ2PD, operand1: Some(Direct(YMM25)), operand2: Some(Indirect(RAX, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 97, 254, 169, 230, 8], OperandSize::Qword)
}

#[test]
fn vcvtqq2pd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTQQ2PD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM5)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 254, 218, 230, 245], OperandSize::Dword)
}

#[test]
fn vcvtqq2pd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTQQ2PD, operand1: Some(Direct(ZMM1)), operand2: Some(Indirect(EBX, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 254, 205, 230, 11], OperandSize::Dword)
}

#[test]
fn vcvtqq2pd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTQQ2PD, operand1: Some(Direct(ZMM21)), operand2: Some(Direct(ZMM1)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 225, 254, 159, 230, 233], OperandSize::Qword)
}

#[test]
fn vcvtqq2pd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTQQ2PD, operand1: Some(Direct(ZMM11)), operand2: Some(Indirect(RBX, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 113, 254, 204, 230, 27], OperandSize::Qword)
}

