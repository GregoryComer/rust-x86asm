use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vlddqu_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VLDDQU, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexed(EDX, EBX, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 240, 12, 90], OperandSize::Dword)
}

#[test]
fn vlddqu_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VLDDQU, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledDisplaced(RDX, Two, 267189977, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 240, 44, 85, 217, 254, 236, 15], OperandSize::Qword)
}

#[test]
fn vlddqu_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VLDDQU, operand1: Some(Direct(YMM5)), operand2: Some(IndirectScaledIndexed(EBX, EDI, Eight, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 255, 240, 44, 251], OperandSize::Dword)
}

#[test]
fn vlddqu_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VLDDQU, operand1: Some(Direct(YMM4)), operand2: Some(IndirectDisplaced(RDX, 961091475, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 255, 240, 162, 147, 23, 73, 57], OperandSize::Qword)
}

