use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fcomp_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FCOMP, operand1: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 14129, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 152, 49, 55], OperandSize::Word)
}

#[test]
fn fcomp_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FCOMP, operand1: Some(IndirectScaledIndexedDisplaced(EDX, ESI, Eight, 2030379410, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 156, 242, 146, 33, 5, 121], OperandSize::Dword)
}

#[test]
fn fcomp_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FCOMP, operand1: Some(IndirectDisplaced(RBX, 652040111, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 155, 175, 87, 221, 38], OperandSize::Qword)
}

#[test]
fn fcomp_4() {
    run_test(&Instruction { mnemonic: Mnemonic::FCOMP, operand1: Some(Direct(ST2)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 218], OperandSize::Word)
}

#[test]
fn fcomp_5() {
    run_test(&Instruction { mnemonic: Mnemonic::FCOMP, operand1: Some(Direct(ST1)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 217], OperandSize::Dword)
}

#[test]
fn fcomp_6() {
    run_test(&Instruction { mnemonic: Mnemonic::FCOMP, operand1: Some(Direct(ST4)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 220], OperandSize::Qword)
}

#[test]
fn fcomp_7() {
    run_test(&Instruction { mnemonic: Mnemonic::FCOMP, operand1: Some(IndirectDisplaced(BP, 8836, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 158, 132, 34], OperandSize::Word)
}

#[test]
fn fcomp_8() {
    run_test(&Instruction { mnemonic: Mnemonic::FCOMP, operand1: Some(Indirect(EAX, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 24], OperandSize::Dword)
}

#[test]
fn fcomp_9() {
    run_test(&Instruction { mnemonic: Mnemonic::FCOMP, operand1: Some(IndirectScaledIndexedDisplaced(RAX, RDX, Eight, 1613748825, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 156, 208, 89, 218, 47, 96], OperandSize::Qword)
}

