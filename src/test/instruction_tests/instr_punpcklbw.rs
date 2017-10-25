use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn punpcklbw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKLBW, operand1: Some(Direct(MM6)), operand2: Some(Direct(MM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 96, 240], OperandSize::Dword)
}

#[test]
fn punpcklbw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKLBW, operand1: Some(Direct(MM2)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, ESI, Eight, 2092495563, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 96, 148, 243, 203, 242, 184, 124], OperandSize::Dword)
}

#[test]
fn punpcklbw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKLBW, operand1: Some(Direct(MM3)), operand2: Some(Direct(MM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 96, 217], OperandSize::Qword)
}

#[test]
fn punpcklbw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKLBW, operand1: Some(Direct(MM4)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RSI, Two, 1791506076, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 96, 164, 119, 156, 54, 200, 106], OperandSize::Qword)
}

#[test]
fn punpcklbw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKLBW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 96, 253], OperandSize::Dword)
}

#[test]
fn punpcklbw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKLBW, operand1: Some(Direct(XMM5)), operand2: Some(IndirectDisplaced(EDX, 573018343, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 96, 170, 231, 144, 39, 34], OperandSize::Dword)
}

#[test]
fn punpcklbw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKLBW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 96, 231], OperandSize::Qword)
}

#[test]
fn punpcklbw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKLBW, operand1: Some(Direct(XMM5)), operand2: Some(Indirect(RDX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 96, 42], OperandSize::Qword)
}

