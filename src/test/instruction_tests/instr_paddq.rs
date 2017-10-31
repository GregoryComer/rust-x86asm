use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn paddq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDQ, operand1: Some(Direct(MM1)), operand2: Some(Direct(MM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 212, 202], OperandSize::Dword)
}

#[test]
fn paddq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDQ, operand1: Some(Direct(MM4)), operand2: Some(Indirect(EAX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 212, 32], OperandSize::Dword)
}

#[test]
fn paddq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDQ, operand1: Some(Direct(MM2)), operand2: Some(Direct(MM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 212, 213], OperandSize::Qword)
}

#[test]
fn paddq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDQ, operand1: Some(Direct(MM0)), operand2: Some(Indirect(RDI, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 212, 7], OperandSize::Qword)
}

#[test]
fn paddq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 212, 199], OperandSize::Dword)
}

#[test]
fn paddq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDQ, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexed(EDX, EDX, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 212, 52, 82], OperandSize::Dword)
}

#[test]
fn paddq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 212, 217], OperandSize::Qword)
}

#[test]
fn paddq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDQ, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledDisplaced(RAX, Four, 867993361, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 212, 4, 133, 17, 135, 188, 51], OperandSize::Qword)
}

