use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn psubusb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBUSB, operand1: Some(Direct(MM1)), operand2: Some(Direct(MM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 216, 204], OperandSize::Dword)
}

#[test]
fn psubusb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBUSB, operand1: Some(Direct(MM4)), operand2: Some(IndirectScaledIndexed(EDI, EDI, Two, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 216, 36, 127], OperandSize::Dword)
}

#[test]
fn psubusb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBUSB, operand1: Some(Direct(MM6)), operand2: Some(Direct(MM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 216, 247], OperandSize::Qword)
}

#[test]
fn psubusb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBUSB, operand1: Some(Direct(MM2)), operand2: Some(IndirectScaledDisplaced(RCX, Four, 1385501575, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 216, 20, 141, 135, 19, 149, 82], OperandSize::Qword)
}

#[test]
fn psubusb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBUSB, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 216, 243], OperandSize::Dword)
}

#[test]
fn psubusb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBUSB, operand1: Some(Direct(XMM0)), operand2: Some(IndirectDisplaced(EAX, 1725469588, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 216, 128, 148, 147, 216, 102], OperandSize::Dword)
}

#[test]
fn psubusb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBUSB, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 216, 208], OperandSize::Qword)
}

#[test]
fn psubusb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBUSB, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledDisplaced(RDX, Two, 1884521789, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 216, 60, 85, 61, 133, 83, 112], OperandSize::Qword)
}

