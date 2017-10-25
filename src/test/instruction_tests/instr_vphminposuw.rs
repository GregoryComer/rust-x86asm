use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vphminposuw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHMINPOSUW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 65, 198], OperandSize::Dword)
}

#[test]
fn vphminposuw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHMINPOSUW, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexed(EBX, ESI, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 65, 12, 243], OperandSize::Dword)
}

#[test]
fn vphminposuw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHMINPOSUW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 65, 210], OperandSize::Qword)
}

#[test]
fn vphminposuw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHMINPOSUW, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexed(RAX, RBX, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 65, 20, 216], OperandSize::Qword)
}

