use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn bndcu_1() {
    run_test(&Instruction { mnemonic: Mnemonic::BNDCU, operand1: Some(Direct(BND2)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 26, 212], OperandSize::Dword)
}

#[test]
fn bndcu_2() {
    run_test(&Instruction { mnemonic: Mnemonic::BNDCU, operand1: Some(Direct(BND2)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, ECX, Four, 297539107, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 26, 148, 137, 35, 22, 188, 17], OperandSize::Dword)
}

#[test]
fn bndcu_3() {
    run_test(&Instruction { mnemonic: Mnemonic::BNDCU, operand1: Some(Direct(BND2)), operand2: Some(Direct(RDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 26, 210], OperandSize::Qword)
}

#[test]
fn bndcu_4() {
    run_test(&Instruction { mnemonic: Mnemonic::BNDCU, operand1: Some(Direct(BND2)), operand2: Some(IndirectScaledDisplaced(RBX, Four, 1720790179, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 26, 20, 157, 163, 44, 145, 102], OperandSize::Qword)
}

