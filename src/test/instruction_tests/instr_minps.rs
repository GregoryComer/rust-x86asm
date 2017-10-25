use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn minps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MINPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 93, 224], OperandSize::Dword)
}

#[test]
fn minps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MINPS, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, ESI, Two, 1511849091, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 93, 164, 112, 131, 252, 28, 90], OperandSize::Dword)
}

#[test]
fn minps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MINPS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 93, 205], OperandSize::Qword)
}

#[test]
fn minps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MINPS, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledDisplaced(RBX, Eight, 1745028051, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 93, 20, 221, 211, 3, 3, 104], OperandSize::Qword)
}

