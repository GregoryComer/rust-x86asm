use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmovsxwd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 35, 217], OperandSize::Dword)
}

#[test]
fn vpmovsxwd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWD, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledDisplaced(EDI, Two, 1429144073, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 35, 12, 125, 9, 2, 47, 85], OperandSize::Dword)
}

#[test]
fn vpmovsxwd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 35, 253], OperandSize::Qword)
}

#[test]
fn vpmovsxwd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWD, operand1: Some(Direct(XMM7)), operand2: Some(Indirect(RCX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 35, 57], OperandSize::Qword)
}

#[test]
fn vpmovsxwd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 35, 224], OperandSize::Dword)
}

#[test]
fn vpmovsxwd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWD, operand1: Some(Direct(YMM3)), operand2: Some(IndirectScaledIndexed(EAX, EDX, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 35, 28, 144], OperandSize::Dword)
}

#[test]
fn vpmovsxwd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 35, 212], OperandSize::Qword)
}

#[test]
fn vpmovsxwd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWD, operand1: Some(Direct(YMM7)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RBX, Eight, 1911785578, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 35, 188, 223, 106, 136, 243, 113], OperandSize::Qword)
}

#[test]
fn vpmovsxwd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 139, 35, 210], OperandSize::Dword)
}

#[test]
fn vpmovsxwd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWD, operand1: Some(Direct(XMM7)), operand2: Some(Indirect(EAX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 141, 35, 56], OperandSize::Dword)
}

#[test]
fn vpmovsxwd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWD, operand1: Some(Direct(XMM17)), operand2: Some(Direct(XMM16)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 162, 125, 138, 35, 200], OperandSize::Qword)
}

#[test]
fn vpmovsxwd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWD, operand1: Some(Direct(XMM17)), operand2: Some(Indirect(RDI, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 226, 125, 139, 35, 15], OperandSize::Qword)
}

#[test]
fn vpmovsxwd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 174, 35, 218], OperandSize::Dword)
}

#[test]
fn vpmovsxwd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWD, operand1: Some(Direct(YMM6)), operand2: Some(IndirectScaledDisplaced(ECX, Four, 668019745, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 175, 35, 52, 141, 33, 44, 209, 39], OperandSize::Dword)
}

#[test]
fn vpmovsxwd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(XMM14)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 210, 125, 170, 35, 246], OperandSize::Qword)
}

#[test]
fn vpmovsxwd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWD, operand1: Some(Direct(YMM1)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RDI, Four, 1590881658, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 171, 35, 140, 184, 122, 237, 210, 94], OperandSize::Qword)
}

#[test]
fn vpmovsxwd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 202, 35, 227], OperandSize::Dword)
}

#[test]
fn vpmovsxwd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWD, operand1: Some(Direct(ZMM0)), operand2: Some(IndirectDisplaced(EAX, 2066519622, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 202, 35, 128, 70, 150, 44, 123], OperandSize::Dword)
}

#[test]
fn vpmovsxwd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWD, operand1: Some(Direct(ZMM22)), operand2: Some(Direct(YMM25)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 130, 125, 202, 35, 241], OperandSize::Qword)
}

#[test]
fn vpmovsxwd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWD, operand1: Some(Direct(ZMM21)), operand2: Some(Indirect(RCX, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 226, 125, 207, 35, 41], OperandSize::Qword)
}

