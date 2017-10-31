use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fadd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FADD, operand1: Some(IndirectDisplaced(SI, 131, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 132, 131, 0], OperandSize::Word)
}

#[test]
fn fadd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FADD, operand1: Some(IndirectScaledIndexed(EDX, ESI, Four, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 4, 178], OperandSize::Dword)
}

#[test]
fn fadd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FADD, operand1: Some(IndirectScaledIndexed(RCX, RDI, Two, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 4, 121], OperandSize::Qword)
}

#[test]
fn fadd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::FADD, operand1: Some(Direct(ST)), operand2: Some(Direct(ST6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 198], OperandSize::Word)
}

#[test]
fn fadd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::FADD, operand1: Some(Direct(ST)), operand2: Some(Direct(ST6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 198], OperandSize::Dword)
}

#[test]
fn fadd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::FADD, operand1: Some(Direct(ST)), operand2: Some(Direct(ST3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 195], OperandSize::Qword)
}

#[test]
fn fadd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::FADD, operand1: Some(IndirectDisplaced(BP, 108, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 70, 108], OperandSize::Word)
}

#[test]
fn fadd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::FADD, operand1: Some(IndirectDisplaced(EAX, 341708861, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 128, 61, 16, 94, 20], OperandSize::Dword)
}

#[test]
fn fadd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::FADD, operand1: Some(Indirect(RDI, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 7], OperandSize::Qword)
}

#[test]
fn fadd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::FADD, operand1: Some(Direct(ST7)), operand2: Some(Direct(ST)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 199], OperandSize::Word)
}

#[test]
fn fadd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::FADD, operand1: Some(Direct(ST5)), operand2: Some(Direct(ST)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 197], OperandSize::Dword)
}

#[test]
fn fadd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::FADD, operand1: Some(Direct(ST7)), operand2: Some(Direct(ST)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 199], OperandSize::Qword)
}

