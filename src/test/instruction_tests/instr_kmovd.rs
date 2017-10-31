use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn kmovd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVD, operand1: Some(Direct(K1)), operand2: Some(Direct(K7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 249, 144, 207], OperandSize::Dword)
}

#[test]
fn kmovd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVD, operand1: Some(Direct(K2)), operand2: Some(IndirectDisplaced(ESI, 1989447340, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 249, 144, 150, 172, 142, 148, 118], OperandSize::Dword)
}

#[test]
fn kmovd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVD, operand1: Some(Direct(K4)), operand2: Some(Direct(K5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 249, 144, 229], OperandSize::Qword)
}

#[test]
fn kmovd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVD, operand1: Some(Direct(K3)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RSI, Four, 1921096163, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 249, 144, 156, 182, 227, 153, 129, 114], OperandSize::Qword)
}

#[test]
fn kmovd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVD, operand1: Some(IndirectScaledDisplaced(EBX, Eight, 1459056174, Some(OperandSize::Dword), None)), operand2: Some(Direct(K7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 249, 145, 60, 221, 46, 110, 247, 86], OperandSize::Dword)
}

#[test]
fn kmovd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVD, operand1: Some(IndirectScaledDisplaced(RSI, Four, 992323964, Some(OperandSize::Dword), None)), operand2: Some(Direct(K2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 249, 145, 20, 181, 124, 169, 37, 59], OperandSize::Qword)
}

#[test]
fn kmovd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVD, operand1: Some(Direct(K5)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 146, 238], OperandSize::Dword)
}

#[test]
fn kmovd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVD, operand1: Some(Direct(K6)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 146, 241], OperandSize::Qword)
}

#[test]
fn kmovd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVD, operand1: Some(Direct(EBX)), operand2: Some(Direct(K6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 147, 222], OperandSize::Dword)
}

#[test]
fn kmovd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVD, operand1: Some(Direct(EDI)), operand2: Some(Direct(K7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 147, 255], OperandSize::Qword)
}

