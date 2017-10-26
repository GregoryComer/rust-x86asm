use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn movupd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVUPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 16, 250], OperandSize::Dword)
}

#[test]
fn movupd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVUPD, operand1: Some(Direct(XMM7)), operand2: Some(IndirectDisplaced(EAX, 1801474551, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 16, 184, 247, 81, 96, 107], OperandSize::Dword)
}

#[test]
fn movupd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVUPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 16, 210], OperandSize::Qword)
}

#[test]
fn movupd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVUPD, operand1: Some(Direct(XMM3)), operand2: Some(Indirect(RSI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 16, 30], OperandSize::Qword)
}

#[test]
fn movupd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVUPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 16, 252], OperandSize::Dword)
}

#[test]
fn movupd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVUPD, operand1: Some(Indirect(EBX, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 17, 11], OperandSize::Dword)
}

#[test]
fn movupd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVUPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 16, 208], OperandSize::Qword)
}

#[test]
fn movupd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVUPD, operand1: Some(IndirectScaledIndexed(RCX, RDX, Two, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 17, 12, 81], OperandSize::Qword)
}

