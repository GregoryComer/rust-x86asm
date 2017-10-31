use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pavgw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PAVGW, operand1: Some(Direct(MM6)), operand2: Some(Direct(MM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 227, 243], OperandSize::Dword)
}

#[test]
fn pavgw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PAVGW, operand1: Some(Direct(MM5)), operand2: Some(IndirectDisplaced(ECX, 1488848554, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 227, 169, 170, 6, 190, 88], OperandSize::Dword)
}

#[test]
fn pavgw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PAVGW, operand1: Some(Direct(MM0)), operand2: Some(Direct(MM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 227, 197], OperandSize::Qword)
}

#[test]
fn pavgw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PAVGW, operand1: Some(Direct(MM3)), operand2: Some(IndirectScaledDisplaced(RSI, Eight, 1243734869, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 227, 28, 245, 85, 227, 33, 74], OperandSize::Qword)
}

#[test]
fn pavgw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PAVGW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 227, 248], OperandSize::Dword)
}

#[test]
fn pavgw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PAVGW, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexed(EDI, EAX, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 227, 36, 135], OperandSize::Dword)
}

#[test]
fn pavgw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PAVGW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 227, 224], OperandSize::Qword)
}

#[test]
fn pavgw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PAVGW, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RDX, Four, 679256299, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 227, 156, 150, 235, 160, 124, 40], OperandSize::Qword)
}

