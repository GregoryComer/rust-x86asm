use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vphminposuw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHMINPOSUW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 65, 215], OperandSize::Dword)
}

#[test]
fn vphminposuw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHMINPOSUW, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, EDI, Eight, 847675034, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 65, 180, 254, 154, 126, 134, 50], OperandSize::Dword)
}

#[test]
fn vphminposuw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHMINPOSUW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 65, 210], OperandSize::Qword)
}

#[test]
fn vphminposuw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHMINPOSUW, operand1: Some(Direct(XMM0)), operand2: Some(Indirect(RBX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 65, 3], OperandSize::Qword)
}

