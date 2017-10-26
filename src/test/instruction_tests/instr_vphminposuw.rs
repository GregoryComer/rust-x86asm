use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vphminposuw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHMINPOSUW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 65, 247], OperandSize::Dword)
}

#[test]
fn vphminposuw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHMINPOSUW, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexed(EDX, EDI, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 65, 52, 186], OperandSize::Dword)
}

#[test]
fn vphminposuw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHMINPOSUW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 65, 251], OperandSize::Qword)
}

#[test]
fn vphminposuw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHMINPOSUW, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledDisplaced(RCX, Two, 2139672567, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 65, 20, 77, 247, 207, 136, 127], OperandSize::Qword)
}

