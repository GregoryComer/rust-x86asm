use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn insertps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::INSERTPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: Some(Literal8(86)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 33, 235, 86], OperandSize::Dword)
}

#[test]
fn insertps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::INSERTPS, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledDisplaced(EBX, Four, 1106484413, Some(OperandSize::Dword), None)), operand3: Some(Literal8(46)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 33, 4, 157, 189, 156, 243, 65, 46], OperandSize::Dword)
}

#[test]
fn insertps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::INSERTPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(Literal8(112)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 33, 241, 112], OperandSize::Qword)
}

#[test]
fn insertps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::INSERTPS, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledDisplaced(RAX, Eight, 578730611, Some(OperandSize::Dword), None)), operand3: Some(Literal8(77)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 33, 20, 197, 115, 186, 126, 34, 77], OperandSize::Qword)
}

