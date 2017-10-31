use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn paddusb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDUSB, operand1: Some(Direct(MM4)), operand2: Some(Direct(MM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 220, 231], OperandSize::Dword)
}

#[test]
fn paddusb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDUSB, operand1: Some(Direct(MM0)), operand2: Some(IndirectScaledDisplaced(ECX, Two, 745477214, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 220, 4, 77, 94, 20, 111, 44], OperandSize::Dword)
}

#[test]
fn paddusb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDUSB, operand1: Some(Direct(MM0)), operand2: Some(Direct(MM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 220, 199], OperandSize::Qword)
}

#[test]
fn paddusb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDUSB, operand1: Some(Direct(MM4)), operand2: Some(IndirectScaledDisplaced(RDX, Two, 263471784, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 220, 36, 85, 168, 66, 180, 15], OperandSize::Qword)
}

#[test]
fn paddusb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDUSB, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 220, 249], OperandSize::Dword)
}

#[test]
fn paddusb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDUSB, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledDisplaced(EDI, Four, 1364502576, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 220, 44, 189, 48, 168, 84, 81], OperandSize::Dword)
}

#[test]
fn paddusb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDUSB, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 220, 222], OperandSize::Qword)
}

#[test]
fn paddusb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDUSB, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RDI, Two, 1147681, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 220, 188, 123, 33, 131, 17, 0], OperandSize::Qword)
}

