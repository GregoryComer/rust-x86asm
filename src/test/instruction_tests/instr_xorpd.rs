use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn xorpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::XORPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 87, 240], OperandSize::Dword)
}

#[test]
fn xorpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::XORPD, operand1: Some(Direct(XMM0)), operand2: Some(IndirectDisplaced(ECX, 1386622604, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 87, 129, 140, 46, 166, 82], OperandSize::Dword)
}

#[test]
fn xorpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::XORPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 87, 255], OperandSize::Qword)
}

#[test]
fn xorpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::XORPD, operand1: Some(Direct(XMM7)), operand2: Some(Indirect(RBX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 87, 59], OperandSize::Qword)
}

