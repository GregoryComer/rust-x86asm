use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fdivr_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FDIVR, operand1: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 138, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 184, 138, 0], OperandSize::Word)
}

#[test]
fn fdivr_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FDIVR, operand1: Some(IndirectScaledIndexedDisplaced(EAX, EAX, Four, 859770985, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 188, 128, 105, 16, 63, 51], OperandSize::Dword)
}

#[test]
fn fdivr_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FDIVR, operand1: Some(Indirect(RCX, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 57], OperandSize::Qword)
}

#[test]
fn fdivr_4() {
    run_test(&Instruction { mnemonic: Mnemonic::FDIVR, operand1: Some(Direct(ST)), operand2: Some(Direct(ST5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 253], OperandSize::Word)
}

#[test]
fn fdivr_5() {
    run_test(&Instruction { mnemonic: Mnemonic::FDIVR, operand1: Some(Direct(ST)), operand2: Some(Direct(ST7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 255], OperandSize::Dword)
}

#[test]
fn fdivr_6() {
    run_test(&Instruction { mnemonic: Mnemonic::FDIVR, operand1: Some(Direct(ST)), operand2: Some(Direct(ST3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 251], OperandSize::Qword)
}

#[test]
fn fdivr_7() {
    run_test(&Instruction { mnemonic: Mnemonic::FDIVR, operand1: Some(IndirectScaledIndexed(BP, SI, One, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 58], OperandSize::Word)
}

#[test]
fn fdivr_8() {
    run_test(&Instruction { mnemonic: Mnemonic::FDIVR, operand1: Some(IndirectDisplaced(EBX, 899977303, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 187, 87, 144, 164, 53], OperandSize::Dword)
}

#[test]
fn fdivr_9() {
    run_test(&Instruction { mnemonic: Mnemonic::FDIVR, operand1: Some(Indirect(RSI, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 62], OperandSize::Qword)
}

#[test]
fn fdivr_10() {
    run_test(&Instruction { mnemonic: Mnemonic::FDIVR, operand1: Some(Direct(ST6)), operand2: Some(Direct(ST)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 246], OperandSize::Word)
}

#[test]
fn fdivr_11() {
    run_test(&Instruction { mnemonic: Mnemonic::FDIVR, operand1: Some(Direct(ST1)), operand2: Some(Direct(ST)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 241], OperandSize::Dword)
}

#[test]
fn fdivr_12() {
    run_test(&Instruction { mnemonic: Mnemonic::FDIVR, operand1: Some(Direct(ST6)), operand2: Some(Direct(ST)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 246], OperandSize::Qword)
}

