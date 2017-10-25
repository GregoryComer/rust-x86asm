use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmovsdw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSDW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 126, 140, 35, 223], OperandSize::Dword)
}

#[test]
fn vpmovsdw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSDW, operand1: Some(Indirect(ESI, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 8, 35, 14], OperandSize::Dword)
}

#[test]
fn vpmovsdw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSDW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM25)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 98, 126, 140, 35, 202], OperandSize::Qword)
}

#[test]
fn vpmovsdw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSDW, operand1: Some(IndirectDisplaced(RSI, 277323989, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM19)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 226, 126, 8, 35, 158, 213, 160, 135, 16], OperandSize::Qword)
}

#[test]
fn vpmovsdw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSDW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 126, 175, 35, 228], OperandSize::Dword)
}

#[test]
fn vpmovsdw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSDW, operand1: Some(IndirectScaledDisplaced(ESI, Eight, 288753245, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 40, 35, 20, 245, 93, 6, 54, 17], OperandSize::Dword)
}

#[test]
fn vpmovsdw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSDW, operand1: Some(Direct(XMM13)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 210, 126, 173, 35, 213], OperandSize::Qword)
}

#[test]
fn vpmovsdw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSDW, operand1: Some(IndirectScaledIndexedDisplaced(RBX, RAX, Two, 1276526088, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(YMM10)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 114, 126, 40, 35, 148, 67, 8, 62, 22, 76], OperandSize::Qword)
}

#[test]
fn vpmovsdw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSDW, operand1: Some(Direct(YMM5)), operand2: Some(Direct(ZMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 126, 201, 35, 229], OperandSize::Dword)
}

#[test]
fn vpmovsdw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSDW, operand1: Some(IndirectScaledIndexed(EBX, ESI, Two, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(ZMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 72, 35, 28, 115], OperandSize::Dword)
}

#[test]
fn vpmovsdw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSDW, operand1: Some(Direct(YMM14)), operand2: Some(Direct(ZMM21)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 194, 126, 203, 35, 238], OperandSize::Qword)
}

#[test]
fn vpmovsdw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSDW, operand1: Some(IndirectDisplaced(RCX, 1533288104, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(ZMM16)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 226, 126, 72, 35, 129, 168, 30, 100, 91], OperandSize::Qword)
}

