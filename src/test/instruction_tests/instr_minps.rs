use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn minps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MINPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 93, 230], OperandSize::Dword)
}

#[test]
fn minps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MINPS, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, EAX, Two, 1360256958, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 93, 140, 67, 190, 223, 19, 81], OperandSize::Dword)
}

#[test]
fn minps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MINPS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 93, 202], OperandSize::Qword)
}

#[test]
fn minps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MINPS, operand1: Some(Direct(XMM1)), operand2: Some(Indirect(RDI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 93, 15], OperandSize::Qword)
}

