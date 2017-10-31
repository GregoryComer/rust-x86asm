use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn phsubd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PHSUBD, operand1: Some(Direct(MM4)), operand2: Some(Direct(MM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 6, 228], OperandSize::Dword)
}

#[test]
fn phsubd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PHSUBD, operand1: Some(Direct(MM0)), operand2: Some(IndirectScaledIndexed(EAX, EDX, Eight, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 6, 4, 208], OperandSize::Dword)
}

#[test]
fn phsubd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PHSUBD, operand1: Some(Direct(MM7)), operand2: Some(Direct(MM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 6, 249], OperandSize::Qword)
}

#[test]
fn phsubd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PHSUBD, operand1: Some(Direct(MM7)), operand2: Some(IndirectDisplaced(RDI, 1747866885, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 6, 191, 5, 85, 46, 104], OperandSize::Qword)
}

#[test]
fn phsubd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PHSUBD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 6, 239], OperandSize::Dword)
}

#[test]
fn phsubd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PHSUBD, operand1: Some(Direct(XMM2)), operand2: Some(Indirect(EAX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 6, 16], OperandSize::Dword)
}

#[test]
fn phsubd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PHSUBD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 6, 193], OperandSize::Qword)
}

#[test]
fn phsubd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PHSUBD, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexed(RBX, RBX, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 6, 52, 91], OperandSize::Qword)
}

