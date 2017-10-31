use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vptestmb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMB, operand1: Some(Direct(K2)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 77, 13, 38, 214], OperandSize::Dword)
}

#[test]
fn vptestmb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMB, operand1: Some(Direct(K1)), operand2: Some(Direct(XMM6)), operand3: Some(Indirect(ECX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 77, 12, 38, 9], OperandSize::Dword)
}

#[test]
fn vptestmb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMB, operand1: Some(Direct(K3)), operand2: Some(Direct(XMM23)), operand3: Some(Direct(XMM22)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 178, 69, 5, 38, 222], OperandSize::Qword)
}

#[test]
fn vptestmb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMB, operand1: Some(Direct(K2)), operand2: Some(Direct(XMM27)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RCX, Two, 1015087440, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 37, 4, 38, 148, 73, 80, 1, 129, 60], OperandSize::Qword)
}

#[test]
fn vptestmb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMB, operand1: Some(Direct(K2)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 101, 46, 38, 210], OperandSize::Dword)
}

#[test]
fn vptestmb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMB, operand1: Some(Direct(K5)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectDisplaced(ECX, 438616023, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 101, 41, 38, 169, 215, 191, 36, 26], OperandSize::Dword)
}

#[test]
fn vptestmb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMB, operand1: Some(Direct(K2)), operand2: Some(Direct(YMM10)), operand3: Some(Direct(YMM22)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 178, 45, 47, 38, 214], OperandSize::Qword)
}

#[test]
fn vptestmb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMB, operand1: Some(Direct(K5)), operand2: Some(Direct(YMM11)), operand3: Some(IndirectDisplaced(RDI, 1037011854, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 37, 42, 38, 175, 142, 139, 207, 61], OperandSize::Qword)
}

#[test]
fn vptestmb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMB, operand1: Some(Direct(K2)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 69, 77, 38, 213], OperandSize::Dword)
}

#[test]
fn vptestmb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMB, operand1: Some(Direct(K1)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, ESI, Two, 1688738355, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 69, 73, 38, 140, 118, 51, 26, 168, 100], OperandSize::Dword)
}

#[test]
fn vptestmb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMB, operand1: Some(Direct(K1)), operand2: Some(Direct(ZMM13)), operand3: Some(Direct(ZMM26)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 146, 21, 79, 38, 202], OperandSize::Qword)
}

#[test]
fn vptestmb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMB, operand1: Some(Direct(K7)), operand2: Some(Direct(ZMM24)), operand3: Some(IndirectScaledIndexed(RSI, RDI, Four, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 61, 68, 38, 60, 190], OperandSize::Qword)
}

