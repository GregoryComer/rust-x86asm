use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn bndcl_1() {
    run_test(&Instruction { mnemonic: Mnemonic::BNDCL, operand1: Some(Direct(BND2)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 26, 212], OperandSize::Dword)
}

#[test]
fn bndcl_2() {
    run_test(&Instruction { mnemonic: Mnemonic::BNDCL, operand1: Some(Direct(BND1)), operand2: Some(IndirectScaledIndexed(EDI, ECX, Four, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 26, 12, 143], OperandSize::Dword)
}

#[test]
fn bndcl_3() {
    run_test(&Instruction { mnemonic: Mnemonic::BNDCL, operand1: Some(Direct(BND2)), operand2: Some(Direct(RSP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 26, 212], OperandSize::Qword)
}

#[test]
fn bndcl_4() {
    run_test(&Instruction { mnemonic: Mnemonic::BNDCL, operand1: Some(Direct(BND0)), operand2: Some(Indirect(RAX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 26, 0], OperandSize::Qword)
}

