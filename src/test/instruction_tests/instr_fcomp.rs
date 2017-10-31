use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fcomp_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FCOMP, operand1: Some(IndirectScaledIndexed(BP, DI, One, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 27], OperandSize::Word)
}

#[test]
fn fcomp_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FCOMP, operand1: Some(IndirectScaledDisplaced(ESI, Eight, 2075422527, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 28, 245, 63, 111, 180, 123], OperandSize::Dword)
}

#[test]
fn fcomp_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FCOMP, operand1: Some(IndirectScaledDisplaced(RCX, Eight, 1217030794, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 28, 205, 138, 106, 138, 72], OperandSize::Qword)
}

#[test]
fn fcomp_4() {
    run_test(&Instruction { mnemonic: Mnemonic::FCOMP, operand1: Some(Direct(ST5)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 221], OperandSize::Word)
}

#[test]
fn fcomp_5() {
    run_test(&Instruction { mnemonic: Mnemonic::FCOMP, operand1: Some(Direct(ST5)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 221], OperandSize::Dword)
}

#[test]
fn fcomp_6() {
    run_test(&Instruction { mnemonic: Mnemonic::FCOMP, operand1: Some(Direct(ST5)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 221], OperandSize::Qword)
}

#[test]
fn fcomp_7() {
    run_test(&Instruction { mnemonic: Mnemonic::FCOMP, operand1: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 970, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 152, 202, 3], OperandSize::Word)
}

#[test]
fn fcomp_8() {
    run_test(&Instruction { mnemonic: Mnemonic::FCOMP, operand1: Some(IndirectScaledIndexed(ESI, EBX, Two, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 28, 94], OperandSize::Dword)
}

#[test]
fn fcomp_9() {
    run_test(&Instruction { mnemonic: Mnemonic::FCOMP, operand1: Some(IndirectScaledIndexed(RCX, RDX, Four, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 28, 145], OperandSize::Qword)
}

