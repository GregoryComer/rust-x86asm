use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vptestnmw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMW, operand1: Some(Direct(K6)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 246, 9, 38, 245], OperandSize::Dword)
}

#[test]
fn vptestnmw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMW, operand1: Some(Direct(K5)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexed(EBX, ECX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 222, 15, 38, 44, 139], OperandSize::Dword)
}

#[test]
fn vptestnmw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMW, operand1: Some(Direct(K7)), operand2: Some(Direct(XMM25)), operand3: Some(Direct(XMM20)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 178, 182, 6, 38, 252], OperandSize::Qword)
}

#[test]
fn vptestnmw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMW, operand1: Some(Direct(K1)), operand2: Some(Direct(XMM22)), operand3: Some(IndirectDisplaced(RDX, 414868823, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 206, 4, 38, 138, 87, 101, 186, 24], OperandSize::Qword)
}

#[test]
fn vptestnmw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMW, operand1: Some(Direct(K2)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 238, 45, 38, 209], OperandSize::Dword)
}

#[test]
fn vptestnmw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMW, operand1: Some(Direct(K4)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledDisplaced(ESI, Eight, 1798806311, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 222, 45, 38, 36, 245, 39, 155, 55, 107], OperandSize::Dword)
}

#[test]
fn vptestnmw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMW, operand1: Some(Direct(K1)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM9)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 210, 230, 43, 38, 201], OperandSize::Qword)
}

#[test]
fn vptestnmw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMW, operand1: Some(Direct(K1)), operand2: Some(Direct(YMM11)), operand3: Some(IndirectScaledIndexed(RDX, RBX, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 166, 41, 38, 12, 154], OperandSize::Qword)
}

#[test]
fn vptestnmw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMW, operand1: Some(Direct(K6)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 206, 79, 38, 246], OperandSize::Dword)
}

#[test]
fn vptestnmw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMW, operand1: Some(Direct(K6)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectDisplaced(EDX, 26508806, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 238, 77, 38, 178, 6, 126, 148, 1], OperandSize::Dword)
}

#[test]
fn vptestnmw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMW, operand1: Some(Direct(K7)), operand2: Some(Direct(ZMM29)), operand3: Some(Direct(ZMM13)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 210, 150, 66, 38, 253], OperandSize::Qword)
}

#[test]
fn vptestnmw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMW, operand1: Some(Direct(K4)), operand2: Some(Direct(ZMM22)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RAX, Two, 744123939, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 206, 71, 38, 164, 71, 35, 110, 90, 44], OperandSize::Qword)
}

