use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn bndcn_1() {
    run_test(&Instruction { mnemonic: Mnemonic::BNDCN, operand1: Some(Direct(BND0)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 27, 199], OperandSize::Dword)
}

#[test]
fn bndcn_2() {
    run_test(&Instruction { mnemonic: Mnemonic::BNDCN, operand1: Some(Direct(BND1)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, ECX, Four, 1725203696, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 27, 140, 138, 240, 132, 212, 102], OperandSize::Dword)
}

#[test]
fn bndcn_3() {
    run_test(&Instruction { mnemonic: Mnemonic::BNDCN, operand1: Some(Direct(BND2)), operand2: Some(Direct(RDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 27, 210], OperandSize::Qword)
}

#[test]
fn bndcn_4() {
    run_test(&Instruction { mnemonic: Mnemonic::BNDCN, operand1: Some(Direct(BND3)), operand2: Some(IndirectScaledDisplaced(RCX, Eight, 704642595, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 27, 28, 205, 35, 254, 255, 41], OperandSize::Qword)
}

