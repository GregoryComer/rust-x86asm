use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vrsqrtss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRTSS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 202, 82, 211], OperandSize::Dword)
}

#[test]
fn vrsqrtss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRTSS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledDisplaced(EBX, Four, 443246371, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 234, 82, 44, 157, 35, 103, 107, 26], OperandSize::Dword)
}

#[test]
fn vrsqrtss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRTSS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 202, 82, 223], OperandSize::Qword)
}

#[test]
fn vrsqrtss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRTSS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectDisplaced(RDI, 1266858804, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 218, 82, 151, 52, 187, 130, 75], OperandSize::Qword)
}

