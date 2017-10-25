use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fistp_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FISTP, operand1: Some(IndirectDisplaced(SI, 28022, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[219, 156, 118, 109], OperandSize::Word)
}

#[test]
fn fistp_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FISTP, operand1: Some(Indirect(EDX, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[219, 26], OperandSize::Dword)
}

#[test]
fn fistp_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FISTP, operand1: Some(IndirectDisplaced(RSI, 698397993, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[219, 158, 41, 181, 160, 41], OperandSize::Qword)
}

#[test]
fn fistp_4() {
    run_test(&Instruction { mnemonic: Mnemonic::FISTP, operand1: Some(IndirectDisplaced(BP, 7046, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[223, 158, 134, 27], OperandSize::Word)
}

#[test]
fn fistp_5() {
    run_test(&Instruction { mnemonic: Mnemonic::FISTP, operand1: Some(Indirect(ECX, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[223, 25], OperandSize::Dword)
}

#[test]
fn fistp_6() {
    run_test(&Instruction { mnemonic: Mnemonic::FISTP, operand1: Some(IndirectScaledIndexedDisplaced(RDI, RSI, Two, 636390160, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[223, 156, 119, 16, 139, 238, 37], OperandSize::Qword)
}

#[test]
fn fistp_7() {
    run_test(&Instruction { mnemonic: Mnemonic::FISTP, operand1: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 9043, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[223, 184, 83, 35], OperandSize::Word)
}

#[test]
fn fistp_8() {
    run_test(&Instruction { mnemonic: Mnemonic::FISTP, operand1: Some(Indirect(EAX, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[223, 56], OperandSize::Dword)
}

#[test]
fn fistp_9() {
    run_test(&Instruction { mnemonic: Mnemonic::FISTP, operand1: Some(IndirectDisplaced(RDI, 1349466839, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[223, 191, 215, 58, 111, 80], OperandSize::Qword)
}

