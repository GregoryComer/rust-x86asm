use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fdivr_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FDIVR, operand1: Some(Indirect(SI, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 60], OperandSize::Word)
}

#[test]
fn fdivr_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FDIVR, operand1: Some(IndirectDisplaced(ESI, 1510179670, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 190, 86, 131, 3, 90], OperandSize::Dword)
}

#[test]
fn fdivr_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FDIVR, operand1: Some(IndirectDisplaced(RDX, 808061230, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 186, 46, 9, 42, 48], OperandSize::Qword)
}

#[test]
fn fdivr_4() {
    run_test(&Instruction { mnemonic: Mnemonic::FDIVR, operand1: Some(Direct(ST)), operand2: Some(Direct(ST1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 249], OperandSize::Word)
}

#[test]
fn fdivr_5() {
    run_test(&Instruction { mnemonic: Mnemonic::FDIVR, operand1: Some(Direct(ST)), operand2: Some(Direct(ST7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 255], OperandSize::Dword)
}

#[test]
fn fdivr_6() {
    run_test(&Instruction { mnemonic: Mnemonic::FDIVR, operand1: Some(Direct(ST)), operand2: Some(Direct(ST6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 254], OperandSize::Qword)
}

#[test]
fn fdivr_7() {
    run_test(&Instruction { mnemonic: Mnemonic::FDIVR, operand1: Some(Memory(24425, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 62, 105, 95], OperandSize::Word)
}

#[test]
fn fdivr_8() {
    run_test(&Instruction { mnemonic: Mnemonic::FDIVR, operand1: Some(IndirectScaledIndexedDisplaced(EBX, EDI, Four, 22355727, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 188, 187, 15, 31, 85, 1], OperandSize::Dword)
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
    run_test(&Instruction { mnemonic: Mnemonic::FDIVR, operand1: Some(Direct(ST4)), operand2: Some(Direct(ST)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 244], OperandSize::Qword)
}

