use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vaddsubps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSUBPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 203, 208, 248], OperandSize::Dword)
}

#[test]
fn vaddsubps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSUBPS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexed(EAX, EBX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 208, 12, 152], OperandSize::Dword)
}

#[test]
fn vaddsubps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSUBPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 195, 208, 227], OperandSize::Qword)
}

#[test]
fn vaddsubps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSUBPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexed(RCX, RCX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 243, 208, 44, 201], OperandSize::Qword)
}

#[test]
fn vaddsubps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSUBPS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 215, 208, 238], OperandSize::Dword)
}

#[test]
fn vaddsubps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSUBPS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM2)), operand3: Some(Indirect(EDI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 239, 208, 7], OperandSize::Dword)
}

#[test]
fn vaddsubps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSUBPS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 199, 208, 250], OperandSize::Qword)
}

#[test]
fn vaddsubps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSUBPS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RBX, Two, 1346802343, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 223, 208, 164, 91, 167, 146, 70, 80], OperandSize::Qword)
}

