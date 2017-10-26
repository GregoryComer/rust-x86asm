use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pminsb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINSB, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 56, 222], OperandSize::Dword)
}

#[test]
fn pminsb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINSB, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledDisplaced(ECX, Two, 224839127, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 56, 60, 77, 215, 197, 102, 13], OperandSize::Dword)
}

#[test]
fn pminsb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINSB, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 56, 249], OperandSize::Qword)
}

#[test]
fn pminsb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINSB, operand1: Some(Direct(XMM3)), operand2: Some(Indirect(RCX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 56, 25], OperandSize::Qword)
}

