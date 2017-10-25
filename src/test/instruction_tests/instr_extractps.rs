use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn extractps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::EXTRACTPS, operand1: Some(Direct(EDX)), operand2: Some(Direct(XMM0)), operand3: Some(Literal8(47)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 23, 194, 47], OperandSize::Dword)
}

#[test]
fn extractps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::EXTRACTPS, operand1: Some(IndirectScaledIndexed(EDI, EBX, Two, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM1)), operand3: Some(Literal8(107)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 23, 12, 95, 107], OperandSize::Dword)
}

#[test]
fn extractps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::EXTRACTPS, operand1: Some(Direct(ESP)), operand2: Some(Direct(XMM6)), operand3: Some(Literal8(12)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 23, 244, 12], OperandSize::Qword)
}

#[test]
fn extractps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::EXTRACTPS, operand1: Some(IndirectScaledIndexed(RBX, RAX, Two, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM0)), operand3: Some(Literal8(34)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 23, 4, 67, 34], OperandSize::Qword)
}

