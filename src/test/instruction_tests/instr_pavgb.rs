use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pavgb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PAVGB, operand1: Some(Direct(MM3)), operand2: Some(Direct(MM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 224, 221], OperandSize::Dword)
}

#[test]
fn pavgb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PAVGB, operand1: Some(Direct(MM2)), operand2: Some(IndirectDisplaced(EBX, 652639767, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 224, 147, 23, 126, 230, 38], OperandSize::Dword)
}

#[test]
fn pavgb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PAVGB, operand1: Some(Direct(MM5)), operand2: Some(Direct(MM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 224, 234], OperandSize::Qword)
}

#[test]
fn pavgb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PAVGB, operand1: Some(Direct(MM1)), operand2: Some(IndirectScaledIndexed(RCX, RCX, Two, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 224, 12, 73], OperandSize::Qword)
}

#[test]
fn pavgb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PAVGB, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 224, 226], OperandSize::Dword)
}

#[test]
fn pavgb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PAVGB, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexed(ECX, EDX, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 224, 60, 209], OperandSize::Dword)
}

#[test]
fn pavgb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PAVGB, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 224, 239], OperandSize::Qword)
}

#[test]
fn pavgb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PAVGB, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RDI, Four, 1930791705, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 224, 172, 186, 25, 139, 21, 115], OperandSize::Qword)
}

