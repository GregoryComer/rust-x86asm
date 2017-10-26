use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn bndcl_1() {
    run_test(&Instruction { mnemonic: Mnemonic::BNDCL, operand1: Some(Direct(BND3)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 26, 222], OperandSize::Dword)
}

#[test]
fn bndcl_2() {
    run_test(&Instruction { mnemonic: Mnemonic::BNDCL, operand1: Some(Direct(BND1)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, EDX, Eight, 1506336112, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 26, 140, 208, 112, 221, 200, 89], OperandSize::Dword)
}

#[test]
fn bndcl_3() {
    run_test(&Instruction { mnemonic: Mnemonic::BNDCL, operand1: Some(Direct(BND2)), operand2: Some(Direct(RSI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 26, 214], OperandSize::Qword)
}

#[test]
fn bndcl_4() {
    run_test(&Instruction { mnemonic: Mnemonic::BNDCL, operand1: Some(Direct(BND1)), operand2: Some(Indirect(RAX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 26, 8], OperandSize::Qword)
}

