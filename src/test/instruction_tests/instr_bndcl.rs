use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn bndcl_1() {
    run_test(&Instruction { mnemonic: Mnemonic::BNDCL, operand1: Some(Direct(BND2)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 26, 214], OperandSize::Dword)
}

#[test]
fn bndcl_2() {
    run_test(&Instruction { mnemonic: Mnemonic::BNDCL, operand1: Some(Direct(BND3)), operand2: Some(IndirectScaledDisplaced(ESI, Eight, 1096634474, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 26, 28, 245, 106, 80, 93, 65], OperandSize::Dword)
}

#[test]
fn bndcl_3() {
    run_test(&Instruction { mnemonic: Mnemonic::BNDCL, operand1: Some(Direct(BND1)), operand2: Some(Direct(RBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 26, 203], OperandSize::Qword)
}

#[test]
fn bndcl_4() {
    run_test(&Instruction { mnemonic: Mnemonic::BNDCL, operand1: Some(Direct(BND2)), operand2: Some(Indirect(RDI, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 26, 23], OperandSize::Qword)
}

