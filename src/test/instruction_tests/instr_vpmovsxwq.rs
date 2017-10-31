use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmovsxwq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 36, 239], OperandSize::Dword)
}

#[test]
fn vpmovsxwq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWQ, operand1: Some(Direct(XMM4)), operand2: Some(IndirectDisplaced(ECX, 1543976814, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 36, 161, 110, 55, 7, 92], OperandSize::Dword)
}

#[test]
fn vpmovsxwq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 36, 212], OperandSize::Qword)
}

#[test]
fn vpmovsxwq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWQ, operand1: Some(Direct(XMM1)), operand2: Some(Indirect(RDX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 36, 10], OperandSize::Qword)
}

#[test]
fn vpmovsxwq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWQ, operand1: Some(Direct(YMM2)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 36, 209], OperandSize::Dword)
}

#[test]
fn vpmovsxwq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWQ, operand1: Some(Direct(YMM1)), operand2: Some(IndirectDisplaced(ECX, 1420021897, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 36, 137, 137, 208, 163, 84], OperandSize::Dword)
}

#[test]
fn vpmovsxwq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWQ, operand1: Some(Direct(YMM7)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 36, 249], OperandSize::Qword)
}

#[test]
fn vpmovsxwq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWQ, operand1: Some(Direct(YMM5)), operand2: Some(IndirectDisplaced(RDI, 1545442914, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 36, 175, 98, 150, 29, 92], OperandSize::Qword)
}

#[test]
fn vpmovsxwq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 139, 36, 243], OperandSize::Dword)
}

#[test]
fn vpmovsxwq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWQ, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, EDI, Eight, 84949752, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 143, 36, 172, 250, 248, 58, 16, 5], OperandSize::Dword)
}

#[test]
fn vpmovsxwq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM12)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 210, 125, 139, 36, 252], OperandSize::Qword)
}

#[test]
fn vpmovsxwq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWQ, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexed(RCX, RSI, Eight, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 138, 36, 20, 241], OperandSize::Qword)
}

#[test]
fn vpmovsxwq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWQ, operand1: Some(Direct(YMM3)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 171, 36, 219], OperandSize::Dword)
}

#[test]
fn vpmovsxwq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWQ, operand1: Some(Direct(YMM1)), operand2: Some(Indirect(EDX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 171, 36, 10], OperandSize::Dword)
}

#[test]
fn vpmovsxwq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWQ, operand1: Some(Direct(YMM9)), operand2: Some(Direct(XMM15)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 82, 125, 171, 36, 207], OperandSize::Qword)
}

#[test]
fn vpmovsxwq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWQ, operand1: Some(Direct(YMM23)), operand2: Some(IndirectScaledIndexed(RCX, RDI, Two, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 226, 125, 169, 36, 60, 121], OperandSize::Qword)
}

#[test]
fn vpmovsxwq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWQ, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 206, 36, 251], OperandSize::Dword)
}

#[test]
fn vpmovsxwq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWQ, operand1: Some(Direct(ZMM4)), operand2: Some(IndirectScaledIndexed(EAX, EAX, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 203, 36, 36, 128], OperandSize::Dword)
}

#[test]
fn vpmovsxwq_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWQ, operand1: Some(Direct(ZMM25)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 98, 125, 203, 36, 205], OperandSize::Qword)
}

#[test]
fn vpmovsxwq_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWQ, operand1: Some(Direct(ZMM21)), operand2: Some(IndirectScaledIndexed(RCX, RAX, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 226, 125, 201, 36, 44, 65], OperandSize::Qword)
}

