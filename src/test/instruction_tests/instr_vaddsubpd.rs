use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vaddsubpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSUBPD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 208, 228], OperandSize::Dword)
}

#[test]
fn vaddsubpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSUBPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, ECX, Two, 1254418047, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 208, 180, 79, 127, 230, 196, 74], OperandSize::Dword)
}

#[test]
fn vaddsubpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSUBPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 208, 242], OperandSize::Qword)
}

#[test]
fn vaddsubpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSUBPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RDX, Two, 685185308, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 208, 156, 82, 28, 25, 215, 40], OperandSize::Qword)
}

#[test]
fn vaddsubpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSUBPD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 208, 247], OperandSize::Dword)
}

#[test]
fn vaddsubpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSUBPD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectDisplaced(ECX, 50526256, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 208, 129, 48, 248, 2, 3], OperandSize::Dword)
}

#[test]
fn vaddsubpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSUBPD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 208, 242], OperandSize::Qword)
}

#[test]
fn vaddsubpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSUBPD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexed(RCX, RDX, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 208, 44, 81], OperandSize::Qword)
}

