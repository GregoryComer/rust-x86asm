use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vbroadcasti32x2_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTI32x2, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 142, 89, 202], OperandSize::Dword)
}

#[test]
fn vbroadcasti32x2_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTI32x2, operand1: Some(Direct(XMM7)), operand2: Some(IndirectDisplaced(EDI, 1454739409, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 142, 89, 191, 209, 143, 181, 86], OperandSize::Dword)
}

#[test]
fn vbroadcasti32x2_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTI32x2, operand1: Some(Direct(XMM29)), operand2: Some(Direct(XMM27)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 2, 125, 143, 89, 235], OperandSize::Qword)
}

#[test]
fn vbroadcasti32x2_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTI32x2, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexed(RDI, RBX, Eight, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 141, 89, 44, 223], OperandSize::Qword)
}

#[test]
fn vbroadcasti32x2_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTI32x2, operand1: Some(Direct(YMM2)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 173, 89, 212], OperandSize::Dword)
}

#[test]
fn vbroadcasti32x2_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTI32x2, operand1: Some(Direct(YMM7)), operand2: Some(IndirectScaledDisplaced(EBX, Four, 93071856, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 170, 89, 60, 157, 240, 41, 140, 5], OperandSize::Dword)
}

#[test]
fn vbroadcasti32x2_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTI32x2, operand1: Some(Direct(YMM9)), operand2: Some(Direct(XMM24)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 18, 125, 175, 89, 200], OperandSize::Qword)
}

#[test]
fn vbroadcasti32x2_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTI32x2, operand1: Some(Direct(YMM23)), operand2: Some(IndirectDisplaced(RBX, 1920552976, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 226, 125, 172, 89, 187, 16, 80, 121, 114], OperandSize::Qword)
}

#[test]
fn vbroadcasti32x2_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTI32x2, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 206, 89, 223], OperandSize::Dword)
}

#[test]
fn vbroadcasti32x2_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTI32x2, operand1: Some(Direct(ZMM6)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, ESI, Four, 283711840, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 201, 89, 180, 178, 96, 25, 233, 16], OperandSize::Dword)
}

#[test]
fn vbroadcasti32x2_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTI32x2, operand1: Some(Direct(ZMM10)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 114, 125, 204, 89, 209], OperandSize::Qword)
}

#[test]
fn vbroadcasti32x2_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTI32x2, operand1: Some(Direct(ZMM13)), operand2: Some(IndirectDisplaced(RAX, 1726543663, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 114, 125, 203, 89, 168, 47, 247, 232, 102], OperandSize::Qword)
}

