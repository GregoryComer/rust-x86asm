use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvtph2ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPH2PS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 19, 236], OperandSize::Dword)
}

#[test]
fn vcvtph2ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPH2PS, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexed(ECX, ECX, Eight, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 19, 12, 201], OperandSize::Dword)
}

#[test]
fn vcvtph2ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPH2PS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 19, 249], OperandSize::Qword)
}

#[test]
fn vcvtph2ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPH2PS, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RSI, Eight, 1522764812, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 19, 164, 241, 12, 140, 195, 90], OperandSize::Qword)
}

#[test]
fn vcvtph2ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPH2PS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 19, 213], OperandSize::Dword)
}

#[test]
fn vcvtph2ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPH2PS, operand1: Some(Direct(YMM6)), operand2: Some(IndirectScaledIndexed(EBX, EDI, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 19, 52, 123], OperandSize::Dword)
}

#[test]
fn vcvtph2ps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPH2PS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 19, 212], OperandSize::Qword)
}

#[test]
fn vcvtph2ps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPH2PS, operand1: Some(Direct(YMM5)), operand2: Some(IndirectScaledIndexed(RDI, RDI, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 19, 44, 191], OperandSize::Qword)
}

#[test]
fn vcvtph2ps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPH2PS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 141, 19, 194], OperandSize::Dword)
}

#[test]
fn vcvtph2ps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPH2PS, operand1: Some(Direct(XMM7)), operand2: Some(IndirectDisplaced(EBX, 64471010, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 142, 19, 187, 226, 191, 215, 3], OperandSize::Dword)
}

#[test]
fn vcvtph2ps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPH2PS, operand1: Some(Direct(XMM15)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 114, 125, 143, 19, 252], OperandSize::Qword)
}

#[test]
fn vcvtph2ps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPH2PS, operand1: Some(Direct(XMM8)), operand2: Some(Indirect(RDX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 114, 125, 140, 19, 2], OperandSize::Qword)
}

#[test]
fn vcvtph2ps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPH2PS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 171, 19, 243], OperandSize::Dword)
}

#[test]
fn vcvtph2ps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPH2PS, operand1: Some(Direct(YMM2)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, EDI, Eight, 319418636, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 171, 19, 148, 248, 12, 241, 9, 19], OperandSize::Dword)
}

#[test]
fn vcvtph2ps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPH2PS, operand1: Some(Direct(YMM28)), operand2: Some(Direct(XMM31)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 2, 125, 170, 19, 231], OperandSize::Qword)
}

#[test]
fn vcvtph2ps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPH2PS, operand1: Some(Direct(YMM19)), operand2: Some(Indirect(RBX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 226, 125, 173, 19, 27], OperandSize::Qword)
}

#[test]
fn vcvtph2ps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPH2PS, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 154, 19, 237], OperandSize::Dword)
}

#[test]
fn vcvtph2ps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPH2PS, operand1: Some(Direct(ZMM7)), operand2: Some(IndirectScaledIndexed(EAX, EDX, Two, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 205, 19, 60, 80], OperandSize::Dword)
}

#[test]
fn vcvtph2ps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPH2PS, operand1: Some(Direct(ZMM26)), operand2: Some(Direct(YMM11)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K1), broadcast: None }, &[98, 66, 125, 153, 19, 211], OperandSize::Qword)
}

#[test]
fn vcvtph2ps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPH2PS, operand1: Some(Direct(ZMM26)), operand2: Some(IndirectScaledIndexed(RBX, RSI, Two, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 98, 125, 207, 19, 20, 115], OperandSize::Qword)
}

