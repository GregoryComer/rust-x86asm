use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vphminposuw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHMINPOSUW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 65, 235], OperandSize::Dword)
}

#[test]
fn vphminposuw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHMINPOSUW, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledDisplaced(EAX, Four, 149425090, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 65, 4, 133, 194, 11, 232, 8], OperandSize::Dword)
}

#[test]
fn vphminposuw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHMINPOSUW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 65, 228], OperandSize::Qword)
}

#[test]
fn vphminposuw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHMINPOSUW, operand1: Some(Direct(XMM5)), operand2: Some(Indirect(RDX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 65, 42], OperandSize::Qword)
}

