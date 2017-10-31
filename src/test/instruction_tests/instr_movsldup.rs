use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn movsldup_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSLDUP, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 18, 248], OperandSize::Dword)
}

#[test]
fn movsldup_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSLDUP, operand1: Some(Direct(XMM1)), operand2: Some(IndirectDisplaced(ESI, 1143096763, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 18, 142, 187, 69, 34, 68], OperandSize::Dword)
}

#[test]
fn movsldup_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSLDUP, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 18, 236], OperandSize::Qword)
}

#[test]
fn movsldup_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSLDUP, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RDX, Four, 1358190849, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 18, 188, 145, 1, 89, 244, 80], OperandSize::Qword)
}

