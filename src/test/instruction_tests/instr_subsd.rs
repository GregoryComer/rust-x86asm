use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn subsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SUBSD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 92, 213], OperandSize::Dword)
}

#[test]
fn subsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SUBSD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexed(EBX, EDI, Eight, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 92, 36, 251], OperandSize::Dword)
}

#[test]
fn subsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SUBSD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 92, 196], OperandSize::Qword)
}

#[test]
fn subsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SUBSD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectDisplaced(RBX, 212302300, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 92, 163, 220, 121, 167, 12], OperandSize::Qword)
}

