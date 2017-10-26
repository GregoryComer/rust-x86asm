use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn mulsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MULSD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 89, 196], OperandSize::Dword)
}

#[test]
fn mulsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MULSD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledDisplaced(EBX, Two, 1541843153, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 89, 36, 93, 209, 168, 230, 91], OperandSize::Dword)
}

#[test]
fn mulsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MULSD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 89, 248], OperandSize::Qword)
}

#[test]
fn mulsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MULSD, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexed(RDI, RCX, Two, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 89, 12, 79], OperandSize::Qword)
}

