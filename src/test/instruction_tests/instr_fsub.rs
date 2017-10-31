use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fsub_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FSUB, operand1: Some(IndirectDisplaced(BX, 31746, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 167, 2, 124], OperandSize::Word)
}

#[test]
fn fsub_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FSUB, operand1: Some(IndirectDisplaced(EDX, 1156231356, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 162, 188, 176, 234, 68], OperandSize::Dword)
}

#[test]
fn fsub_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FSUB, operand1: Some(IndirectDisplaced(RSI, 1562531298, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 166, 226, 85, 34, 93], OperandSize::Qword)
}

#[test]
fn fsub_4() {
    run_test(&Instruction { mnemonic: Mnemonic::FSUB, operand1: Some(Direct(ST)), operand2: Some(Direct(ST6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 230], OperandSize::Word)
}

#[test]
fn fsub_5() {
    run_test(&Instruction { mnemonic: Mnemonic::FSUB, operand1: Some(Direct(ST)), operand2: Some(Direct(ST5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 229], OperandSize::Dword)
}

#[test]
fn fsub_6() {
    run_test(&Instruction { mnemonic: Mnemonic::FSUB, operand1: Some(Direct(ST)), operand2: Some(Direct(ST3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 227], OperandSize::Qword)
}

#[test]
fn fsub_7() {
    run_test(&Instruction { mnemonic: Mnemonic::FSUB, operand1: Some(IndirectDisplaced(DI, 30741, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 165, 21, 120], OperandSize::Word)
}

#[test]
fn fsub_8() {
    run_test(&Instruction { mnemonic: Mnemonic::FSUB, operand1: Some(IndirectScaledDisplaced(EAX, Two, 252631721, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 36, 69, 169, 218, 14, 15], OperandSize::Dword)
}

#[test]
fn fsub_9() {
    run_test(&Instruction { mnemonic: Mnemonic::FSUB, operand1: Some(IndirectScaledIndexed(RAX, RAX, Eight, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 36, 192], OperandSize::Qword)
}

#[test]
fn fsub_10() {
    run_test(&Instruction { mnemonic: Mnemonic::FSUB, operand1: Some(Direct(ST4)), operand2: Some(Direct(ST)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 236], OperandSize::Word)
}

#[test]
fn fsub_11() {
    run_test(&Instruction { mnemonic: Mnemonic::FSUB, operand1: Some(Direct(ST4)), operand2: Some(Direct(ST)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 236], OperandSize::Dword)
}

#[test]
fn fsub_12() {
    run_test(&Instruction { mnemonic: Mnemonic::FSUB, operand1: Some(Direct(ST5)), operand2: Some(Direct(ST)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 237], OperandSize::Qword)
}

