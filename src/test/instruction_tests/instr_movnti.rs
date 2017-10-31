use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn movnti_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVNTI, operand1: Some(IndirectDisplaced(BP, 5766, Some(OperandSize::Dword), None)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 195, 166, 134, 22], OperandSize::Word)
}

#[test]
fn movnti_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVNTI, operand1: Some(IndirectDisplaced(ESI, 1228451844, Some(OperandSize::Dword), None)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 195, 158, 4, 176, 56, 73], OperandSize::Dword)
}

#[test]
fn movnti_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVNTI, operand1: Some(IndirectScaledIndexed(RDX, RBX, Two, Some(OperandSize::Dword), None)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 195, 20, 90], OperandSize::Qword)
}

#[test]
fn movnti_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVNTI, operand1: Some(IndirectScaledIndexedDisplaced(RBX, RAX, Two, 350190394, Some(OperandSize::Qword), None)), operand2: Some(Direct(RCX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 195, 140, 67, 58, 123, 223, 20], OperandSize::Qword)
}

