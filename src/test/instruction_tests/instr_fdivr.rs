use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fdivr_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FDIVR, operand1: Some(Memory(15722, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 62, 106, 61], OperandSize::Word)
}

#[test]
fn fdivr_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FDIVR, operand1: Some(IndirectScaledDisplaced(EAX, Eight, 1476943995, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 60, 197, 123, 96, 8, 88], OperandSize::Dword)
}

#[test]
fn fdivr_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FDIVR, operand1: Some(Indirect(RAX, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 56], OperandSize::Qword)
}

#[test]
fn fdivr_4() {
    run_test(&Instruction { mnemonic: Mnemonic::FDIVR, operand1: Some(Direct(ST)), operand2: Some(Direct(ST6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 254], OperandSize::Word)
}

#[test]
fn fdivr_5() {
    run_test(&Instruction { mnemonic: Mnemonic::FDIVR, operand1: Some(Direct(ST)), operand2: Some(Direct(ST6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 254], OperandSize::Dword)
}

#[test]
fn fdivr_6() {
    run_test(&Instruction { mnemonic: Mnemonic::FDIVR, operand1: Some(Direct(ST)), operand2: Some(Direct(ST4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 252], OperandSize::Qword)
}

#[test]
fn fdivr_7() {
    run_test(&Instruction { mnemonic: Mnemonic::FDIVR, operand1: Some(Memory(18251, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 62, 75, 71], OperandSize::Word)
}

#[test]
fn fdivr_8() {
    run_test(&Instruction { mnemonic: Mnemonic::FDIVR, operand1: Some(IndirectScaledDisplaced(ESI, Four, 1932343531, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 60, 181, 235, 56, 45, 115], OperandSize::Dword)
}

#[test]
fn fdivr_9() {
    run_test(&Instruction { mnemonic: Mnemonic::FDIVR, operand1: Some(IndirectScaledIndexedDisplaced(RBX, RBX, Eight, 588191930, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 188, 219, 186, 24, 15, 35], OperandSize::Qword)
}

#[test]
fn fdivr_10() {
    run_test(&Instruction { mnemonic: Mnemonic::FDIVR, operand1: Some(Direct(ST6)), operand2: Some(Direct(ST)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 246], OperandSize::Word)
}

#[test]
fn fdivr_11() {
    run_test(&Instruction { mnemonic: Mnemonic::FDIVR, operand1: Some(Direct(ST2)), operand2: Some(Direct(ST)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 242], OperandSize::Dword)
}

#[test]
fn fdivr_12() {
    run_test(&Instruction { mnemonic: Mnemonic::FDIVR, operand1: Some(Direct(ST4)), operand2: Some(Direct(ST)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 244], OperandSize::Qword)
}

