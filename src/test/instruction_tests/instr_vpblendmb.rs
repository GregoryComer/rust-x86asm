use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpblendmb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMB, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 93, 140, 102, 196], OperandSize::Dword)
}

#[test]
fn vpblendmb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMB, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexed(EAX, ECX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 85, 143, 102, 44, 136], OperandSize::Dword)
}

#[test]
fn vpblendmb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMB, operand1: Some(Direct(XMM15)), operand2: Some(Direct(XMM23)), operand3: Some(Direct(XMM17)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 50, 69, 129, 102, 249], OperandSize::Qword)
}

#[test]
fn vpblendmb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMB, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM11)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RSI, Eight, 822173715, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 37, 140, 102, 132, 241, 19, 96, 1, 49], OperandSize::Qword)
}

#[test]
fn vpblendmb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMB, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 117, 169, 102, 225], OperandSize::Dword)
}

#[test]
fn vpblendmb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMB, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectDisplaced(EBX, 591973486, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 69, 175, 102, 147, 110, 204, 72, 35], OperandSize::Dword)
}

#[test]
fn vpblendmb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMB, operand1: Some(Direct(YMM15)), operand2: Some(Direct(YMM17)), operand3: Some(Direct(YMM15)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 82, 117, 164, 102, 255], OperandSize::Qword)
}

#[test]
fn vpblendmb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMB, operand1: Some(Direct(YMM12)), operand2: Some(Direct(YMM13)), operand3: Some(IndirectScaledDisplaced(RDX, Two, 1776590848, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 114, 21, 175, 102, 36, 85, 0, 160, 228, 105], OperandSize::Qword)
}

#[test]
fn vpblendmb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMB, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 101, 204, 102, 199], OperandSize::Dword)
}

#[test]
fn vpblendmb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMB, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectScaledDisplaced(EAX, Two, 490592876, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 206, 102, 28, 69, 108, 218, 61, 29], OperandSize::Dword)
}

#[test]
fn vpblendmb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMB, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM15)), operand3: Some(Direct(ZMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 5, 207, 102, 249], OperandSize::Qword)
}

#[test]
fn vpblendmb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMB, operand1: Some(Direct(ZMM19)), operand2: Some(Direct(ZMM24)), operand3: Some(IndirectScaledIndexed(RDX, RDI, Two, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 226, 61, 195, 102, 28, 122], OperandSize::Qword)
}

