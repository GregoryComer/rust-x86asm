use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pmaxsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXSD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 61, 221], OperandSize::Dword)
}

#[test]
fn pmaxsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXSD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectDisplaced(EDI, 83365549, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 61, 167, 173, 14, 248, 4], OperandSize::Dword)
}

#[test]
fn pmaxsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXSD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 61, 253], OperandSize::Qword)
}

#[test]
fn pmaxsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXSD, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RAX, Two, 1998955844, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 61, 132, 67, 68, 165, 37, 119], OperandSize::Qword)
}

