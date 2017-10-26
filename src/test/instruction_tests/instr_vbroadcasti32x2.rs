use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vbroadcasti32x2_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTI32x2, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 140, 89, 214], OperandSize::Dword)
}

#[test]
fn vbroadcasti32x2_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTI32x2, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, EBX, Four, 1315121329, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 137, 89, 148, 159, 177, 40, 99, 78], OperandSize::Dword)
}

#[test]
fn vbroadcasti32x2_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTI32x2, operand1: Some(Direct(XMM19)), operand2: Some(Direct(XMM30)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 130, 125, 138, 89, 222], OperandSize::Qword)
}

#[test]
fn vbroadcasti32x2_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTI32x2, operand1: Some(Direct(XMM10)), operand2: Some(Indirect(RDI, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 114, 125, 139, 89, 23], OperandSize::Qword)
}

#[test]
fn vbroadcasti32x2_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTI32x2, operand1: Some(Direct(YMM6)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 171, 89, 244], OperandSize::Dword)
}

#[test]
fn vbroadcasti32x2_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTI32x2, operand1: Some(Direct(YMM5)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, ECX, Four, 419771819, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 172, 89, 172, 142, 171, 53, 5, 25], OperandSize::Dword)
}

#[test]
fn vbroadcasti32x2_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTI32x2, operand1: Some(Direct(YMM3)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 169, 89, 217], OperandSize::Qword)
}

#[test]
fn vbroadcasti32x2_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTI32x2, operand1: Some(Direct(YMM25)), operand2: Some(IndirectScaledDisplaced(RDI, Eight, 1252165522, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 98, 125, 171, 89, 12, 253, 146, 135, 162, 74], OperandSize::Qword)
}

#[test]
fn vbroadcasti32x2_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTI32x2, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 203, 89, 205], OperandSize::Dword)
}

#[test]
fn vbroadcasti32x2_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTI32x2, operand1: Some(Direct(ZMM7)), operand2: Some(IndirectScaledIndexed(EDI, EAX, Two, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 204, 89, 60, 71], OperandSize::Dword)
}

#[test]
fn vbroadcasti32x2_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTI32x2, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(XMM11)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 210, 125, 204, 89, 235], OperandSize::Qword)
}

#[test]
fn vbroadcasti32x2_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTI32x2, operand1: Some(Direct(ZMM3)), operand2: Some(IndirectScaledIndexed(RDI, RAX, Eight, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 204, 89, 28, 199], OperandSize::Qword)
}

