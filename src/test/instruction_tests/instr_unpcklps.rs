use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn unpcklps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::UNPCKLPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 20, 247], OperandSize::Dword)
}

#[test]
fn unpcklps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::UNPCKLPS, operand1: Some(Direct(XMM2)), operand2: Some(Indirect(EAX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 20, 16], OperandSize::Dword)
}

#[test]
fn unpcklps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::UNPCKLPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 20, 229], OperandSize::Qword)
}

#[test]
fn unpcklps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::UNPCKLPS, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RAX, Four, 1154504875, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 20, 180, 130, 171, 88, 208, 68], OperandSize::Qword)
}

