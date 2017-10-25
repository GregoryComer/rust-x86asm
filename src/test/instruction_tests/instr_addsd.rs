use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn addsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::ADDSD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 88, 255], OperandSize::Dword)
}

#[test]
fn addsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::ADDSD, operand1: Some(Direct(XMM0)), operand2: Some(IndirectDisplaced(ECX, 1592279181, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 88, 129, 141, 64, 232, 94], OperandSize::Dword)
}

#[test]
fn addsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::ADDSD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 88, 240], OperandSize::Qword)
}

#[test]
fn addsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::ADDSD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectDisplaced(RBX, 208129928, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 88, 163, 136, 207, 103, 12], OperandSize::Qword)
}

