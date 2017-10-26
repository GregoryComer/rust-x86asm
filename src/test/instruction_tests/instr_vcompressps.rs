use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcompressps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMPRESSPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 139, 138, 223], OperandSize::Dword)
}

#[test]
fn vcompressps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMPRESSPS, operand1: Some(IndirectScaledIndexedDisplaced(ESI, EBX, Four, 1169749064, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 125, 8, 138, 156, 158, 72, 244, 184, 69], OperandSize::Dword)
}

#[test]
fn vcompressps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMPRESSPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM28)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 98, 125, 142, 138, 228], OperandSize::Qword)
}

#[test]
fn vcompressps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMPRESSPS, operand1: Some(IndirectScaledDisplaced(RDI, Eight, 1228311543, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM12)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 114, 125, 8, 138, 36, 253, 247, 139, 54, 73], OperandSize::Qword)
}

#[test]
fn vcompressps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMPRESSPS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 173, 138, 208], OperandSize::Dword)
}

#[test]
fn vcompressps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMPRESSPS, operand1: Some(IndirectScaledIndexed(EBX, ESI, Four, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 125, 40, 138, 28, 179], OperandSize::Dword)
}

#[test]
fn vcompressps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMPRESSPS, operand1: Some(Direct(YMM12)), operand2: Some(Direct(YMM24)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 66, 125, 173, 138, 196], OperandSize::Qword)
}

#[test]
fn vcompressps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMPRESSPS, operand1: Some(IndirectScaledIndexedDisplaced(RAX, RDX, Eight, 1658095888, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM11)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 114, 125, 40, 138, 156, 208, 16, 137, 212, 98], OperandSize::Qword)
}

#[test]
fn vcompressps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMPRESSPS, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 202, 138, 205], OperandSize::Dword)
}

#[test]
fn vcompressps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMPRESSPS, operand1: Some(IndirectDisplaced(ESI, 787243245, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 125, 72, 138, 158, 237, 96, 236, 46], OperandSize::Dword)
}

#[test]
fn vcompressps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMPRESSPS, operand1: Some(Direct(ZMM22)), operand2: Some(Direct(ZMM11)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 50, 125, 201, 138, 222], OperandSize::Qword)
}

#[test]
fn vcompressps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMPRESSPS, operand1: Some(IndirectScaledDisplaced(RAX, Eight, 728524866, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM23)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 226, 125, 72, 138, 60, 197, 66, 104, 108, 43], OperandSize::Qword)
}

