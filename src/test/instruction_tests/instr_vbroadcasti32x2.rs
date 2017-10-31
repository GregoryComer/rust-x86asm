use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vbroadcasti32x2_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTI32x2, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 139, 89, 229], OperandSize::Dword)
}

#[test]
fn vbroadcasti32x2_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTI32x2, operand1: Some(Direct(XMM7)), operand2: Some(IndirectDisplaced(EAX, 1608529645, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 140, 89, 184, 237, 54, 224, 95], OperandSize::Dword)
}

#[test]
fn vbroadcasti32x2_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTI32x2, operand1: Some(Direct(XMM14)), operand2: Some(Direct(XMM23)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 50, 125, 141, 89, 247], OperandSize::Qword)
}

#[test]
fn vbroadcasti32x2_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTI32x2, operand1: Some(Direct(XMM25)), operand2: Some(Indirect(RAX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 98, 125, 140, 89, 8], OperandSize::Qword)
}

#[test]
fn vbroadcasti32x2_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTI32x2, operand1: Some(Direct(YMM6)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 169, 89, 245], OperandSize::Dword)
}

#[test]
fn vbroadcasti32x2_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTI32x2, operand1: Some(Direct(YMM6)), operand2: Some(IndirectScaledIndexed(ESI, EAX, Four, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 170, 89, 52, 134], OperandSize::Dword)
}

#[test]
fn vbroadcasti32x2_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTI32x2, operand1: Some(Direct(YMM21)), operand2: Some(Direct(XMM15)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 194, 125, 173, 89, 239], OperandSize::Qword)
}

#[test]
fn vbroadcasti32x2_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTI32x2, operand1: Some(Direct(YMM5)), operand2: Some(Indirect(RDI, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 175, 89, 47], OperandSize::Qword)
}

#[test]
fn vbroadcasti32x2_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTI32x2, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 207, 89, 242], OperandSize::Dword)
}

#[test]
fn vbroadcasti32x2_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTI32x2, operand1: Some(Direct(ZMM1)), operand2: Some(IndirectScaledIndexed(ESI, ESI, Eight, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 205, 89, 12, 246], OperandSize::Dword)
}

#[test]
fn vbroadcasti32x2_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTI32x2, operand1: Some(Direct(ZMM22)), operand2: Some(Direct(XMM16)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 162, 125, 204, 89, 240], OperandSize::Qword)
}

#[test]
fn vbroadcasti32x2_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTI32x2, operand1: Some(Direct(ZMM1)), operand2: Some(IndirectScaledDisplaced(RBX, Four, 1158113845, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 203, 89, 12, 157, 53, 106, 7, 69], OperandSize::Qword)
}

