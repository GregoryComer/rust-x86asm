use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmovwb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVWB, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 126, 142, 48, 230], OperandSize::Dword)
}

#[test]
fn vpmovwb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVWB, operand1: Some(IndirectDisplaced(ESI, 563368942, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 8, 48, 174, 238, 83, 148, 33], OperandSize::Dword)
}

#[test]
fn vpmovwb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVWB, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM24)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 98, 126, 140, 48, 195], OperandSize::Qword)
}

#[test]
fn vpmovwb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVWB, operand1: Some(IndirectScaledDisplaced(RSI, Four, 1467060862, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM14)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 114, 126, 8, 48, 52, 181, 126, 146, 113, 87], OperandSize::Qword)
}

#[test]
fn vpmovwb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVWB, operand1: Some(Direct(XMM5)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 126, 170, 48, 221], OperandSize::Dword)
}

#[test]
fn vpmovwb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVWB, operand1: Some(IndirectScaledDisplaced(EDI, Two, 778432859, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 40, 48, 4, 125, 91, 241, 101, 46], OperandSize::Dword)
}

#[test]
fn vpmovwb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVWB, operand1: Some(Direct(XMM12)), operand2: Some(Direct(YMM21)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 194, 126, 172, 48, 236], OperandSize::Qword)
}

#[test]
fn vpmovwb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVWB, operand1: Some(IndirectScaledDisplaced(RBX, Two, 303215579, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(YMM14)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 114, 126, 40, 48, 52, 93, 219, 179, 18, 18], OperandSize::Qword)
}

#[test]
fn vpmovwb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVWB, operand1: Some(Direct(YMM2)), operand2: Some(Direct(ZMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 126, 205, 48, 226], OperandSize::Dword)
}

#[test]
fn vpmovwb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVWB, operand1: Some(IndirectDisplaced(EBX, 1969185399, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(ZMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 72, 48, 139, 119, 98, 95, 117], OperandSize::Dword)
}

#[test]
fn vpmovwb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVWB, operand1: Some(Direct(YMM14)), operand2: Some(Direct(ZMM22)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 194, 126, 205, 48, 246], OperandSize::Qword)
}

#[test]
fn vpmovwb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVWB, operand1: Some(IndirectScaledIndexedDisplaced(RDX, RSI, Two, 228769899, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(ZMM28)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 98, 126, 72, 48, 164, 114, 107, 192, 162, 13], OperandSize::Qword)
}

