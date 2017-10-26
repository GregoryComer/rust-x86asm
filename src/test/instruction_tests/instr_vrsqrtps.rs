use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vrsqrtps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRTPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 82, 244], OperandSize::Dword)
}

#[test]
fn vrsqrtps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRTPS, operand1: Some(Direct(XMM0)), operand2: Some(IndirectDisplaced(EAX, 723915112, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 82, 128, 104, 17, 38, 43], OperandSize::Dword)
}

#[test]
fn vrsqrtps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRTPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 82, 217], OperandSize::Qword)
}

#[test]
fn vrsqrtps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRTPS, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexed(RAX, RDX, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 82, 4, 80], OperandSize::Qword)
}

#[test]
fn vrsqrtps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRTPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 82, 207], OperandSize::Dword)
}

#[test]
fn vrsqrtps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRTPS, operand1: Some(Direct(YMM2)), operand2: Some(IndirectScaledDisplaced(ESI, Two, 1599262931, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 82, 20, 117, 211, 208, 82, 95], OperandSize::Dword)
}

#[test]
fn vrsqrtps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRTPS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 82, 208], OperandSize::Qword)
}

#[test]
fn vrsqrtps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRTPS, operand1: Some(Direct(YMM5)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RDX, Two, 1815970463, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 82, 172, 86, 159, 130, 61, 108], OperandSize::Qword)
}

