use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fdivr_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FDIVR, operand1: Some(IndirectDisplaced(BP, 6251, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 190, 107, 24], OperandSize::Word)
}

#[test]
fn fdivr_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FDIVR, operand1: Some(IndirectScaledIndexedDisplaced(EAX, ECX, Two, 488256611, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 188, 72, 99, 52, 26, 29], OperandSize::Dword)
}

#[test]
fn fdivr_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FDIVR, operand1: Some(IndirectDisplaced(RCX, 162726031, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 185, 143, 0, 179, 9], OperandSize::Qword)
}

#[test]
fn fdivr_4() {
    run_test(&Instruction { mnemonic: Mnemonic::FDIVR, operand1: Some(Direct(ST)), operand2: Some(Direct(ST1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 249], OperandSize::Word)
}

#[test]
fn fdivr_5() {
    run_test(&Instruction { mnemonic: Mnemonic::FDIVR, operand1: Some(Direct(ST)), operand2: Some(Direct(ST1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 249], OperandSize::Dword)
}

#[test]
fn fdivr_6() {
    run_test(&Instruction { mnemonic: Mnemonic::FDIVR, operand1: Some(Direct(ST)), operand2: Some(Direct(ST4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 252], OperandSize::Qword)
}

#[test]
fn fdivr_7() {
    run_test(&Instruction { mnemonic: Mnemonic::FDIVR, operand1: Some(IndirectScaledIndexed(BP, SI, One, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 58], OperandSize::Word)
}

#[test]
fn fdivr_8() {
    run_test(&Instruction { mnemonic: Mnemonic::FDIVR, operand1: Some(IndirectDisplaced(EBX, 351462343, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 187, 199, 227, 242, 20], OperandSize::Dword)
}

#[test]
fn fdivr_9() {
    run_test(&Instruction { mnemonic: Mnemonic::FDIVR, operand1: Some(IndirectScaledIndexed(RDX, RSI, Eight, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 60, 242], OperandSize::Qword)
}

#[test]
fn fdivr_10() {
    run_test(&Instruction { mnemonic: Mnemonic::FDIVR, operand1: Some(Direct(ST4)), operand2: Some(Direct(ST)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 244], OperandSize::Word)
}

#[test]
fn fdivr_11() {
    run_test(&Instruction { mnemonic: Mnemonic::FDIVR, operand1: Some(Direct(ST5)), operand2: Some(Direct(ST)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 245], OperandSize::Dword)
}

#[test]
fn fdivr_12() {
    run_test(&Instruction { mnemonic: Mnemonic::FDIVR, operand1: Some(Direct(ST2)), operand2: Some(Direct(ST)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 242], OperandSize::Qword)
}

