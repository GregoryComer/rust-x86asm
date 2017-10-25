use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fidivr_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FIDIVR, operand1: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 21, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[218, 120, 21], OperandSize::Word)
}

#[test]
fn fidivr_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FIDIVR, operand1: Some(IndirectScaledIndexed(ECX, EDI, Two, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[218, 60, 121], OperandSize::Dword)
}

#[test]
fn fidivr_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FIDIVR, operand1: Some(IndirectDisplaced(RAX, 527748839, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[218, 184, 231, 206, 116, 31], OperandSize::Qword)
}

#[test]
fn fidivr_4() {
    run_test(&Instruction { mnemonic: Mnemonic::FIDIVR, operand1: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 191, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[222, 185, 191, 0], OperandSize::Word)
}

#[test]
fn fidivr_5() {
    run_test(&Instruction { mnemonic: Mnemonic::FIDIVR, operand1: Some(IndirectScaledDisplaced(EAX, Eight, 1465572994, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[222, 60, 197, 130, 222, 90, 87], OperandSize::Dword)
}

#[test]
fn fidivr_6() {
    run_test(&Instruction { mnemonic: Mnemonic::FIDIVR, operand1: Some(IndirectScaledIndexed(RAX, RCX, Eight, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[222, 60, 200], OperandSize::Qword)
}

