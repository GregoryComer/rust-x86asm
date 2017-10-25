use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pavgb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PAVGB, operand1: Some(Direct(MM6)), operand2: Some(Direct(MM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 224, 242], OperandSize::Dword)
}

#[test]
fn pavgb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PAVGB, operand1: Some(Direct(MM5)), operand2: Some(IndirectScaledIndexed(ESI, ECX, Four, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 224, 44, 142], OperandSize::Dword)
}

#[test]
fn pavgb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PAVGB, operand1: Some(Direct(MM3)), operand2: Some(Direct(MM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 224, 219], OperandSize::Qword)
}

#[test]
fn pavgb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PAVGB, operand1: Some(Direct(MM3)), operand2: Some(IndirectDisplaced(RSI, 196520895, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 224, 158, 191, 171, 182, 11], OperandSize::Qword)
}

#[test]
fn pavgb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PAVGB, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 224, 221], OperandSize::Dword)
}

#[test]
fn pavgb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PAVGB, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledDisplaced(ECX, Four, 907613674, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 224, 28, 141, 234, 21, 25, 54], OperandSize::Dword)
}

#[test]
fn pavgb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PAVGB, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 224, 226], OperandSize::Qword)
}

#[test]
fn pavgb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PAVGB, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexed(RCX, RDX, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 224, 60, 81], OperandSize::Qword)
}

