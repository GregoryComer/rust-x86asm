use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fdiv_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FDIV, operand1: Some(IndirectDisplaced(BP, 26831, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 182, 207, 104], OperandSize::Word)
}

#[test]
fn fdiv_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FDIV, operand1: Some(Indirect(EDI, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 55], OperandSize::Dword)
}

#[test]
fn fdiv_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FDIV, operand1: Some(IndirectScaledDisplaced(RDX, Eight, 1423917849, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 52, 213, 25, 67, 223, 84], OperandSize::Qword)
}

#[test]
fn fdiv_4() {
    run_test(&Instruction { mnemonic: Mnemonic::FDIV, operand1: Some(Direct(ST)), operand2: Some(Direct(ST3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 243], OperandSize::Word)
}

#[test]
fn fdiv_5() {
    run_test(&Instruction { mnemonic: Mnemonic::FDIV, operand1: Some(Direct(ST)), operand2: Some(Direct(ST6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 246], OperandSize::Dword)
}

#[test]
fn fdiv_6() {
    run_test(&Instruction { mnemonic: Mnemonic::FDIV, operand1: Some(Direct(ST)), operand2: Some(Direct(ST7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 247], OperandSize::Qword)
}

#[test]
fn fdiv_7() {
    run_test(&Instruction { mnemonic: Mnemonic::FDIV, operand1: Some(Indirect(DI, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 53], OperandSize::Word)
}

#[test]
fn fdiv_8() {
    run_test(&Instruction { mnemonic: Mnemonic::FDIV, operand1: Some(IndirectScaledIndexedDisplaced(ESI, EBX, Four, 256643826, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 180, 158, 242, 18, 76, 15], OperandSize::Dword)
}

#[test]
fn fdiv_9() {
    run_test(&Instruction { mnemonic: Mnemonic::FDIV, operand1: Some(IndirectDisplaced(RCX, 183135052, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 177, 76, 107, 234, 10], OperandSize::Qword)
}

#[test]
fn fdiv_10() {
    run_test(&Instruction { mnemonic: Mnemonic::FDIV, operand1: Some(Direct(ST4)), operand2: Some(Direct(ST)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 252], OperandSize::Word)
}

#[test]
fn fdiv_11() {
    run_test(&Instruction { mnemonic: Mnemonic::FDIV, operand1: Some(Direct(ST1)), operand2: Some(Direct(ST)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 249], OperandSize::Dword)
}

#[test]
fn fdiv_12() {
    run_test(&Instruction { mnemonic: Mnemonic::FDIV, operand1: Some(Direct(ST3)), operand2: Some(Direct(ST)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 251], OperandSize::Qword)
}

