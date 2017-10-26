use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn psubusb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBUSB, operand1: Some(Direct(MM5)), operand2: Some(Direct(MM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 216, 235], OperandSize::Dword)
}

#[test]
fn psubusb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBUSB, operand1: Some(Direct(MM0)), operand2: Some(IndirectDisplaced(ESI, 948616762, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 216, 134, 58, 190, 138, 56], OperandSize::Dword)
}

#[test]
fn psubusb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBUSB, operand1: Some(Direct(MM2)), operand2: Some(Direct(MM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 216, 213], OperandSize::Qword)
}

#[test]
fn psubusb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBUSB, operand1: Some(Direct(MM6)), operand2: Some(IndirectDisplaced(RBX, 981829552, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 216, 179, 176, 135, 133, 58], OperandSize::Qword)
}

#[test]
fn psubusb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBUSB, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 216, 243], OperandSize::Dword)
}

#[test]
fn psubusb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBUSB, operand1: Some(Direct(XMM0)), operand2: Some(Indirect(EDX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 216, 2], OperandSize::Dword)
}

#[test]
fn psubusb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBUSB, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 216, 229], OperandSize::Qword)
}

#[test]
fn psubusb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBUSB, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexed(RBX, RBX, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 216, 44, 219], OperandSize::Qword)
}

