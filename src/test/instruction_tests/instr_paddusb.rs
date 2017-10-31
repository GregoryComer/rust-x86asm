use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn paddusb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDUSB, operand1: Some(Direct(MM5)), operand2: Some(Direct(MM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 220, 235], OperandSize::Dword)
}

#[test]
fn paddusb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDUSB, operand1: Some(Direct(MM7)), operand2: Some(IndirectScaledIndexed(ESI, EDI, Two, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 220, 60, 126], OperandSize::Dword)
}

#[test]
fn paddusb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDUSB, operand1: Some(Direct(MM0)), operand2: Some(Direct(MM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 220, 198], OperandSize::Qword)
}

#[test]
fn paddusb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDUSB, operand1: Some(Direct(MM0)), operand2: Some(IndirectDisplaced(RDI, 1272142602, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 220, 135, 10, 91, 211, 75], OperandSize::Qword)
}

#[test]
fn paddusb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDUSB, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 220, 201], OperandSize::Dword)
}

#[test]
fn paddusb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDUSB, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexed(ESI, EDX, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 220, 60, 214], OperandSize::Dword)
}

#[test]
fn paddusb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDUSB, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 220, 250], OperandSize::Qword)
}

#[test]
fn paddusb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDUSB, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RAX, Two, 127562363, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 220, 140, 70, 123, 114, 154, 7], OperandSize::Qword)
}

