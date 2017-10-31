use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn insertps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::INSERTPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: Some(Literal8(42)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 33, 239, 42], OperandSize::Dword)
}

#[test]
fn insertps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::INSERTPS, operand1: Some(Direct(XMM5)), operand2: Some(IndirectDisplaced(EBX, 255511222, Some(OperandSize::Dword), None)), operand3: Some(Literal8(110)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 33, 171, 182, 202, 58, 15, 110], OperandSize::Dword)
}

#[test]
fn insertps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::INSERTPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(Literal8(29)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 33, 247, 29], OperandSize::Qword)
}

#[test]
fn insertps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::INSERTPS, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexed(RCX, RSI, Four, Some(OperandSize::Dword), None)), operand3: Some(Literal8(109)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 33, 20, 177, 109], OperandSize::Qword)
}

