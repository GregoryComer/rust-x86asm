use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pmovsxbd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVSXBD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 33, 222], OperandSize::Dword)
}

#[test]
fn pmovsxbd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVSXBD, operand1: Some(Direct(XMM1)), operand2: Some(Indirect(EDI, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 33, 15], OperandSize::Dword)
}

#[test]
fn pmovsxbd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVSXBD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 33, 232], OperandSize::Qword)
}

#[test]
fn pmovsxbd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVSXBD, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledDisplaced(RSI, Eight, 989909567, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 33, 20, 245, 63, 210, 0, 59], OperandSize::Qword)
}

