use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vaddsubps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSUBPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 208, 224], OperandSize::Dword)
}

#[test]
fn vaddsubps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSUBPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EBX, Two, 1435894350, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 195, 208, 164, 94, 78, 2, 150, 85], OperandSize::Dword)
}

#[test]
fn vaddsubps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSUBPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 235, 208, 228], OperandSize::Qword)
}

#[test]
fn vaddsubps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSUBPS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(RBX, 1812256638, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 243, 208, 139, 126, 215, 4, 108], OperandSize::Qword)
}

#[test]
fn vaddsubps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSUBPS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 239, 208, 239], OperandSize::Dword)
}

#[test]
fn vaddsubps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSUBPS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledDisplaced(EAX, Two, 928226500, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 231, 208, 44, 69, 196, 156, 83, 55], OperandSize::Dword)
}

#[test]
fn vaddsubps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSUBPS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 199, 208, 243], OperandSize::Qword)
}

#[test]
fn vaddsubps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSUBPS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectDisplaced(RSI, 1131240662, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 223, 208, 150, 214, 92, 109, 67], OperandSize::Qword)
}

