use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn andpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::ANDPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 84, 248], OperandSize::Dword)
}

#[test]
fn andpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::ANDPD, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, EDI, Eight, 1163971173, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 84, 172, 249, 101, 202, 96, 69], OperandSize::Dword)
}

#[test]
fn andpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::ANDPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 84, 223], OperandSize::Qword)
}

#[test]
fn andpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::ANDPD, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledDisplaced(RCX, Four, 1947716163, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 84, 44, 141, 67, 202, 23, 116], OperandSize::Qword)
}

