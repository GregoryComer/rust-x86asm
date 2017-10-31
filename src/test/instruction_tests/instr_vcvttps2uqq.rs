use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvttps2uqq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2UQQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 125, 141, 120, 211], OperandSize::Dword)
}

#[test]
fn vcvttps2uqq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2UQQ, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, EBX, Eight, 569209048, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 125, 138, 120, 188, 219, 216, 112, 237, 33], OperandSize::Dword)
}

#[test]
fn vcvttps2uqq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2UQQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM24)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 145, 125, 141, 120, 224], OperandSize::Qword)
}

#[test]
fn vcvttps2uqq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2UQQ, operand1: Some(Direct(XMM24)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RSI, Eight, 2058766615, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 97, 125, 143, 120, 132, 241, 23, 73, 182, 122], OperandSize::Qword)
}

#[test]
fn vcvttps2uqq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2UQQ, operand1: Some(Direct(YMM3)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 125, 174, 120, 216], OperandSize::Dword)
}

#[test]
fn vcvttps2uqq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2UQQ, operand1: Some(Direct(YMM1)), operand2: Some(IndirectScaledIndexed(EBX, EDX, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 125, 170, 120, 12, 147], OperandSize::Dword)
}

#[test]
fn vcvttps2uqq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2UQQ, operand1: Some(Direct(YMM26)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 97, 125, 173, 120, 211], OperandSize::Qword)
}

#[test]
fn vcvttps2uqq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2UQQ, operand1: Some(Direct(YMM20)), operand2: Some(IndirectDisplaced(RDI, 1589501512, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 225, 125, 174, 120, 167, 72, 222, 189, 94], OperandSize::Qword)
}

#[test]
fn vcvttps2uqq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2UQQ, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 125, 156, 120, 249], OperandSize::Dword)
}

#[test]
fn vcvttps2uqq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2UQQ, operand1: Some(Direct(ZMM1)), operand2: Some(IndirectScaledIndexed(EAX, EDX, Two, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 125, 207, 120, 12, 80], OperandSize::Dword)
}

#[test]
fn vcvttps2uqq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2UQQ, operand1: Some(Direct(ZMM21)), operand2: Some(Direct(YMM18)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K4), broadcast: None }, &[98, 161, 125, 156, 120, 234], OperandSize::Qword)
}

#[test]
fn vcvttps2uqq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2UQQ, operand1: Some(Direct(ZMM25)), operand2: Some(IndirectScaledDisplaced(RDI, Four, 1353987029, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 97, 125, 202, 120, 12, 189, 213, 51, 180, 80], OperandSize::Qword)
}

