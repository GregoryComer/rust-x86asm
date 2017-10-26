use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn ficomp_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FICOMP, operand1: Some(Indirect(DI, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[218, 29], OperandSize::Word)
}

#[test]
fn ficomp_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FICOMP, operand1: Some(IndirectScaledDisplaced(EDX, Four, 949355876, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[218, 28, 149, 100, 5, 150, 56], OperandSize::Dword)
}

#[test]
fn ficomp_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FICOMP, operand1: Some(Indirect(RBX, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[218, 27], OperandSize::Qword)
}

#[test]
fn ficomp_4() {
    run_test(&Instruction { mnemonic: Mnemonic::FICOMP, operand1: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 22435, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[222, 154, 163, 87], OperandSize::Word)
}

#[test]
fn ficomp_5() {
    run_test(&Instruction { mnemonic: Mnemonic::FICOMP, operand1: Some(IndirectScaledIndexedDisplaced(EDI, EDX, Two, 829136340, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[222, 156, 87, 212, 157, 107, 49], OperandSize::Dword)
}

#[test]
fn ficomp_6() {
    run_test(&Instruction { mnemonic: Mnemonic::FICOMP, operand1: Some(IndirectScaledIndexed(RCX, RAX, Four, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[222, 28, 129], OperandSize::Qword)
}

