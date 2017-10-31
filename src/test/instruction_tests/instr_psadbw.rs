use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn psadbw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PSADBW, operand1: Some(Direct(MM5)), operand2: Some(Direct(MM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 246, 237], OperandSize::Dword)
}

#[test]
fn psadbw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PSADBW, operand1: Some(Direct(MM7)), operand2: Some(IndirectDisplaced(EDI, 1733640753, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 246, 191, 49, 66, 85, 103], OperandSize::Dword)
}

#[test]
fn psadbw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PSADBW, operand1: Some(Direct(MM3)), operand2: Some(Direct(MM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 246, 219], OperandSize::Qword)
}

#[test]
fn psadbw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PSADBW, operand1: Some(Direct(MM7)), operand2: Some(IndirectScaledDisplaced(RCX, Eight, 1601964271, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 246, 60, 205, 239, 8, 124, 95], OperandSize::Qword)
}

#[test]
fn psadbw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PSADBW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 246, 217], OperandSize::Dword)
}

#[test]
fn psadbw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PSADBW, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledDisplaced(EAX, Four, 2113755961, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 246, 20, 133, 57, 91, 253, 125], OperandSize::Dword)
}

#[test]
fn psadbw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PSADBW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 246, 230], OperandSize::Qword)
}

#[test]
fn psadbw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PSADBW, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledDisplaced(RAX, Four, 275687528, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 246, 12, 133, 104, 168, 110, 16], OperandSize::Qword)
}

