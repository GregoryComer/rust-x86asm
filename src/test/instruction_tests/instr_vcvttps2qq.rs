use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvttps2qq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2QQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 125, 138, 122, 252], OperandSize::Dword)
}

#[test]
fn vcvttps2qq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2QQ, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexed(EAX, EDX, Two, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 125, 137, 122, 12, 80], OperandSize::Dword)
}

#[test]
fn vcvttps2qq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2QQ, operand1: Some(Direct(XMM14)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 113, 125, 142, 122, 244], OperandSize::Qword)
}

#[test]
fn vcvttps2qq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2QQ, operand1: Some(Direct(XMM29)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RDI, Four, 1188827522, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 97, 125, 141, 122, 172, 190, 130, 17, 220, 70], OperandSize::Qword)
}

#[test]
fn vcvttps2qq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2QQ, operand1: Some(Direct(YMM3)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 125, 170, 122, 219], OperandSize::Dword)
}

#[test]
fn vcvttps2qq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2QQ, operand1: Some(Direct(YMM7)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, EBX, Two, 1678638803, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 125, 174, 122, 188, 94, 211, 254, 13, 100], OperandSize::Dword)
}

#[test]
fn vcvttps2qq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2QQ, operand1: Some(Direct(YMM17)), operand2: Some(Direct(XMM28)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 129, 125, 171, 122, 204], OperandSize::Qword)
}

#[test]
fn vcvttps2qq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2QQ, operand1: Some(Direct(YMM19)), operand2: Some(IndirectScaledDisplaced(RBX, Two, 716698632, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 225, 125, 175, 122, 28, 93, 8, 244, 183, 42], OperandSize::Qword)
}

#[test]
fn vcvttps2qq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2QQ, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 125, 156, 122, 236], OperandSize::Dword)
}

#[test]
fn vcvttps2qq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2QQ, operand1: Some(Direct(ZMM1)), operand2: Some(IndirectDisplaced(EDX, 1590502440, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 125, 204, 122, 138, 40, 36, 205, 94], OperandSize::Dword)
}

#[test]
fn vcvttps2qq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2QQ, operand1: Some(Direct(ZMM27)), operand2: Some(Direct(YMM10)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K6), broadcast: None }, &[98, 65, 125, 158, 122, 218], OperandSize::Qword)
}

#[test]
fn vcvttps2qq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2QQ, operand1: Some(Direct(ZMM2)), operand2: Some(IndirectDisplaced(RDI, 175276331, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 125, 205, 122, 151, 43, 129, 114, 10], OperandSize::Qword)
}

