use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn bndcl_1() {
    run_test(&Instruction { mnemonic: Mnemonic::BNDCL, operand1: Some(Direct(BND0)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 26, 193], OperandSize::Dword)
}

#[test]
fn bndcl_2() {
    run_test(&Instruction { mnemonic: Mnemonic::BNDCL, operand1: Some(Direct(BND3)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, EDX, Four, 94145718, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 26, 156, 146, 182, 140, 156, 5], OperandSize::Dword)
}

#[test]
fn bndcl_3() {
    run_test(&Instruction { mnemonic: Mnemonic::BNDCL, operand1: Some(Direct(BND0)), operand2: Some(Direct(RBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 26, 195], OperandSize::Qword)
}

#[test]
fn bndcl_4() {
    run_test(&Instruction { mnemonic: Mnemonic::BNDCL, operand1: Some(Direct(BND0)), operand2: Some(IndirectDisplaced(RBX, 682572982, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 26, 131, 182, 60, 175, 40], OperandSize::Qword)
}

