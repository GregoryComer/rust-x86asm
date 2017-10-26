use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fcomp_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FCOMP, operand1: Some(Indirect(BX, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 31], OperandSize::Word)
}

#[test]
fn fcomp_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FCOMP, operand1: Some(Indirect(EDX, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 26], OperandSize::Dword)
}

#[test]
fn fcomp_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FCOMP, operand1: Some(IndirectScaledIndexed(RCX, RDX, Four, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 28, 145], OperandSize::Qword)
}

#[test]
fn fcomp_4() {
    run_test(&Instruction { mnemonic: Mnemonic::FCOMP, operand1: Some(Direct(ST1)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 217], OperandSize::Word)
}

#[test]
fn fcomp_5() {
    run_test(&Instruction { mnemonic: Mnemonic::FCOMP, operand1: Some(Direct(ST3)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 219], OperandSize::Dword)
}

#[test]
fn fcomp_6() {
    run_test(&Instruction { mnemonic: Mnemonic::FCOMP, operand1: Some(Direct(ST6)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 222], OperandSize::Qword)
}

#[test]
fn fcomp_7() {
    run_test(&Instruction { mnemonic: Mnemonic::FCOMP, operand1: Some(IndirectDisplaced(BP, 226, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 158, 226, 0], OperandSize::Word)
}

#[test]
fn fcomp_8() {
    run_test(&Instruction { mnemonic: Mnemonic::FCOMP, operand1: Some(IndirectDisplaced(EDI, 739613160, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 159, 232, 153, 21, 44], OperandSize::Dword)
}

#[test]
fn fcomp_9() {
    run_test(&Instruction { mnemonic: Mnemonic::FCOMP, operand1: Some(IndirectScaledIndexedDisplaced(RAX, RDX, Two, 2098257020, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 156, 80, 124, 220, 16, 125], OperandSize::Qword)
}

