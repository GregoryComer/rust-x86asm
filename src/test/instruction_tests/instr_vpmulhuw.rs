use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmulhuw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHUW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 228, 221], OperandSize::Dword)
}

#[test]
fn vpmulhuw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHUW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectDisplaced(EDI, 667288321, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 228, 143, 1, 3, 198, 39], OperandSize::Dword)
}

#[test]
fn vpmulhuw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHUW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 228, 202], OperandSize::Qword)
}

#[test]
fn vpmulhuw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHUW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(Indirect(RAX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 228, 56], OperandSize::Qword)
}

#[test]
fn vpmulhuw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHUW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 228, 244], OperandSize::Dword)
}

#[test]
fn vpmulhuw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHUW, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledDisplaced(EDI, Two, 1383997297, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 228, 28, 125, 113, 31, 126, 82], OperandSize::Dword)
}

#[test]
fn vpmulhuw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHUW, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 228, 223], OperandSize::Qword)
}

#[test]
fn vpmulhuw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHUW, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RSI, Eight, 810129374, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 228, 172, 246, 222, 151, 73, 48], OperandSize::Qword)
}

#[test]
fn vpmulhuw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHUW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 93, 137, 228, 230], OperandSize::Dword)
}

#[test]
fn vpmulhuw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHUW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 85, 140, 228, 22], OperandSize::Dword)
}

#[test]
fn vpmulhuw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHUW, operand1: Some(Direct(XMM28)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM12)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 65, 69, 141, 228, 228], OperandSize::Qword)
}

#[test]
fn vpmulhuw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHUW, operand1: Some(Direct(XMM8)), operand2: Some(Direct(XMM28)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RSI, Four, 1647744839, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 113, 29, 135, 228, 132, 177, 71, 151, 54, 98], OperandSize::Qword)
}

#[test]
fn vpmulhuw_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHUW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 85, 171, 228, 204], OperandSize::Dword)
}

#[test]
fn vpmulhuw_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHUW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexed(ESI, EDX, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 77, 174, 228, 12, 150], OperandSize::Dword)
}

#[test]
fn vpmulhuw_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHUW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM31)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 145, 125, 175, 228, 215], OperandSize::Qword)
}

#[test]
fn vpmulhuw_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHUW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM13)), operand3: Some(IndirectDisplaced(RDX, 1031287274, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 21, 169, 228, 162, 234, 49, 120, 61], OperandSize::Qword)
}

#[test]
fn vpmulhuw_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHUW, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 69, 201, 228, 236], OperandSize::Dword)
}

#[test]
fn vpmulhuw_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHUW, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectDisplaced(EBX, 1818037788, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 125, 201, 228, 131, 28, 14, 93, 108], OperandSize::Dword)
}

#[test]
fn vpmulhuw_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHUW, operand1: Some(Direct(ZMM16)), operand2: Some(Direct(ZMM29)), operand3: Some(Direct(ZMM19)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 161, 21, 194, 228, 195], OperandSize::Qword)
}

#[test]
fn vpmulhuw_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHUW, operand1: Some(Direct(ZMM24)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RSI, Two, 2105393900, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 97, 77, 206, 228, 132, 113, 236, 194, 125, 125], OperandSize::Qword)
}

