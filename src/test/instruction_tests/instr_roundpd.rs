use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn roundpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::ROUNDPD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: Some(Literal8(6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 9, 239, 6], OperandSize::Dword)
}

#[test]
fn roundpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::ROUNDPD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledDisplaced(EAX, Four, 820911131, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 9, 36, 133, 27, 28, 238, 48, 7], OperandSize::Dword)
}

#[test]
fn roundpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::ROUNDPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: Some(Literal8(32)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 9, 246, 32], OperandSize::Qword)
}

#[test]
fn roundpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::ROUNDPD, operand1: Some(Direct(XMM1)), operand2: Some(IndirectDisplaced(RAX, 1450404050, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(63)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 9, 136, 210, 104, 115, 86, 63], OperandSize::Qword)
}

