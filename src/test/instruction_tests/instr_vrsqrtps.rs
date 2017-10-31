use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vrsqrtps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRTPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 82, 215], OperandSize::Dword)
}

#[test]
fn vrsqrtps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRTPS, operand1: Some(Direct(XMM6)), operand2: Some(IndirectDisplaced(EDX, 1079697313, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 82, 178, 161, 223, 90, 64], OperandSize::Dword)
}

#[test]
fn vrsqrtps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRTPS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 82, 201], OperandSize::Qword)
}

#[test]
fn vrsqrtps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRTPS, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexed(RSI, RDI, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 82, 20, 254], OperandSize::Qword)
}

#[test]
fn vrsqrtps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRTPS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 82, 248], OperandSize::Dword)
}

#[test]
fn vrsqrtps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRTPS, operand1: Some(Direct(YMM4)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, ECX, Eight, 650375499, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 82, 164, 206, 75, 241, 195, 38], OperandSize::Dword)
}

#[test]
fn vrsqrtps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRTPS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 82, 193], OperandSize::Qword)
}

#[test]
fn vrsqrtps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRTPS, operand1: Some(Direct(YMM1)), operand2: Some(IndirectScaledDisplaced(RAX, Two, 1566430451, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 82, 12, 69, 243, 212, 93, 93], OperandSize::Qword)
}

