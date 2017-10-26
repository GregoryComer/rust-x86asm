use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vblendvpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDVPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM2)), operand4: Some(Direct(XMM3)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 97, 75, 250, 48], OperandSize::Dword)
}

#[test]
fn vblendvpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDVPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: Some(Indirect(ECX, Some(OperandSize::Xmmword), None)), operand4: Some(Direct(XMM1)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 113, 75, 57, 16], OperandSize::Dword)
}

#[test]
fn vblendvpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDVPD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM6)), operand4: Some(Direct(XMM2)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 89, 75, 198, 32], OperandSize::Qword)
}

#[test]
fn vblendvpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDVPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledDisplaced(RDI, Two, 675917658, Some(OperandSize::Xmmword), None)), operand4: Some(Direct(XMM0)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 97, 75, 52, 125, 90, 175, 73, 40, 0], OperandSize::Qword)
}

#[test]
fn vblendvpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDVPD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM0)), operand4: Some(Direct(YMM1)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 69, 75, 224, 16], OperandSize::Dword)
}

#[test]
fn vblendvpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDVPD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexed(EDI, ECX, Eight, Some(OperandSize::Ymmword), None)), operand4: Some(Direct(YMM4)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 93, 75, 36, 207, 64], OperandSize::Dword)
}

#[test]
fn vblendvpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDVPD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM7)), operand4: Some(Direct(YMM0)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 117, 75, 231, 0], OperandSize::Qword)
}

#[test]
fn vblendvpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDVPD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledDisplaced(RAX, Eight, 2052297718, Some(OperandSize::Ymmword), None)), operand4: Some(Direct(YMM6)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 75, 36, 197, 246, 147, 83, 122, 96], OperandSize::Qword)
}

