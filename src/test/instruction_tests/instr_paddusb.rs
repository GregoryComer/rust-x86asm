use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn paddusb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDUSB, operand1: Some(Direct(MM0)), operand2: Some(Direct(MM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 220, 192], OperandSize::Dword)
}

#[test]
fn paddusb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDUSB, operand1: Some(Direct(MM4)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, EDX, Eight, 1773946480, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 220, 164, 208, 112, 70, 188, 105], OperandSize::Dword)
}

#[test]
fn paddusb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDUSB, operand1: Some(Direct(MM5)), operand2: Some(Direct(MM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 220, 233], OperandSize::Qword)
}

#[test]
fn paddusb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDUSB, operand1: Some(Direct(MM0)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RCX, Four, 63420230, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 220, 132, 138, 70, 183, 199, 3], OperandSize::Qword)
}

#[test]
fn paddusb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDUSB, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 220, 196], OperandSize::Dword)
}

#[test]
fn paddusb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDUSB, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledDisplaced(ESI, Two, 62685890, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 220, 52, 117, 194, 130, 188, 3], OperandSize::Dword)
}

#[test]
fn paddusb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDUSB, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 220, 221], OperandSize::Qword)
}

#[test]
fn paddusb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDUSB, operand1: Some(Direct(XMM7)), operand2: Some(Indirect(RCX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 220, 57], OperandSize::Qword)
}

