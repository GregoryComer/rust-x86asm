use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cvtsi2ss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSI2SS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 42, 223], OperandSize::Dword)
}

#[test]
fn cvtsi2ss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSI2SS, operand1: Some(Direct(XMM7)), operand2: Some(Indirect(EDI, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 42, 63], OperandSize::Dword)
}

#[test]
fn cvtsi2ss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSI2SS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 42, 226], OperandSize::Qword)
}

#[test]
fn cvtsi2ss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSI2SS, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledDisplaced(RSI, Two, 1287331239, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 42, 60, 117, 167, 29, 187, 76], OperandSize::Qword)
}

#[test]
fn cvtsi2ss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSI2SS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(RCX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 72, 15, 42, 233], OperandSize::Qword)
}

#[test]
fn cvtsi2ss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSI2SS, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledDisplaced(RCX, Two, 1224216682, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 72, 15, 42, 52, 77, 106, 16, 248, 72], OperandSize::Qword)
}

