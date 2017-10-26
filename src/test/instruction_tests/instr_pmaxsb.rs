use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pmaxsb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXSB, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 60, 241], OperandSize::Dword)
}

#[test]
fn pmaxsb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXSB, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, ESI, Four, 1231074322, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 60, 140, 177, 18, 180, 96, 73], OperandSize::Dword)
}

#[test]
fn pmaxsb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXSB, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 60, 192], OperandSize::Qword)
}

#[test]
fn pmaxsb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXSB, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexed(RDX, RBX, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 60, 44, 154], OperandSize::Qword)
}

