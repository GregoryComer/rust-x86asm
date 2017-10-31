use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvtps2uqq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2UQQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 125, 141, 121, 212], OperandSize::Dword)
}

#[test]
fn vcvtps2uqq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2UQQ, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledDisplaced(ECX, Four, 782247532, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 125, 143, 121, 52, 141, 108, 38, 160, 46], OperandSize::Dword)
}

#[test]
fn vcvtps2uqq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2UQQ, operand1: Some(Direct(XMM8)), operand2: Some(Direct(XMM27)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 17, 125, 142, 121, 195], OperandSize::Qword)
}

#[test]
fn vcvtps2uqq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2UQQ, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexed(RDI, RSI, Two, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 125, 137, 121, 60, 119], OperandSize::Qword)
}

#[test]
fn vcvtps2uqq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2UQQ, operand1: Some(Direct(YMM4)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 125, 171, 121, 230], OperandSize::Dword)
}

#[test]
fn vcvtps2uqq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2UQQ, operand1: Some(Direct(YMM0)), operand2: Some(IndirectScaledDisplaced(ESI, Four, 1742127559, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 125, 170, 121, 4, 181, 199, 193, 214, 103], OperandSize::Dword)
}

#[test]
fn vcvtps2uqq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2UQQ, operand1: Some(Direct(YMM12)), operand2: Some(Direct(XMM24)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 17, 125, 171, 121, 224], OperandSize::Qword)
}

#[test]
fn vcvtps2uqq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2UQQ, operand1: Some(Direct(YMM21)), operand2: Some(IndirectScaledDisplaced(RAX, Eight, 796565832, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 225, 125, 173, 121, 44, 197, 72, 161, 122, 47], OperandSize::Qword)
}

#[test]
fn vcvtps2uqq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2UQQ, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 125, 251, 121, 197], OperandSize::Dword)
}

#[test]
fn vcvtps2uqq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2UQQ, operand1: Some(Direct(ZMM5)), operand2: Some(IndirectDisplaced(EAX, 1019708652, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 125, 202, 121, 168, 236, 132, 199, 60], OperandSize::Dword)
}

#[test]
fn vcvtps2uqq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2UQQ, operand1: Some(Direct(ZMM27)), operand2: Some(Direct(YMM21)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 33, 125, 158, 121, 221], OperandSize::Qword)
}

#[test]
fn vcvtps2uqq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2UQQ, operand1: Some(Direct(ZMM25)), operand2: Some(IndirectDisplaced(RDI, 1515537008, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 97, 125, 203, 121, 143, 112, 66, 85, 90], OperandSize::Qword)
}

