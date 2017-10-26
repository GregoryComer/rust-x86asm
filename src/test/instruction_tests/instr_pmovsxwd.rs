use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pmovsxwd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVSXWD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 35, 217], OperandSize::Dword)
}

#[test]
fn pmovsxwd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVSXWD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexed(EBX, ESI, Eight, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 35, 36, 243], OperandSize::Dword)
}

#[test]
fn pmovsxwd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVSXWD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 35, 193], OperandSize::Qword)
}

#[test]
fn pmovsxwd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVSXWD, operand1: Some(Direct(XMM6)), operand2: Some(Indirect(RSI, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 35, 54], OperandSize::Qword)
}

