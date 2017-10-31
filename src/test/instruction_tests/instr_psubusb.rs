use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn psubusb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBUSB, operand1: Some(Direct(MM6)), operand2: Some(Direct(MM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 216, 242], OperandSize::Dword)
}

#[test]
fn psubusb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBUSB, operand1: Some(Direct(MM6)), operand2: Some(IndirectScaledDisplaced(EAX, Eight, 528909685, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 216, 52, 197, 117, 133, 134, 31], OperandSize::Dword)
}

#[test]
fn psubusb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBUSB, operand1: Some(Direct(MM6)), operand2: Some(Direct(MM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 216, 247], OperandSize::Qword)
}

#[test]
fn psubusb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBUSB, operand1: Some(Direct(MM5)), operand2: Some(IndirectScaledDisplaced(RCX, Four, 880195099, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 216, 44, 141, 27, 182, 118, 52], OperandSize::Qword)
}

#[test]
fn psubusb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBUSB, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 216, 232], OperandSize::Dword)
}

#[test]
fn psubusb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBUSB, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexed(EBX, EAX, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 216, 36, 131], OperandSize::Dword)
}

#[test]
fn psubusb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBUSB, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 216, 232], OperandSize::Qword)
}

#[test]
fn psubusb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBUSB, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledDisplaced(RCX, Eight, 1547422022, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 216, 44, 205, 70, 201, 59, 92], OperandSize::Qword)
}

