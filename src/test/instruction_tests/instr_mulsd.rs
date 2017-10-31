use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn mulsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MULSD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 89, 213], OperandSize::Dword)
}

#[test]
fn mulsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MULSD, operand1: Some(Direct(XMM2)), operand2: Some(Indirect(EDX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 89, 18], OperandSize::Dword)
}

#[test]
fn mulsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MULSD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 89, 205], OperandSize::Qword)
}

#[test]
fn mulsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MULSD, operand1: Some(Direct(XMM2)), operand2: Some(IndirectDisplaced(RDX, 465111408, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 89, 146, 112, 9, 185, 27], OperandSize::Qword)
}

