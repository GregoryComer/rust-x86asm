use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pmovsxbw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVSXBW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 32, 208], OperandSize::Dword)
}

#[test]
fn pmovsxbw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVSXBW, operand1: Some(Direct(XMM5)), operand2: Some(Indirect(EAX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 32, 40], OperandSize::Dword)
}

#[test]
fn pmovsxbw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVSXBW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 32, 252], OperandSize::Qword)
}

#[test]
fn pmovsxbw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVSXBW, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexed(RSI, RDI, Two, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 32, 20, 126], OperandSize::Qword)
}

