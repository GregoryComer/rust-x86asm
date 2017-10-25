use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn punpckldq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKLDQ, operand1: Some(Direct(MM0)), operand2: Some(Direct(MM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 98, 192], OperandSize::Dword)
}

#[test]
fn punpckldq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKLDQ, operand1: Some(Direct(MM6)), operand2: Some(IndirectScaledIndexed(EDI, ESI, Two, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 98, 52, 119], OperandSize::Dword)
}

#[test]
fn punpckldq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKLDQ, operand1: Some(Direct(MM3)), operand2: Some(Direct(MM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 98, 219], OperandSize::Qword)
}

#[test]
fn punpckldq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKLDQ, operand1: Some(Direct(MM2)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RAX, Four, 2099109598, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 98, 148, 128, 222, 222, 29, 125], OperandSize::Qword)
}

#[test]
fn punpckldq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKLDQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 98, 230], OperandSize::Dword)
}

#[test]
fn punpckldq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKLDQ, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexed(ECX, ECX, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 98, 28, 201], OperandSize::Dword)
}

#[test]
fn punpckldq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKLDQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 98, 214], OperandSize::Qword)
}

#[test]
fn punpckldq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKLDQ, operand1: Some(Direct(XMM3)), operand2: Some(IndirectDisplaced(RAX, 965686092, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 98, 152, 76, 51, 143, 57], OperandSize::Qword)
}

