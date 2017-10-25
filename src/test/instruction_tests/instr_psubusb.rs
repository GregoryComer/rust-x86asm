use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn psubusb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBUSB, operand1: Some(Direct(MM1)), operand2: Some(Direct(MM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 216, 207], OperandSize::Dword)
}

#[test]
fn psubusb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBUSB, operand1: Some(Direct(MM4)), operand2: Some(IndirectDisplaced(ESI, 1410321829, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 216, 166, 165, 205, 15, 84], OperandSize::Dword)
}

#[test]
fn psubusb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBUSB, operand1: Some(Direct(MM1)), operand2: Some(Direct(MM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 216, 201], OperandSize::Qword)
}

#[test]
fn psubusb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBUSB, operand1: Some(Direct(MM3)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RDI, Eight, 1473065566, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 216, 156, 254, 94, 50, 205, 87], OperandSize::Qword)
}

#[test]
fn psubusb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBUSB, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 216, 195], OperandSize::Dword)
}

#[test]
fn psubusb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBUSB, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, ESI, Eight, 289640097, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 216, 132, 246, 161, 142, 67, 17], OperandSize::Dword)
}

#[test]
fn psubusb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBUSB, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 216, 217], OperandSize::Qword)
}

#[test]
fn psubusb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBUSB, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexed(RCX, RBX, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 216, 60, 217], OperandSize::Qword)
}

