use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn minsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MINSD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 93, 234], OperandSize::Dword)
}

#[test]
fn minsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MINSD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledDisplaced(EDX, Four, 752204915, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 93, 36, 149, 115, 188, 213, 44], OperandSize::Dword)
}

#[test]
fn minsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MINSD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 93, 218], OperandSize::Qword)
}

#[test]
fn minsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MINSD, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RDI, Two, 1596698908, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 93, 172, 121, 28, 177, 43, 95], OperandSize::Qword)
}

