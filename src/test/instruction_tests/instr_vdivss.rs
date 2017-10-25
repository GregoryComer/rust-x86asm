use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vdivss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVSS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 234, 94, 221], OperandSize::Dword)
}

#[test]
fn vdivss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVSS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectDisplaced(EDX, 1427155521, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 226, 94, 130, 65, 170, 16, 85], OperandSize::Dword)
}

#[test]
fn vdivss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVSS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 218, 94, 251], OperandSize::Qword)
}

#[test]
fn vdivss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVSS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(RAX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 194, 94, 40], OperandSize::Qword)
}

#[test]
fn vdivss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVSS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 118, 191, 94, 217], OperandSize::Dword)
}

#[test]
fn vdivss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVSS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledDisplaced(ECX, Two, 1424332117, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 86, 139, 94, 52, 77, 85, 149, 229, 84], OperandSize::Dword)
}

#[test]
fn vdivss_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVSS, operand1: Some(Direct(XMM11)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM31)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 17, 110, 187, 94, 223], OperandSize::Qword)
}

#[test]
fn vdivss_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVSS, operand1: Some(Direct(XMM21)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RDI, Eight, 883179145, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 225, 86, 139, 94, 172, 255, 137, 62, 164, 52], OperandSize::Qword)
}

