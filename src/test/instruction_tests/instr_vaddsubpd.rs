use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vaddsubpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSUBPD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 208, 197], OperandSize::Dword)
}

#[test]
fn vaddsubpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSUBPD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledDisplaced(EBX, Two, 1971932777, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 208, 4, 93, 105, 78, 137, 117], OperandSize::Dword)
}

#[test]
fn vaddsubpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSUBPD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 208, 239], OperandSize::Qword)
}

#[test]
fn vaddsubpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSUBPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexed(RAX, RSI, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 208, 52, 112], OperandSize::Qword)
}

#[test]
fn vaddsubpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSUBPD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 208, 246], OperandSize::Dword)
}

#[test]
fn vaddsubpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSUBPD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectDisplaced(ECX, 329529803, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 208, 185, 203, 57, 164, 19], OperandSize::Dword)
}

#[test]
fn vaddsubpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSUBPD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 208, 213], OperandSize::Qword)
}

#[test]
fn vaddsubpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSUBPD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectDisplaced(RSI, 1668870441, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 208, 150, 41, 241, 120, 99], OperandSize::Qword)
}

