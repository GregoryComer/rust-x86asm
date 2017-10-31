use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn psubd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBD, operand1: Some(Direct(MM5)), operand2: Some(Direct(MM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 250, 232], OperandSize::Dword)
}

#[test]
fn psubd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBD, operand1: Some(Direct(MM4)), operand2: Some(IndirectDisplaced(ESI, 640605602, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 250, 166, 162, 221, 46, 38], OperandSize::Dword)
}

#[test]
fn psubd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBD, operand1: Some(Direct(MM5)), operand2: Some(Direct(MM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 250, 237], OperandSize::Qword)
}

#[test]
fn psubd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBD, operand1: Some(Direct(MM4)), operand2: Some(Indirect(RAX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 250, 32], OperandSize::Qword)
}

#[test]
fn psubd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 250, 217], OperandSize::Dword)
}

#[test]
fn psubd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBD, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, ESI, Eight, 1862460985, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 250, 140, 243, 57, 230, 2, 111], OperandSize::Dword)
}

#[test]
fn psubd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 250, 236], OperandSize::Qword)
}

#[test]
fn psubd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBD, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexed(RDX, RCX, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 250, 4, 202], OperandSize::Qword)
}

