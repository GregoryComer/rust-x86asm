use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcompressps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMPRESSPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 142, 138, 235], OperandSize::Dword)
}

#[test]
fn vcompressps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMPRESSPS, operand1: Some(Indirect(EDI, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 125, 8, 138, 7], OperandSize::Dword)
}

#[test]
fn vcompressps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMPRESSPS, operand1: Some(Direct(XMM25)), operand2: Some(Direct(XMM8)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 18, 125, 138, 138, 193], OperandSize::Qword)
}

#[test]
fn vcompressps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMPRESSPS, operand1: Some(IndirectScaledIndexedDisplaced(RSI, RSI, Four, 450497781, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM14)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 114, 125, 8, 138, 180, 182, 245, 12, 218, 26], OperandSize::Qword)
}

#[test]
fn vcompressps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMPRESSPS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 174, 138, 194], OperandSize::Dword)
}

#[test]
fn vcompressps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMPRESSPS, operand1: Some(IndirectScaledIndexed(EBX, ESI, Two, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 125, 40, 138, 36, 115], OperandSize::Dword)
}

#[test]
fn vcompressps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMPRESSPS, operand1: Some(Direct(YMM31)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 146, 125, 170, 138, 231], OperandSize::Qword)
}

#[test]
fn vcompressps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMPRESSPS, operand1: Some(IndirectScaledDisplaced(RDX, Four, 1376118929, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM23)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 226, 125, 40, 138, 60, 149, 145, 232, 5, 82], OperandSize::Qword)
}

#[test]
fn vcompressps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMPRESSPS, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 204, 138, 252], OperandSize::Dword)
}

#[test]
fn vcompressps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMPRESSPS, operand1: Some(Indirect(ESI, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 125, 72, 138, 62], OperandSize::Dword)
}

#[test]
fn vcompressps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMPRESSPS, operand1: Some(Direct(ZMM27)), operand2: Some(Direct(ZMM9)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 18, 125, 207, 138, 203], OperandSize::Qword)
}

#[test]
fn vcompressps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMPRESSPS, operand1: Some(IndirectScaledIndexed(RDX, RCX, Eight, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM10)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 114, 125, 72, 138, 20, 202], OperandSize::Qword)
}

