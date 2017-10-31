use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pmovsxbd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVSXBD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 33, 233], OperandSize::Dword)
}

#[test]
fn pmovsxbd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVSXBD, operand1: Some(Direct(XMM5)), operand2: Some(IndirectDisplaced(EDI, 848635867, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 33, 175, 219, 39, 149, 50], OperandSize::Dword)
}

#[test]
fn pmovsxbd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVSXBD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 33, 196], OperandSize::Qword)
}

#[test]
fn pmovsxbd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVSXBD, operand1: Some(Direct(XMM2)), operand2: Some(Indirect(RCX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 33, 17], OperandSize::Qword)
}

