use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vrsqrtps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRTPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 82, 250], OperandSize::Dword)
}

#[test]
fn vrsqrtps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRTPS, operand1: Some(Direct(XMM3)), operand2: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 82, 30], OperandSize::Dword)
}

#[test]
fn vrsqrtps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRTPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 82, 226], OperandSize::Qword)
}

#[test]
fn vrsqrtps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRTPS, operand1: Some(Direct(XMM4)), operand2: Some(Indirect(RCX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 82, 33], OperandSize::Qword)
}

#[test]
fn vrsqrtps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRTPS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 82, 232], OperandSize::Dword)
}

#[test]
fn vrsqrtps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRTPS, operand1: Some(Direct(YMM5)), operand2: Some(IndirectScaledIndexed(ECX, EDX, Eight, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 82, 44, 209], OperandSize::Dword)
}

#[test]
fn vrsqrtps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRTPS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 82, 211], OperandSize::Qword)
}

#[test]
fn vrsqrtps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRTPS, operand1: Some(Direct(YMM0)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RCX, Eight, 1889759143, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 82, 132, 207, 167, 111, 163, 112], OperandSize::Qword)
}

