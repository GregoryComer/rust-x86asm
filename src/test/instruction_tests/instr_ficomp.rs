use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn ficomp_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FICOMP, operand1: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 9399, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[218, 153, 183, 36], OperandSize::Word)
}

#[test]
fn ficomp_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FICOMP, operand1: Some(IndirectScaledIndexed(EAX, EDX, Eight, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[218, 28, 208], OperandSize::Dword)
}

#[test]
fn ficomp_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FICOMP, operand1: Some(Indirect(RCX, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[218, 25], OperandSize::Qword)
}

#[test]
fn ficomp_4() {
    run_test(&Instruction { mnemonic: Mnemonic::FICOMP, operand1: Some(IndirectDisplaced(SI, 9150, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[222, 156, 190, 35], OperandSize::Word)
}

#[test]
fn ficomp_5() {
    run_test(&Instruction { mnemonic: Mnemonic::FICOMP, operand1: Some(IndirectScaledIndexed(EAX, EAX, Eight, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[222, 28, 192], OperandSize::Dword)
}

#[test]
fn ficomp_6() {
    run_test(&Instruction { mnemonic: Mnemonic::FICOMP, operand1: Some(Indirect(RCX, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[222, 25], OperandSize::Qword)
}

