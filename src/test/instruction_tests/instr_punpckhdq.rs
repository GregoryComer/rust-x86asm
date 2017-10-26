use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn punpckhdq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKHDQ, operand1: Some(Direct(MM0)), operand2: Some(Direct(MM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 106, 197], OperandSize::Dword)
}

#[test]
fn punpckhdq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKHDQ, operand1: Some(Direct(MM5)), operand2: Some(IndirectScaledDisplaced(ESI, Two, 568415494, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 106, 44, 117, 6, 85, 225, 33], OperandSize::Dword)
}

#[test]
fn punpckhdq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKHDQ, operand1: Some(Direct(MM5)), operand2: Some(Direct(MM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 106, 235], OperandSize::Qword)
}

#[test]
fn punpckhdq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKHDQ, operand1: Some(Direct(MM6)), operand2: Some(Indirect(RSI, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 106, 54], OperandSize::Qword)
}

#[test]
fn punpckhdq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKHDQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 106, 205], OperandSize::Dword)
}

#[test]
fn punpckhdq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKHDQ, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexed(EDX, ESI, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 106, 20, 114], OperandSize::Dword)
}

#[test]
fn punpckhdq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKHDQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 106, 192], OperandSize::Qword)
}

#[test]
fn punpckhdq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKHDQ, operand1: Some(Direct(XMM1)), operand2: Some(Indirect(RDX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 106, 10], OperandSize::Qword)
}

