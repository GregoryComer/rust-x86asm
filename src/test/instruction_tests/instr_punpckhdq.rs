use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn punpckhdq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKHDQ, operand1: Some(Direct(MM6)), operand2: Some(Direct(MM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 106, 247], OperandSize::Dword)
}

#[test]
fn punpckhdq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKHDQ, operand1: Some(Direct(MM4)), operand2: Some(IndirectScaledDisplaced(ECX, Two, 1051211935, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 106, 36, 77, 159, 56, 168, 62], OperandSize::Dword)
}

#[test]
fn punpckhdq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKHDQ, operand1: Some(Direct(MM6)), operand2: Some(Direct(MM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 106, 241], OperandSize::Qword)
}

#[test]
fn punpckhdq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKHDQ, operand1: Some(Direct(MM2)), operand2: Some(IndirectScaledIndexed(RAX, RSI, Two, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 106, 20, 112], OperandSize::Qword)
}

#[test]
fn punpckhdq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKHDQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 106, 225], OperandSize::Dword)
}

#[test]
fn punpckhdq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKHDQ, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, EDI, Two, 457160619, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 106, 188, 121, 171, 183, 63, 27], OperandSize::Dword)
}

#[test]
fn punpckhdq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKHDQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 106, 235], OperandSize::Qword)
}

#[test]
fn punpckhdq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKHDQ, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledDisplaced(RDI, Four, 140231887, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 106, 20, 189, 207, 196, 91, 8], OperandSize::Qword)
}

