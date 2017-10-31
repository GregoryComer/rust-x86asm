use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn rorx_1() {
    run_test(&Instruction { mnemonic: Mnemonic::RORX, operand1: Some(Direct(ECX)), operand2: Some(Direct(EBP)), operand3: Some(Literal8(25)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 123, 240, 205, 25], OperandSize::Dword)
}

#[test]
fn rorx_2() {
    run_test(&Instruction { mnemonic: Mnemonic::RORX, operand1: Some(Direct(EDI)), operand2: Some(IndirectScaledIndexed(ESI, EDI, Eight, Some(OperandSize::Dword), None)), operand3: Some(Literal8(94)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 123, 240, 60, 254, 94], OperandSize::Dword)
}

#[test]
fn rorx_3() {
    run_test(&Instruction { mnemonic: Mnemonic::RORX, operand1: Some(Direct(EBP)), operand2: Some(Direct(EDX)), operand3: Some(Literal8(94)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 123, 240, 234, 94], OperandSize::Qword)
}

#[test]
fn rorx_4() {
    run_test(&Instruction { mnemonic: Mnemonic::RORX, operand1: Some(Direct(EDI)), operand2: Some(IndirectDisplaced(RSI, 1138610628, Some(OperandSize::Dword), None)), operand3: Some(Literal8(109)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 123, 240, 190, 196, 209, 221, 67, 109], OperandSize::Qword)
}

#[test]
fn rorx_5() {
    run_test(&Instruction { mnemonic: Mnemonic::RORX, operand1: Some(Direct(RCX)), operand2: Some(Direct(RDI)), operand3: Some(Literal8(96)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 251, 240, 207, 96], OperandSize::Qword)
}

#[test]
fn rorx_6() {
    run_test(&Instruction { mnemonic: Mnemonic::RORX, operand1: Some(Direct(RDI)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RBX, Four, 744668984, Some(OperandSize::Qword), None)), operand3: Some(Literal8(66)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 251, 240, 188, 159, 56, 191, 98, 44, 66], OperandSize::Qword)
}

