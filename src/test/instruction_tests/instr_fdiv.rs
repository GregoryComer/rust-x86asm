use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fdiv_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FDIV, operand1: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 12977, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 177, 177, 50], OperandSize::Word)
}

#[test]
fn fdiv_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FDIV, operand1: Some(Indirect(EDX, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 50], OperandSize::Dword)
}

#[test]
fn fdiv_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FDIV, operand1: Some(IndirectScaledDisplaced(RAX, Eight, 1246024820, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 52, 197, 116, 212, 68, 74], OperandSize::Qword)
}

#[test]
fn fdiv_4() {
    run_test(&Instruction { mnemonic: Mnemonic::FDIV, operand1: Some(Direct(ST)), operand2: Some(Direct(ST4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 244], OperandSize::Word)
}

#[test]
fn fdiv_5() {
    run_test(&Instruction { mnemonic: Mnemonic::FDIV, operand1: Some(Direct(ST)), operand2: Some(Direct(ST2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 242], OperandSize::Dword)
}

#[test]
fn fdiv_6() {
    run_test(&Instruction { mnemonic: Mnemonic::FDIV, operand1: Some(Direct(ST)), operand2: Some(Direct(ST1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 241], OperandSize::Qword)
}

#[test]
fn fdiv_7() {
    run_test(&Instruction { mnemonic: Mnemonic::FDIV, operand1: Some(IndirectDisplaced(SI, 7404, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 180, 236, 28], OperandSize::Word)
}

#[test]
fn fdiv_8() {
    run_test(&Instruction { mnemonic: Mnemonic::FDIV, operand1: Some(Indirect(ECX, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 49], OperandSize::Dword)
}

#[test]
fn fdiv_9() {
    run_test(&Instruction { mnemonic: Mnemonic::FDIV, operand1: Some(IndirectScaledIndexedDisplaced(RBX, RSI, Four, 1224922924, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 180, 179, 44, 215, 2, 73], OperandSize::Qword)
}

#[test]
fn fdiv_10() {
    run_test(&Instruction { mnemonic: Mnemonic::FDIV, operand1: Some(Direct(ST5)), operand2: Some(Direct(ST)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 253], OperandSize::Word)
}

#[test]
fn fdiv_11() {
    run_test(&Instruction { mnemonic: Mnemonic::FDIV, operand1: Some(Direct(ST5)), operand2: Some(Direct(ST)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 253], OperandSize::Dword)
}

#[test]
fn fdiv_12() {
    run_test(&Instruction { mnemonic: Mnemonic::FDIV, operand1: Some(Direct(ST1)), operand2: Some(Direct(ST)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 249], OperandSize::Qword)
}

