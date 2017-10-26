use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvtqq2ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTQQ2PS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 252, 138, 91, 217], OperandSize::Dword)
}

#[test]
fn vcvtqq2ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTQQ2PS, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, EDI, Four, 1577159236, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 252, 142, 91, 188, 185, 68, 138, 1, 94], OperandSize::Dword)
}

#[test]
fn vcvtqq2ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTQQ2PS, operand1: Some(Direct(XMM19)), operand2: Some(Direct(XMM27)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 129, 252, 142, 91, 219], OperandSize::Qword)
}

#[test]
fn vcvtqq2ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTQQ2PS, operand1: Some(Direct(XMM21)), operand2: Some(IndirectDisplaced(RDX, 201677720, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 225, 252, 141, 91, 170, 152, 91, 5, 12], OperandSize::Qword)
}

#[test]
fn vcvtqq2ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTQQ2PS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 252, 172, 91, 194], OperandSize::Dword)
}

#[test]
fn vcvtqq2ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTQQ2PS, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, EAX, Eight, 1096147143, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 252, 171, 91, 156, 199, 199, 224, 85, 65], OperandSize::Dword)
}

#[test]
fn vcvtqq2ps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTQQ2PS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 252, 173, 91, 240], OperandSize::Qword)
}

#[test]
fn vcvtqq2ps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTQQ2PS, operand1: Some(Direct(XMM10)), operand2: Some(IndirectScaledIndexed(RDX, RSI, Two, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 113, 252, 174, 91, 20, 114], OperandSize::Qword)
}

#[test]
fn vcvtqq2ps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTQQ2PS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(ZMM3)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 252, 223, 91, 211], OperandSize::Dword)
}

#[test]
fn vcvtqq2ps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTQQ2PS, operand1: Some(Direct(YMM5)), operand2: Some(Indirect(EDI, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 252, 207, 91, 47], OperandSize::Dword)
}

#[test]
fn vcvtqq2ps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTQQ2PS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(ZMM15)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 209, 252, 223, 91, 231], OperandSize::Qword)
}

#[test]
fn vcvtqq2ps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTQQ2PS, operand1: Some(Direct(YMM8)), operand2: Some(IndirectScaledDisplaced(RCX, Eight, 1280583744, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 113, 252, 207, 91, 4, 205, 64, 40, 84, 76], OperandSize::Qword)
}

