use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fdiv_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FDIV, operand1: Some(IndirectDisplaced(SI, 20973, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 180, 237, 81], OperandSize::Word)
}

#[test]
fn fdiv_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FDIV, operand1: Some(IndirectScaledIndexedDisplaced(ESI, EDX, Eight, 1302978448, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 180, 214, 144, 223, 169, 77], OperandSize::Dword)
}

#[test]
fn fdiv_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FDIV, operand1: Some(IndirectDisplaced(RDX, 1972618984, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 178, 232, 198, 147, 117], OperandSize::Qword)
}

#[test]
fn fdiv_4() {
    run_test(&Instruction { mnemonic: Mnemonic::FDIV, operand1: Some(Direct(ST)), operand2: Some(Direct(ST4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 244], OperandSize::Word)
}

#[test]
fn fdiv_5() {
    run_test(&Instruction { mnemonic: Mnemonic::FDIV, operand1: Some(Direct(ST)), operand2: Some(Direct(ST5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 245], OperandSize::Dword)
}

#[test]
fn fdiv_6() {
    run_test(&Instruction { mnemonic: Mnemonic::FDIV, operand1: Some(Direct(ST)), operand2: Some(Direct(ST7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 247], OperandSize::Qword)
}

#[test]
fn fdiv_7() {
    run_test(&Instruction { mnemonic: Mnemonic::FDIV, operand1: Some(IndirectScaledIndexed(BP, SI, One, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 50], OperandSize::Word)
}

#[test]
fn fdiv_8() {
    run_test(&Instruction { mnemonic: Mnemonic::FDIV, operand1: Some(Indirect(EDX, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 50], OperandSize::Dword)
}

#[test]
fn fdiv_9() {
    run_test(&Instruction { mnemonic: Mnemonic::FDIV, operand1: Some(IndirectScaledIndexedDisplaced(RBX, RDI, Eight, 2106791462, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 180, 251, 38, 22, 147, 125], OperandSize::Qword)
}

#[test]
fn fdiv_10() {
    run_test(&Instruction { mnemonic: Mnemonic::FDIV, operand1: Some(Direct(ST2)), operand2: Some(Direct(ST)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 250], OperandSize::Word)
}

#[test]
fn fdiv_11() {
    run_test(&Instruction { mnemonic: Mnemonic::FDIV, operand1: Some(Direct(ST5)), operand2: Some(Direct(ST)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 253], OperandSize::Dword)
}

#[test]
fn fdiv_12() {
    run_test(&Instruction { mnemonic: Mnemonic::FDIV, operand1: Some(Direct(ST1)), operand2: Some(Direct(ST)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 249], OperandSize::Qword)
}

