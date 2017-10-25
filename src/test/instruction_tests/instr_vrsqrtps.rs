use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vrsqrtps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRTPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 82, 249], OperandSize::Dword)
}

#[test]
fn vrsqrtps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRTPS, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexed(EAX, EDI, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 82, 36, 120], OperandSize::Dword)
}

#[test]
fn vrsqrtps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRTPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 82, 234], OperandSize::Qword)
}

#[test]
fn vrsqrtps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRTPS, operand1: Some(Direct(XMM4)), operand2: Some(IndirectDisplaced(RSI, 6270640, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 82, 166, 176, 174, 95, 0], OperandSize::Qword)
}

#[test]
fn vrsqrtps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRTPS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 82, 196], OperandSize::Dword)
}

#[test]
fn vrsqrtps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRTPS, operand1: Some(Direct(YMM0)), operand2: Some(IndirectScaledDisplaced(ESI, Two, 420718899, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 82, 4, 117, 51, 169, 19, 25], OperandSize::Dword)
}

#[test]
fn vrsqrtps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRTPS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 82, 199], OperandSize::Qword)
}

#[test]
fn vrsqrtps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRTPS, operand1: Some(Direct(YMM0)), operand2: Some(IndirectScaledIndexed(RSI, RCX, Two, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 82, 4, 78], OperandSize::Qword)
}

