use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmovsxwq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 36, 218], OperandSize::Dword)
}

#[test]
fn vpmovsxwq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWQ, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledDisplaced(ESI, Four, 1067930084, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 36, 44, 181, 228, 81, 167, 63], OperandSize::Dword)
}

#[test]
fn vpmovsxwq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 36, 239], OperandSize::Qword)
}

#[test]
fn vpmovsxwq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWQ, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexed(RDI, RDI, Two, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 36, 52, 127], OperandSize::Qword)
}

#[test]
fn vpmovsxwq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWQ, operand1: Some(Direct(YMM7)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 36, 250], OperandSize::Dword)
}

#[test]
fn vpmovsxwq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWQ, operand1: Some(Direct(YMM2)), operand2: Some(IndirectScaledIndexed(ECX, EDX, Four, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 36, 20, 145], OperandSize::Dword)
}

#[test]
fn vpmovsxwq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWQ, operand1: Some(Direct(YMM7)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 36, 248], OperandSize::Qword)
}

#[test]
fn vpmovsxwq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWQ, operand1: Some(Direct(YMM2)), operand2: Some(IndirectScaledIndexed(RDX, RCX, Eight, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 36, 20, 202], OperandSize::Qword)
}

#[test]
fn vpmovsxwq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 143, 36, 201], OperandSize::Dword)
}

#[test]
fn vpmovsxwq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWQ, operand1: Some(Direct(XMM6)), operand2: Some(IndirectDisplaced(ESI, 1022895726, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 141, 36, 182, 110, 38, 248, 60], OperandSize::Dword)
}

#[test]
fn vpmovsxwq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWQ, operand1: Some(Direct(XMM11)), operand2: Some(Direct(XMM30)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 18, 125, 139, 36, 222], OperandSize::Qword)
}

#[test]
fn vpmovsxwq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWQ, operand1: Some(Direct(XMM23)), operand2: Some(IndirectScaledIndexed(RCX, RBX, Eight, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 226, 125, 138, 36, 60, 217], OperandSize::Qword)
}

#[test]
fn vpmovsxwq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWQ, operand1: Some(Direct(YMM2)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 170, 36, 214], OperandSize::Dword)
}

#[test]
fn vpmovsxwq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWQ, operand1: Some(Direct(YMM3)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, EDX, Four, 497815076, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 171, 36, 156, 150, 36, 14, 172, 29], OperandSize::Dword)
}

#[test]
fn vpmovsxwq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWQ, operand1: Some(Direct(YMM29)), operand2: Some(Direct(XMM20)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 34, 125, 175, 36, 236], OperandSize::Qword)
}

#[test]
fn vpmovsxwq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWQ, operand1: Some(Direct(YMM4)), operand2: Some(IndirectScaledIndexed(RCX, RDI, Four, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 171, 36, 36, 185], OperandSize::Qword)
}

#[test]
fn vpmovsxwq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWQ, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 205, 36, 222], OperandSize::Dword)
}

#[test]
fn vpmovsxwq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWQ, operand1: Some(Direct(ZMM3)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, EDX, Two, 1574070056, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 207, 36, 156, 86, 40, 103, 210, 93], OperandSize::Dword)
}

#[test]
fn vpmovsxwq_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWQ, operand1: Some(Direct(ZMM27)), operand2: Some(Direct(XMM14)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 66, 125, 203, 36, 222], OperandSize::Qword)
}

#[test]
fn vpmovsxwq_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWQ, operand1: Some(Direct(ZMM28)), operand2: Some(IndirectScaledDisplaced(RCX, Eight, 540935259, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 98, 125, 204, 36, 36, 205, 91, 4, 62, 32], OperandSize::Qword)
}

