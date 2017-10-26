use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpminuw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 58, 221], OperandSize::Dword)
}

#[test]
fn vpminuw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledDisplaced(EDX, Four, 1071457254, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 58, 12, 149, 230, 35, 221, 63], OperandSize::Dword)
}

#[test]
fn vpminuw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 58, 235], OperandSize::Qword)
}

#[test]
fn vpminuw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectDisplaced(RBX, 514820115, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 58, 147, 19, 136, 175, 30], OperandSize::Qword)
}

#[test]
fn vpminuw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUW, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 117, 58, 253], OperandSize::Dword)
}

#[test]
fn vpminuw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUW, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledDisplaced(EDI, Eight, 2047829982, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 117, 58, 28, 253, 222, 103, 15, 122], OperandSize::Dword)
}

#[test]
fn vpminuw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 58, 202], OperandSize::Qword)
}

#[test]
fn vpminuw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUW, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledDisplaced(RDI, Four, 1714697988, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 69, 58, 60, 189, 4, 55, 52, 102], OperandSize::Qword)
}

#[test]
fn vpminuw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 109, 140, 58, 234], OperandSize::Dword)
}

#[test]
fn vpminuw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexed(EDI, ECX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 77, 141, 58, 60, 79], OperandSize::Dword)
}

#[test]
fn vpminuw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUW, operand1: Some(Direct(XMM9)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM29)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 18, 85, 137, 58, 205], OperandSize::Qword)
}

#[test]
fn vpminuw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM26)), operand3: Some(IndirectDisplaced(RBX, 49368966, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 45, 130, 58, 187, 134, 79, 241, 2], OperandSize::Qword)
}

#[test]
fn vpminuw_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUW, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 109, 174, 58, 252], OperandSize::Dword)
}

#[test]
fn vpminuw_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EDI, Eight, 534308578, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 109, 172, 58, 164, 254, 226, 230, 216, 31], OperandSize::Dword)
}

#[test]
fn vpminuw_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUW, operand1: Some(Direct(YMM26)), operand2: Some(Direct(YMM24)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 98, 61, 162, 58, 210], OperandSize::Qword)
}

#[test]
fn vpminuw_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM12)), operand3: Some(IndirectScaledIndexed(RAX, RSI, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 29, 169, 58, 4, 112], OperandSize::Qword)
}

#[test]
fn vpminuw_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUW, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(ZMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 109, 207, 58, 199], OperandSize::Dword)
}

#[test]
fn vpminuw_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUW, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectDisplaced(ESI, 493018388, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 201, 58, 150, 20, 221, 98, 29], OperandSize::Dword)
}

#[test]
fn vpminuw_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUW, operand1: Some(Direct(ZMM24)), operand2: Some(Direct(ZMM22)), operand3: Some(Direct(ZMM10)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 66, 77, 193, 58, 194], OperandSize::Qword)
}

#[test]
fn vpminuw_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUW, operand1: Some(Direct(ZMM31)), operand2: Some(Direct(ZMM18)), operand3: Some(Indirect(RDI, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 98, 109, 199, 58, 63], OperandSize::Qword)
}

