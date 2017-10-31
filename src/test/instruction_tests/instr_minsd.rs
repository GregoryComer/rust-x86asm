use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn minsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MINSD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 93, 223], OperandSize::Dword)
}

#[test]
fn minsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MINSD, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, EAX, Eight, 1713221474, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 93, 148, 195, 98, 175, 29, 102], OperandSize::Dword)
}

#[test]
fn minsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MINSD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 93, 240], OperandSize::Qword)
}

#[test]
fn minsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MINSD, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RCX, Two, 1188550023, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 93, 148, 74, 135, 213, 215, 70], OperandSize::Qword)
}

