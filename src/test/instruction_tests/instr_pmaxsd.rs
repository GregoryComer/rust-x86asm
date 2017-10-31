use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pmaxsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXSD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 61, 202], OperandSize::Dword)
}

#[test]
fn pmaxsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXSD, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexed(EAX, ESI, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 61, 20, 112], OperandSize::Dword)
}

#[test]
fn pmaxsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXSD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 61, 214], OperandSize::Qword)
}

#[test]
fn pmaxsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXSD, operand1: Some(Direct(XMM5)), operand2: Some(IndirectDisplaced(RCX, 136279626, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 61, 169, 74, 118, 31, 8], OperandSize::Qword)
}

