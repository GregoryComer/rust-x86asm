use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn phminposuw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PHMINPOSUW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 65, 211], OperandSize::Dword)
}

#[test]
fn phminposuw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PHMINPOSUW, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexed(EDI, ESI, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 65, 36, 119], OperandSize::Dword)
}

#[test]
fn phminposuw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PHMINPOSUW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 65, 227], OperandSize::Qword)
}

#[test]
fn phminposuw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PHMINPOSUW, operand1: Some(Direct(XMM5)), operand2: Some(Indirect(RCX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 65, 41], OperandSize::Qword)
}

