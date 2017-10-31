use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vptestnmb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMB, operand1: Some(Direct(K7)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 86, 12, 38, 255], OperandSize::Dword)
}

#[test]
fn vptestnmb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMB, operand1: Some(Direct(K7)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexed(ESI, EBX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 78, 15, 38, 60, 222], OperandSize::Dword)
}

#[test]
fn vptestnmb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMB, operand1: Some(Direct(K2)), operand2: Some(Direct(XMM23)), operand3: Some(Direct(XMM17)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 178, 70, 7, 38, 209], OperandSize::Qword)
}

#[test]
fn vptestnmb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMB, operand1: Some(Direct(K3)), operand2: Some(Direct(XMM4)), operand3: Some(Indirect(RAX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 94, 13, 38, 24], OperandSize::Qword)
}

#[test]
fn vptestnmb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMB, operand1: Some(Direct(K2)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 86, 41, 38, 215], OperandSize::Dword)
}

#[test]
fn vptestnmb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMB, operand1: Some(Direct(K3)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledDisplaced(EDI, Four, 1128521866, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 70, 43, 38, 28, 189, 138, 224, 67, 67], OperandSize::Dword)
}

#[test]
fn vptestnmb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMB, operand1: Some(Direct(K3)), operand2: Some(Direct(YMM13)), operand3: Some(Direct(YMM10)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 210, 22, 43, 38, 218], OperandSize::Qword)
}

#[test]
fn vptestnmb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMB, operand1: Some(Direct(K1)), operand2: Some(Direct(YMM31)), operand3: Some(IndirectDisplaced(RSI, 509200347, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 6, 36, 38, 142, 219, 199, 89, 30], OperandSize::Qword)
}

#[test]
fn vptestnmb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMB, operand1: Some(Direct(K7)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 86, 77, 38, 252], OperandSize::Dword)
}

#[test]
fn vptestnmb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMB, operand1: Some(Direct(K6)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, ECX, Two, 1682708449, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 126, 77, 38, 180, 73, 225, 23, 76, 100], OperandSize::Dword)
}

#[test]
fn vptestnmb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMB, operand1: Some(Direct(K6)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 70, 77, 38, 240], OperandSize::Qword)
}

#[test]
fn vptestnmb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMB, operand1: Some(Direct(K3)), operand2: Some(Direct(ZMM25)), operand3: Some(IndirectScaledIndexed(RBX, RSI, Eight, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 54, 70, 38, 28, 243], OperandSize::Qword)
}

