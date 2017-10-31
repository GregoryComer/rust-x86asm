use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcompressps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMPRESSPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 140, 138, 254], OperandSize::Dword)
}

#[test]
fn vcompressps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMPRESSPS, operand1: Some(IndirectDisplaced(EAX, 1311958448, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 125, 8, 138, 152, 176, 229, 50, 78], OperandSize::Dword)
}

#[test]
fn vcompressps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMPRESSPS, operand1: Some(Direct(XMM25)), operand2: Some(Direct(XMM25)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 2, 125, 141, 138, 201], OperandSize::Qword)
}

#[test]
fn vcompressps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMPRESSPS, operand1: Some(IndirectDisplaced(RBX, 1785068638, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM26)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 98, 125, 8, 138, 147, 94, 252, 101, 106], OperandSize::Qword)
}

#[test]
fn vcompressps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMPRESSPS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 171, 138, 239], OperandSize::Dword)
}

#[test]
fn vcompressps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMPRESSPS, operand1: Some(IndirectScaledIndexed(EDI, EAX, Two, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 125, 40, 138, 60, 71], OperandSize::Dword)
}

#[test]
fn vcompressps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMPRESSPS, operand1: Some(Direct(YMM23)), operand2: Some(Direct(YMM22)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 162, 125, 175, 138, 247], OperandSize::Qword)
}

#[test]
fn vcompressps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMPRESSPS, operand1: Some(IndirectScaledDisplaced(RBX, Two, 283239487, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 125, 40, 138, 4, 93, 63, 228, 225, 16], OperandSize::Qword)
}

#[test]
fn vcompressps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMPRESSPS, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 207, 138, 203], OperandSize::Dword)
}

#[test]
fn vcompressps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMPRESSPS, operand1: Some(IndirectScaledIndexedDisplaced(EDX, EBX, Four, 1519844282, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 125, 72, 138, 140, 154, 186, 251, 150, 90], OperandSize::Dword)
}

#[test]
fn vcompressps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMPRESSPS, operand1: Some(Direct(ZMM21)), operand2: Some(Direct(ZMM20)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 162, 125, 206, 138, 229], OperandSize::Qword)
}

#[test]
fn vcompressps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMPRESSPS, operand1: Some(Indirect(RAX, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 125, 72, 138, 24], OperandSize::Qword)
}

