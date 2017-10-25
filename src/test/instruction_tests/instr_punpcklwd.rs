use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn punpcklwd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKLWD, operand1: Some(Direct(MM1)), operand2: Some(Direct(MM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 97, 206], OperandSize::Dword)
}

#[test]
fn punpcklwd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKLWD, operand1: Some(Direct(MM2)), operand2: Some(IndirectDisplaced(ECX, 662175875, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 97, 145, 131, 0, 120, 39], OperandSize::Dword)
}

#[test]
fn punpcklwd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKLWD, operand1: Some(Direct(MM5)), operand2: Some(Direct(MM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 97, 235], OperandSize::Qword)
}

#[test]
fn punpcklwd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKLWD, operand1: Some(Direct(MM2)), operand2: Some(IndirectScaledDisplaced(RAX, Four, 1845221288, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 97, 20, 133, 168, 215, 251, 109], OperandSize::Qword)
}

#[test]
fn punpcklwd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKLWD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 97, 243], OperandSize::Dword)
}

#[test]
fn punpcklwd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKLWD, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, EDX, Two, 363097584, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 97, 156, 80, 240, 109, 164, 21], OperandSize::Dword)
}

#[test]
fn punpcklwd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKLWD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 97, 228], OperandSize::Qword)
}

#[test]
fn punpcklwd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKLWD, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RDX, Eight, 1124798591, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 97, 140, 214, 127, 16, 11, 67], OperandSize::Qword)
}

