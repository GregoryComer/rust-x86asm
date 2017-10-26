use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn extractps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::EXTRACTPS, operand1: Some(Direct(ESP)), operand2: Some(Direct(XMM5)), operand3: Some(Literal8(114)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 23, 236, 114], OperandSize::Dword)
}

#[test]
fn extractps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::EXTRACTPS, operand1: Some(IndirectScaledIndexedDisplaced(EDX, EDI, Two, 2017330259, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM1)), operand3: Some(Literal8(118)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 23, 140, 122, 83, 4, 62, 120, 118], OperandSize::Dword)
}

#[test]
fn extractps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::EXTRACTPS, operand1: Some(Direct(ESI)), operand2: Some(Direct(XMM5)), operand3: Some(Literal8(56)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 23, 238, 56], OperandSize::Qword)
}

#[test]
fn extractps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::EXTRACTPS, operand1: Some(IndirectScaledIndexed(RCX, RAX, Four, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM3)), operand3: Some(Literal8(17)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 23, 28, 129, 17], OperandSize::Qword)
}

