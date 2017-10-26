use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn movlpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVLPD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexed(EBX, EAX, Two, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 18, 36, 67], OperandSize::Dword)
}

#[test]
fn movlpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVLPD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledDisplaced(RAX, Two, 2033973011, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 18, 36, 69, 19, 247, 59, 121], OperandSize::Qword)
}

#[test]
fn movlpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVLPD, operand1: Some(Indirect(EBX, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 19, 59], OperandSize::Dword)
}

#[test]
fn movlpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVLPD, operand1: Some(IndirectScaledIndexed(RSI, RAX, Two, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 19, 4, 70], OperandSize::Qword)
}

