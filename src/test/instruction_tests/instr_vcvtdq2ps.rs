use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvtdq2ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 91, 223], OperandSize::Dword)
}

#[test]
fn vcvtdq2ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PS, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, EDI, Two, 803481327, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 91, 164, 123, 239, 38, 228, 47], OperandSize::Dword)
}

#[test]
fn vcvtdq2ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 91, 226], OperandSize::Qword)
}

#[test]
fn vcvtdq2ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PS, operand1: Some(Direct(XMM1)), operand2: Some(IndirectDisplaced(RDX, 383167451, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 91, 138, 219, 171, 214, 22], OperandSize::Qword)
}

#[test]
fn vcvtdq2ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 91, 255], OperandSize::Dword)
}

#[test]
fn vcvtdq2ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PS, operand1: Some(Direct(YMM4)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, EDX, Two, 1823109086, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 91, 164, 87, 222, 111, 170, 108], OperandSize::Dword)
}

#[test]
fn vcvtdq2ps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 91, 199], OperandSize::Qword)
}

#[test]
fn vcvtdq2ps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PS, operand1: Some(Direct(YMM4)), operand2: Some(IndirectScaledIndexed(RDX, RBX, Eight, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 91, 36, 218], OperandSize::Qword)
}

#[test]
fn vcvtdq2ps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 124, 138, 91, 202], OperandSize::Dword)
}

#[test]
fn vcvtdq2ps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PS, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexed(EDX, EAX, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 124, 142, 91, 4, 66], OperandSize::Dword)
}

#[test]
fn vcvtdq2ps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PS, operand1: Some(Direct(XMM25)), operand2: Some(Direct(XMM18)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 33, 124, 142, 91, 202], OperandSize::Qword)
}

#[test]
fn vcvtdq2ps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PS, operand1: Some(Direct(XMM8)), operand2: Some(IndirectScaledDisplaced(RAX, Four, 2103183967, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 113, 124, 139, 91, 4, 133, 95, 10, 92, 125], OperandSize::Qword)
}

#[test]
fn vcvtdq2ps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 124, 174, 91, 197], OperandSize::Dword)
}

#[test]
fn vcvtdq2ps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PS, operand1: Some(Direct(YMM5)), operand2: Some(IndirectDisplaced(EDI, 1928586053, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 124, 169, 91, 175, 69, 227, 243, 114], OperandSize::Dword)
}

#[test]
fn vcvtdq2ps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PS, operand1: Some(Direct(YMM24)), operand2: Some(Direct(YMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 97, 124, 175, 91, 198], OperandSize::Qword)
}

#[test]
fn vcvtdq2ps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PS, operand1: Some(Direct(YMM21)), operand2: Some(IndirectDisplaced(RCX, 1751671917, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 225, 124, 171, 91, 169, 109, 100, 104, 104], OperandSize::Qword)
}

#[test]
fn vcvtdq2ps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PS, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM1)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 124, 187, 91, 225], OperandSize::Dword)
}

#[test]
fn vcvtdq2ps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PS, operand1: Some(Direct(ZMM6)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, EAX, Four, 2125156906, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 124, 205, 91, 180, 135, 42, 82, 171, 126], OperandSize::Dword)
}

#[test]
fn vcvtdq2ps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PS, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM21)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 177, 124, 220, 91, 197], OperandSize::Qword)
}

#[test]
fn vcvtdq2ps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTDQ2PS, operand1: Some(Direct(ZMM1)), operand2: Some(Indirect(RSI, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 124, 206, 91, 14], OperandSize::Qword)
}

