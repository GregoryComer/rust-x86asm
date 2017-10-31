use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn ficomp_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FICOMP, operand1: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 193, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[218, 153, 193, 0], OperandSize::Word)
}

#[test]
fn ficomp_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FICOMP, operand1: Some(IndirectScaledIndexed(EBX, EAX, Eight, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[218, 28, 195], OperandSize::Dword)
}

#[test]
fn ficomp_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FICOMP, operand1: Some(IndirectScaledDisplaced(RAX, Two, 1258260903, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[218, 28, 69, 167, 137, 255, 74], OperandSize::Qword)
}

#[test]
fn ficomp_4() {
    run_test(&Instruction { mnemonic: Mnemonic::FICOMP, operand1: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 12298, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[222, 153, 10, 48], OperandSize::Word)
}

#[test]
fn ficomp_5() {
    run_test(&Instruction { mnemonic: Mnemonic::FICOMP, operand1: Some(Indirect(EBX, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[222, 27], OperandSize::Dword)
}

#[test]
fn ficomp_6() {
    run_test(&Instruction { mnemonic: Mnemonic::FICOMP, operand1: Some(Indirect(RDX, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[222, 26], OperandSize::Qword)
}

