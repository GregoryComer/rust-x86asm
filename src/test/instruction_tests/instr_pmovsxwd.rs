use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pmovsxwd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVSXWD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 35, 249], OperandSize::Dword)
}

#[test]
fn pmovsxwd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVSXWD, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledDisplaced(EAX, Eight, 754691413, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 35, 60, 197, 85, 173, 251, 44], OperandSize::Dword)
}

#[test]
fn pmovsxwd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVSXWD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 35, 242], OperandSize::Qword)
}

#[test]
fn pmovsxwd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVSXWD, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexed(RDI, RBX, Two, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 35, 44, 95], OperandSize::Qword)
}

