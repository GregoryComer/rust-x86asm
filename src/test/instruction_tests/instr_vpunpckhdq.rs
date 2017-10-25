use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpunpckhdq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHDQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 106, 211], OperandSize::Dword)
}

#[test]
fn vpunpckhdq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHDQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexed(ECX, ECX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 106, 20, 137], OperandSize::Dword)
}

#[test]
fn vpunpckhdq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHDQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 106, 238], OperandSize::Qword)
}

#[test]
fn vpunpckhdq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHDQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectDisplaced(RDI, 1692774994, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 106, 159, 82, 178, 229, 100], OperandSize::Qword)
}

#[test]
fn vpunpckhdq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHDQ, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 106, 238], OperandSize::Dword)
}

#[test]
fn vpunpckhdq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHDQ, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledDisplaced(ESI, Two, 1054902117, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 106, 36, 117, 101, 135, 224, 62], OperandSize::Dword)
}

#[test]
fn vpunpckhdq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHDQ, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 106, 218], OperandSize::Qword)
}

#[test]
fn vpunpckhdq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHDQ, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectDisplaced(RDI, 1334651393, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 106, 135, 1, 42, 141, 79], OperandSize::Qword)
}

#[test]
fn vpunpckhdq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHDQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 117, 142, 106, 194], OperandSize::Dword)
}

#[test]
fn vpunpckhdq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHDQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EDX, Eight, 271110842, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 117, 139, 106, 164, 215, 186, 210, 40, 16], OperandSize::Dword)
}

#[test]
fn vpunpckhdq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHDQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledDisplaced(ECX, Eight, 2045280267, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 101, 153, 106, 60, 205, 11, 128, 232, 121], OperandSize::Dword)
}

#[test]
fn vpunpckhdq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHDQ, operand1: Some(Direct(XMM21)), operand2: Some(Direct(XMM11)), operand3: Some(Direct(XMM26)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 129, 37, 143, 106, 234], OperandSize::Qword)
}

#[test]
fn vpunpckhdq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHDQ, operand1: Some(Direct(XMM21)), operand2: Some(Direct(XMM6)), operand3: Some(Indirect(RCX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 225, 77, 138, 106, 41], OperandSize::Qword)
}

#[test]
fn vpunpckhdq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHDQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM28)), operand3: Some(IndirectScaledDisplaced(RCX, Four, 161195577, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 29, 146, 106, 20, 141, 57, 166, 155, 9], OperandSize::Qword)
}

