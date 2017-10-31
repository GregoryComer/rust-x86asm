use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvtps2uqq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2UQQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 125, 137, 121, 232], OperandSize::Dword)
}

#[test]
fn vcvtps2uqq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2UQQ, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledDisplaced(EDI, Two, 485344873, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 125, 140, 121, 36, 125, 105, 198, 237, 28], OperandSize::Dword)
}

#[test]
fn vcvtps2uqq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2UQQ, operand1: Some(Direct(XMM27)), operand2: Some(Direct(XMM14)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 65, 125, 141, 121, 222], OperandSize::Qword)
}

#[test]
fn vcvtps2uqq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2UQQ, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexed(RBX, RDX, Eight, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 125, 143, 121, 60, 211], OperandSize::Qword)
}

#[test]
fn vcvtps2uqq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2UQQ, operand1: Some(Direct(YMM1)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 125, 170, 121, 204], OperandSize::Dword)
}

#[test]
fn vcvtps2uqq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2UQQ, operand1: Some(Direct(YMM3)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, EDI, Two, 1525580501, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 125, 171, 121, 156, 127, 213, 130, 238, 90], OperandSize::Dword)
}

#[test]
fn vcvtps2uqq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2UQQ, operand1: Some(Direct(YMM0)), operand2: Some(Direct(XMM30)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 145, 125, 169, 121, 198], OperandSize::Qword)
}

#[test]
fn vcvtps2uqq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2UQQ, operand1: Some(Direct(YMM12)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RDX, Eight, 1458014430, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 113, 125, 173, 121, 164, 211, 222, 136, 231, 86], OperandSize::Qword)
}

#[test]
fn vcvtps2uqq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2UQQ, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 125, 189, 121, 245], OperandSize::Dword)
}

#[test]
fn vcvtps2uqq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2UQQ, operand1: Some(Direct(ZMM4)), operand2: Some(IndirectDisplaced(EAX, 1414988969, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 125, 201, 121, 160, 169, 4, 87, 84], OperandSize::Dword)
}

#[test]
fn vcvtps2uqq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2UQQ, operand1: Some(Direct(ZMM31)), operand2: Some(Direct(YMM30)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 1, 125, 154, 121, 254], OperandSize::Qword)
}

#[test]
fn vcvtps2uqq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2UQQ, operand1: Some(Direct(ZMM25)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RSI, Two, 2073724767, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 97, 125, 207, 121, 140, 114, 95, 135, 154, 123], OperandSize::Qword)
}

