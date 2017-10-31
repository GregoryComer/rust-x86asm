use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn psubd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBD, operand1: Some(Direct(MM5)), operand2: Some(Direct(MM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 250, 236], OperandSize::Dword)
}

#[test]
fn psubd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBD, operand1: Some(Direct(MM3)), operand2: Some(IndirectScaledDisplaced(ESI, Eight, 491773854, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 250, 28, 245, 158, 223, 79, 29], OperandSize::Dword)
}

#[test]
fn psubd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBD, operand1: Some(Direct(MM7)), operand2: Some(Direct(MM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 250, 251], OperandSize::Qword)
}

#[test]
fn psubd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBD, operand1: Some(Direct(MM5)), operand2: Some(IndirectScaledIndexed(RDI, RAX, Eight, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 250, 44, 199], OperandSize::Qword)
}

#[test]
fn psubd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 250, 232], OperandSize::Dword)
}

#[test]
fn psubd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBD, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, EDI, Four, 733347562, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 250, 156, 184, 234, 254, 181, 43], OperandSize::Dword)
}

#[test]
fn psubd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 250, 251], OperandSize::Qword)
}

#[test]
fn psubd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledDisplaced(RDX, Two, 1804004182, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 250, 36, 85, 86, 235, 134, 107], OperandSize::Qword)
}

