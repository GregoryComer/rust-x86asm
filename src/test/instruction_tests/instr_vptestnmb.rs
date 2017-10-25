use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vptestnmb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMB, operand1: Some(Direct(K7)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 102, 11, 38, 248], OperandSize::Dword)
}

#[test]
fn vptestnmb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMB, operand1: Some(Direct(K1)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EDI, Two, 614878151, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 70, 10, 38, 140, 126, 199, 75, 166, 36], OperandSize::Dword)
}

#[test]
fn vptestnmb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMB, operand1: Some(Direct(K4)), operand2: Some(Direct(XMM8)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 62, 11, 38, 225], OperandSize::Qword)
}

#[test]
fn vptestnmb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMB, operand1: Some(Direct(K7)), operand2: Some(Direct(XMM29)), operand3: Some(IndirectScaledIndexed(RBX, RBX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 22, 1, 38, 60, 219], OperandSize::Qword)
}

#[test]
fn vptestnmb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMB, operand1: Some(Direct(K4)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 94, 42, 38, 224], OperandSize::Dword)
}

#[test]
fn vptestnmb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMB, operand1: Some(Direct(K6)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledDisplaced(EAX, Eight, 1794140754, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 70, 47, 38, 52, 197, 82, 106, 240, 106], OperandSize::Dword)
}

#[test]
fn vptestnmb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMB, operand1: Some(Direct(K5)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM14)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 210, 102, 46, 38, 238], OperandSize::Qword)
}

#[test]
fn vptestnmb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMB, operand1: Some(Direct(K6)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledDisplaced(RAX, Four, 343822520, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 126, 44, 38, 52, 133, 184, 80, 126, 20], OperandSize::Qword)
}

#[test]
fn vptestnmb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMB, operand1: Some(Direct(K1)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 86, 74, 38, 201], OperandSize::Dword)
}

#[test]
fn vptestnmb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMB, operand1: Some(Direct(K3)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectDisplaced(ECX, 1773927562, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 86, 79, 38, 153, 138, 252, 187, 105], OperandSize::Dword)
}

#[test]
fn vptestnmb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMB, operand1: Some(Direct(K5)), operand2: Some(Direct(ZMM19)), operand3: Some(Direct(ZMM26)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 146, 102, 67, 38, 234], OperandSize::Qword)
}

#[test]
fn vptestnmb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMB, operand1: Some(Direct(K1)), operand2: Some(Direct(ZMM25)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RAX, Four, 517132223, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 54, 71, 38, 140, 130, 191, 207, 210, 30], OperandSize::Qword)
}

