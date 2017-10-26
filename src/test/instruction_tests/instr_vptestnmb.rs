use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vptestnmb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMB, operand1: Some(Direct(K3)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 118, 9, 38, 222], OperandSize::Dword)
}

#[test]
fn vptestnmb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMB, operand1: Some(Direct(K2)), operand2: Some(Direct(XMM5)), operand3: Some(Indirect(EAX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 86, 11, 38, 16], OperandSize::Dword)
}

#[test]
fn vptestnmb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMB, operand1: Some(Direct(K7)), operand2: Some(Direct(XMM28)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 30, 4, 38, 251], OperandSize::Qword)
}

#[test]
fn vptestnmb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMB, operand1: Some(Direct(K6)), operand2: Some(Direct(XMM15)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RBX, Eight, 1527039936, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 6, 12, 38, 180, 222, 192, 199, 4, 91], OperandSize::Qword)
}

#[test]
fn vptestnmb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMB, operand1: Some(Direct(K7)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 110, 45, 38, 249], OperandSize::Dword)
}

#[test]
fn vptestnmb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMB, operand1: Some(Direct(K4)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexed(EDI, EAX, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 70, 45, 38, 36, 135], OperandSize::Dword)
}

#[test]
fn vptestnmb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMB, operand1: Some(Direct(K7)), operand2: Some(Direct(YMM22)), operand3: Some(Direct(YMM11)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 210, 78, 35, 38, 251], OperandSize::Qword)
}

#[test]
fn vptestnmb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMB, operand1: Some(Direct(K4)), operand2: Some(Direct(YMM13)), operand3: Some(IndirectDisplaced(RDX, 848361445, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 22, 41, 38, 162, 229, 247, 144, 50], OperandSize::Qword)
}

#[test]
fn vptestnmb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMB, operand1: Some(Direct(K2)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 94, 73, 38, 214], OperandSize::Dword)
}

#[test]
fn vptestnmb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMB, operand1: Some(Direct(K6)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectDisplaced(ESI, 1088573722, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 102, 79, 38, 182, 26, 81, 226, 64], OperandSize::Dword)
}

#[test]
fn vptestnmb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMB, operand1: Some(Direct(K2)), operand2: Some(Direct(ZMM19)), operand3: Some(Direct(ZMM27)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 146, 102, 69, 38, 211], OperandSize::Qword)
}

#[test]
fn vptestnmb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMB, operand1: Some(Direct(K2)), operand2: Some(Direct(ZMM14)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RCX, Eight, 74873094, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 14, 75, 38, 148, 202, 6, 121, 118, 4], OperandSize::Qword)
}

