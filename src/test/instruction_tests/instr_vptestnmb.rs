use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vptestnmb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMB, operand1: Some(Direct(K1)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 110, 15, 38, 201], OperandSize::Dword)
}

#[test]
fn vptestnmb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMB, operand1: Some(Direct(K1)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EDX, Two, 1434806535, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 126, 12, 38, 140, 86, 7, 105, 133, 85], OperandSize::Dword)
}

#[test]
fn vptestnmb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMB, operand1: Some(Direct(K3)), operand2: Some(Direct(XMM30)), operand3: Some(Direct(XMM10)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 210, 14, 7, 38, 218], OperandSize::Qword)
}

#[test]
fn vptestnmb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMB, operand1: Some(Direct(K4)), operand2: Some(Direct(XMM4)), operand3: Some(Indirect(RAX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 94, 12, 38, 32], OperandSize::Qword)
}

#[test]
fn vptestnmb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMB, operand1: Some(Direct(K7)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 70, 42, 38, 250], OperandSize::Dword)
}

#[test]
fn vptestnmb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMB, operand1: Some(Direct(K4)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledDisplaced(ESI, Four, 1071564446, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 126, 45, 38, 36, 181, 158, 198, 222, 63], OperandSize::Dword)
}

#[test]
fn vptestnmb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMB, operand1: Some(Direct(K2)), operand2: Some(Direct(YMM11)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 38, 47, 38, 209], OperandSize::Qword)
}

#[test]
fn vptestnmb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMB, operand1: Some(Direct(K7)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RCX, Two, 1132517182, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 78, 47, 38, 188, 72, 62, 215, 128, 67], OperandSize::Qword)
}

#[test]
fn vptestnmb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMB, operand1: Some(Direct(K4)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 70, 75, 38, 225], OperandSize::Dword)
}

#[test]
fn vptestnmb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMB, operand1: Some(Direct(K1)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledDisplaced(ECX, Two, 340043567, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 118, 78, 38, 12, 77, 47, 167, 68, 20], OperandSize::Dword)
}

#[test]
fn vptestnmb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMB, operand1: Some(Direct(K3)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM12)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 210, 86, 73, 38, 220], OperandSize::Qword)
}

#[test]
fn vptestnmb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMB, operand1: Some(Direct(K2)), operand2: Some(Direct(ZMM24)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RCX, Eight, 704575249, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 62, 68, 38, 148, 201, 17, 247, 254, 41], OperandSize::Qword)
}

