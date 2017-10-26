use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmovsdw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSDW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 126, 138, 35, 234], OperandSize::Dword)
}

#[test]
fn vpmovsdw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSDW, operand1: Some(IndirectScaledDisplaced(EDI, Two, 1860406710, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 8, 35, 28, 125, 182, 141, 227, 110], OperandSize::Dword)
}

#[test]
fn vpmovsdw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSDW, operand1: Some(Direct(XMM8)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 210, 126, 142, 35, 216], OperandSize::Qword)
}

#[test]
fn vpmovsdw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSDW, operand1: Some(IndirectScaledDisplaced(RBX, Four, 1221138339, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 8, 35, 4, 157, 163, 23, 201, 72], OperandSize::Qword)
}

#[test]
fn vpmovsdw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSDW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 126, 173, 35, 194], OperandSize::Dword)
}

#[test]
fn vpmovsdw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSDW, operand1: Some(Indirect(EDI, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(YMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 40, 35, 55], OperandSize::Dword)
}

#[test]
fn vpmovsdw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSDW, operand1: Some(Direct(XMM19)), operand2: Some(Direct(YMM19)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 162, 126, 172, 35, 219], OperandSize::Qword)
}

#[test]
fn vpmovsdw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSDW, operand1: Some(IndirectScaledIndexed(RDX, RBX, Four, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(YMM21)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 226, 126, 40, 35, 44, 154], OperandSize::Qword)
}

#[test]
fn vpmovsdw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSDW, operand1: Some(Direct(YMM3)), operand2: Some(Direct(ZMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 126, 207, 35, 243], OperandSize::Dword)
}

#[test]
fn vpmovsdw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSDW, operand1: Some(IndirectDisplaced(EAX, 268550850, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(ZMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 72, 35, 160, 194, 194, 1, 16], OperandSize::Dword)
}

#[test]
fn vpmovsdw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSDW, operand1: Some(Direct(YMM18)), operand2: Some(Direct(ZMM26)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 34, 126, 203, 35, 210], OperandSize::Qword)
}

#[test]
fn vpmovsdw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSDW, operand1: Some(Indirect(RSI, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(ZMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 72, 35, 62], OperandSize::Qword)
}

