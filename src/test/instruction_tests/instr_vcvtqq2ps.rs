use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvtqq2ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTQQ2PS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 252, 141, 91, 246], OperandSize::Dword)
}

#[test]
fn vcvtqq2ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTQQ2PS, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledDisplaced(EAX, Four, 1575884867, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 252, 142, 91, 44, 133, 67, 24, 238, 93], OperandSize::Dword)
}

#[test]
fn vcvtqq2ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTQQ2PS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM15)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 209, 252, 137, 91, 231], OperandSize::Qword)
}

#[test]
fn vcvtqq2ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTQQ2PS, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexed(RSI, RBX, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 252, 140, 91, 44, 94], OperandSize::Qword)
}

#[test]
fn vcvtqq2ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTQQ2PS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 252, 172, 91, 236], OperandSize::Dword)
}

#[test]
fn vcvtqq2ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTQQ2PS, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledDisplaced(EDX, Eight, 1291750325, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 252, 172, 91, 28, 213, 181, 139, 254, 76], OperandSize::Dword)
}

#[test]
fn vcvtqq2ps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTQQ2PS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(YMM8)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 209, 252, 170, 91, 240], OperandSize::Qword)
}

#[test]
fn vcvtqq2ps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTQQ2PS, operand1: Some(Direct(XMM18)), operand2: Some(IndirectScaledDisplaced(RSI, Eight, 1254158283, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 225, 252, 175, 91, 20, 245, 203, 239, 192, 74], OperandSize::Qword)
}

#[test]
fn vcvtqq2ps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTQQ2PS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(ZMM6)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 252, 190, 91, 198], OperandSize::Dword)
}

#[test]
fn vcvtqq2ps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTQQ2PS, operand1: Some(Direct(YMM2)), operand2: Some(IndirectScaledDisplaced(EBX, Four, 1003915007, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 252, 207, 91, 20, 157, 255, 134, 214, 59], OperandSize::Dword)
}

#[test]
fn vcvtqq2ps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTQQ2PS, operand1: Some(Direct(YMM10)), operand2: Some(Direct(ZMM7)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 113, 252, 220, 91, 215], OperandSize::Qword)
}

#[test]
fn vcvtqq2ps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTQQ2PS, operand1: Some(Direct(YMM29)), operand2: Some(IndirectDisplaced(RBX, 559042356, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 97, 252, 205, 91, 171, 52, 79, 82, 33], OperandSize::Qword)
}

