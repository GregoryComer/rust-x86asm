use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn subps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SUBPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 92, 211], OperandSize::Dword)
}

#[test]
fn subps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SUBPS, operand1: Some(Direct(XMM1)), operand2: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 92, 14], OperandSize::Dword)
}

#[test]
fn subps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SUBPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 92, 193], OperandSize::Qword)
}

#[test]
fn subps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SUBPS, operand1: Some(Direct(XMM4)), operand2: Some(IndirectDisplaced(RBX, 436028427, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 92, 163, 11, 68, 253, 25], OperandSize::Qword)
}

