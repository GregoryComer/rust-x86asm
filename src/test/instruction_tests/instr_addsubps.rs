use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn addsubps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::ADDSUBPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 208, 212], OperandSize::Dword)
}

#[test]
fn addsubps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::ADDSUBPS, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexed(EDI, EAX, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 208, 44, 71], OperandSize::Dword)
}

#[test]
fn addsubps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::ADDSUBPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 208, 243], OperandSize::Qword)
}

#[test]
fn addsubps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::ADDSUBPS, operand1: Some(Direct(XMM0)), operand2: Some(IndirectDisplaced(RBX, 1946127272, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 208, 131, 168, 139, 255, 115], OperandSize::Qword)
}

