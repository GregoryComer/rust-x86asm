use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvtps2qq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2QQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 125, 142, 123, 233], OperandSize::Dword)
}

#[test]
fn vcvtps2qq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2QQ, operand1: Some(Direct(XMM7)), operand2: Some(IndirectDisplaced(EDI, 1112869589, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 125, 138, 123, 191, 213, 10, 85, 66], OperandSize::Dword)
}

#[test]
fn vcvtps2qq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2QQ, operand1: Some(Direct(XMM29)), operand2: Some(Direct(XMM22)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 33, 125, 143, 123, 238], OperandSize::Qword)
}

#[test]
fn vcvtps2qq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2QQ, operand1: Some(Direct(XMM13)), operand2: Some(IndirectDisplaced(RSI, 2078130837, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 113, 125, 138, 123, 174, 149, 194, 221, 123], OperandSize::Qword)
}

#[test]
fn vcvtps2qq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2QQ, operand1: Some(Direct(YMM7)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 125, 170, 123, 253], OperandSize::Dword)
}

#[test]
fn vcvtps2qq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2QQ, operand1: Some(Direct(YMM3)), operand2: Some(IndirectDisplaced(EDI, 981628545, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 125, 174, 123, 159, 129, 118, 130, 58], OperandSize::Dword)
}

#[test]
fn vcvtps2qq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2QQ, operand1: Some(Direct(YMM22)), operand2: Some(Direct(XMM10)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 193, 125, 170, 123, 242], OperandSize::Qword)
}

#[test]
fn vcvtps2qq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2QQ, operand1: Some(Direct(YMM14)), operand2: Some(IndirectScaledDisplaced(RCX, Four, 890277277, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 113, 125, 169, 123, 52, 141, 157, 141, 16, 53], OperandSize::Qword)
}

#[test]
fn vcvtps2qq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2QQ, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 125, 250, 123, 193], OperandSize::Dword)
}

#[test]
fn vcvtps2qq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2QQ, operand1: Some(Direct(ZMM3)), operand2: Some(IndirectDisplaced(ECX, 531705603, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 125, 201, 123, 153, 3, 47, 177, 31], OperandSize::Dword)
}

#[test]
fn vcvtps2qq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2QQ, operand1: Some(Direct(ZMM25)), operand2: Some(Direct(YMM14)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 65, 125, 157, 123, 206], OperandSize::Qword)
}

#[test]
fn vcvtps2qq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2QQ, operand1: Some(Direct(ZMM17)), operand2: Some(IndirectScaledDisplaced(RAX, Eight, 535789588, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 225, 125, 202, 123, 12, 197, 20, 128, 239, 31], OperandSize::Qword)
}

