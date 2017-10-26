use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fiadd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FIADD, operand1: Some(IndirectScaledIndexed(BP, DI, One, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[218, 3], OperandSize::Word)
}

#[test]
fn fiadd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FIADD, operand1: Some(IndirectDisplaced(EAX, 1990766240, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[218, 128, 160, 174, 168, 118], OperandSize::Dword)
}

#[test]
fn fiadd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FIADD, operand1: Some(IndirectDisplaced(RDX, 301946131, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[218, 130, 19, 85, 255, 17], OperandSize::Qword)
}

#[test]
fn fiadd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::FIADD, operand1: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 19816, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[222, 130, 104, 77], OperandSize::Word)
}

#[test]
fn fiadd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::FIADD, operand1: Some(IndirectScaledDisplaced(EAX, Two, 62501287, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[222, 4, 69, 167, 177, 185, 3], OperandSize::Dword)
}

#[test]
fn fiadd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::FIADD, operand1: Some(Indirect(RCX, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[222, 1], OperandSize::Qword)
}

