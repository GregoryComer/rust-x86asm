use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmovsxdq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXDQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 37, 230], OperandSize::Dword)
}

#[test]
fn vpmovsxdq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXDQ, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexed(ECX, EDI, Four, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 37, 20, 185], OperandSize::Dword)
}

#[test]
fn vpmovsxdq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXDQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 37, 205], OperandSize::Qword)
}

#[test]
fn vpmovsxdq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXDQ, operand1: Some(Direct(XMM4)), operand2: Some(Indirect(RAX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 37, 32], OperandSize::Qword)
}

#[test]
fn vpmovsxdq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXDQ, operand1: Some(Direct(YMM4)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 37, 226], OperandSize::Dword)
}

#[test]
fn vpmovsxdq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXDQ, operand1: Some(Direct(YMM2)), operand2: Some(Indirect(EBX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 37, 19], OperandSize::Dword)
}

#[test]
fn vpmovsxdq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXDQ, operand1: Some(Direct(YMM3)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 37, 223], OperandSize::Qword)
}

#[test]
fn vpmovsxdq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXDQ, operand1: Some(Direct(YMM2)), operand2: Some(IndirectScaledIndexed(RSI, RCX, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 37, 20, 78], OperandSize::Qword)
}

#[test]
fn vpmovsxdq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXDQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 139, 37, 200], OperandSize::Dword)
}

#[test]
fn vpmovsxdq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXDQ, operand1: Some(Direct(XMM5)), operand2: Some(IndirectDisplaced(ECX, 696792954, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 138, 37, 169, 122, 55, 136, 41], OperandSize::Dword)
}

#[test]
fn vpmovsxdq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXDQ, operand1: Some(Direct(XMM9)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 114, 125, 140, 37, 204], OperandSize::Qword)
}

#[test]
fn vpmovsxdq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXDQ, operand1: Some(Direct(XMM11)), operand2: Some(Indirect(RBX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 114, 125, 143, 37, 27], OperandSize::Qword)
}

#[test]
fn vpmovsxdq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXDQ, operand1: Some(Direct(YMM2)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 174, 37, 208], OperandSize::Dword)
}

#[test]
fn vpmovsxdq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXDQ, operand1: Some(Direct(YMM2)), operand2: Some(IndirectDisplaced(ESI, 1833622262, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 170, 37, 150, 246, 218, 74, 109], OperandSize::Dword)
}

#[test]
fn vpmovsxdq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXDQ, operand1: Some(Direct(YMM6)), operand2: Some(Direct(XMM21)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 178, 125, 172, 37, 245], OperandSize::Qword)
}

#[test]
fn vpmovsxdq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXDQ, operand1: Some(Direct(YMM22)), operand2: Some(Indirect(RSI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 226, 125, 172, 37, 54], OperandSize::Qword)
}

#[test]
fn vpmovsxdq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXDQ, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 202, 37, 240], OperandSize::Dword)
}

#[test]
fn vpmovsxdq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXDQ, operand1: Some(Direct(ZMM6)), operand2: Some(IndirectScaledDisplaced(ECX, Eight, 84646494, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 202, 37, 52, 205, 94, 154, 11, 5], OperandSize::Dword)
}

#[test]
fn vpmovsxdq_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXDQ, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 203, 37, 195], OperandSize::Qword)
}

#[test]
fn vpmovsxdq_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXDQ, operand1: Some(Direct(ZMM2)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RDI, Eight, 450139750, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 203, 37, 148, 248, 102, 150, 212, 26], OperandSize::Qword)
}

