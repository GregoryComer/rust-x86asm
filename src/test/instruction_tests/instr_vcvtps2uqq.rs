use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvtps2uqq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2UQQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 125, 137, 121, 249], OperandSize::Dword)
}

#[test]
fn vcvtps2uqq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2UQQ, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, EBX, Two, 245866793, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 125, 141, 121, 180, 89, 41, 161, 167, 14], OperandSize::Dword)
}

#[test]
fn vcvtps2uqq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2UQQ, operand1: Some(Direct(XMM19)), operand2: Some(Direct(XMM21)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 161, 125, 143, 121, 221], OperandSize::Qword)
}

#[test]
fn vcvtps2uqq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2UQQ, operand1: Some(Direct(XMM27)), operand2: Some(IndirectScaledDisplaced(RAX, Eight, 1892852893, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 97, 125, 141, 121, 28, 197, 157, 164, 210, 112], OperandSize::Qword)
}

#[test]
fn vcvtps2uqq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2UQQ, operand1: Some(Direct(YMM5)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 125, 172, 121, 237], OperandSize::Dword)
}

#[test]
fn vcvtps2uqq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2UQQ, operand1: Some(Direct(YMM3)), operand2: Some(IndirectScaledIndexed(ECX, ECX, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 125, 172, 121, 28, 73], OperandSize::Dword)
}

#[test]
fn vcvtps2uqq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2UQQ, operand1: Some(Direct(YMM17)), operand2: Some(Direct(XMM25)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 129, 125, 169, 121, 201], OperandSize::Qword)
}

#[test]
fn vcvtps2uqq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2UQQ, operand1: Some(Direct(YMM10)), operand2: Some(Indirect(RSI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 113, 125, 172, 121, 22], OperandSize::Qword)
}

#[test]
fn vcvtps2uqq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2UQQ, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 125, 155, 121, 216], OperandSize::Dword)
}

#[test]
fn vcvtps2uqq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2UQQ, operand1: Some(Direct(ZMM3)), operand2: Some(IndirectScaledDisplaced(EDI, Two, 137806205, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 125, 204, 121, 28, 125, 125, 193, 54, 8], OperandSize::Dword)
}

#[test]
fn vcvtps2uqq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2UQQ, operand1: Some(Direct(ZMM27)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 97, 125, 191, 121, 221], OperandSize::Qword)
}

#[test]
fn vcvtps2uqq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2UQQ, operand1: Some(Direct(ZMM21)), operand2: Some(IndirectScaledDisplaced(RDX, Four, 1033457677, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 225, 125, 202, 121, 44, 149, 13, 80, 153, 61], OperandSize::Qword)
}

