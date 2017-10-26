use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpalignr_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPALIGNR, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM7)), operand4: Some(Literal8(105)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 15, 207, 105], OperandSize::Dword)
}

#[test]
fn vpalignr_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPALIGNR, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EBX, Eight, 268058046, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(29)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 65, 15, 148, 222, 190, 61, 250, 15, 29], OperandSize::Dword)
}

#[test]
fn vpalignr_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPALIGNR, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM4)), operand4: Some(Literal8(120)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 113, 15, 204, 120], OperandSize::Qword)
}

#[test]
fn vpalignr_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPALIGNR, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledDisplaced(RDI, Eight, 1800746546, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(110)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 89, 15, 36, 253, 50, 54, 85, 107, 110], OperandSize::Qword)
}

#[test]
fn vpalignr_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPALIGNR, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM1)), operand4: Some(Literal8(86)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 85, 15, 217, 86], OperandSize::Dword)
}

#[test]
fn vpalignr_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPALIGNR, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM7)), operand3: Some(Indirect(EDI, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(95)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 69, 15, 63, 95], OperandSize::Dword)
}

#[test]
fn vpalignr_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPALIGNR, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM7)), operand4: Some(Literal8(115)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 69, 15, 223, 115], OperandSize::Qword)
}

#[test]
fn vpalignr_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPALIGNR, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM0)), operand3: Some(Indirect(RCX, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(103)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 15, 33, 103], OperandSize::Qword)
}

#[test]
fn vpalignr_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPALIGNR, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM3)), operand4: Some(Literal8(65)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 93, 141, 15, 251, 65], OperandSize::Dword)
}

#[test]
fn vpalignr_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPALIGNR, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EDI, Four, 1768837426, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(105)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 125, 138, 15, 132, 184, 50, 81, 110, 105, 105], OperandSize::Dword)
}

#[test]
fn vpalignr_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPALIGNR, operand1: Some(Direct(XMM12)), operand2: Some(Direct(XMM20)), operand3: Some(Direct(XMM14)), operand4: Some(Literal8(68)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 83, 93, 133, 15, 230, 68], OperandSize::Qword)
}

#[test]
fn vpalignr_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPALIGNR, operand1: Some(Direct(XMM9)), operand2: Some(Direct(XMM27)), operand3: Some(Indirect(RDI, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(67)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 115, 37, 135, 15, 15, 67], OperandSize::Qword)
}

#[test]
fn vpalignr_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPALIGNR, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM4)), operand4: Some(Literal8(92)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 93, 173, 15, 236, 92], OperandSize::Dword)
}

#[test]
fn vpalignr_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPALIGNR, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM4)), operand3: Some(Indirect(EAX, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(103)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 93, 174, 15, 0, 103], OperandSize::Dword)
}

#[test]
fn vpalignr_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPALIGNR, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM21)), operand3: Some(Direct(YMM10)), operand4: Some(Literal8(31)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 211, 85, 164, 15, 242, 31], OperandSize::Qword)
}

#[test]
fn vpalignr_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPALIGNR, operand1: Some(Direct(YMM8)), operand2: Some(Direct(YMM23)), operand3: Some(IndirectScaledDisplaced(RCX, Eight, 382618992, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(17)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 115, 69, 161, 15, 4, 205, 112, 77, 206, 22, 17], OperandSize::Qword)
}

#[test]
fn vpalignr_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPALIGNR, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM5)), operand4: Some(Literal8(69)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 101, 201, 15, 253, 69], OperandSize::Dword)
}

#[test]
fn vpalignr_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPALIGNR, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledIndexed(EBX, ESI, Two, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(0)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 77, 205, 15, 20, 115, 0], OperandSize::Dword)
}

#[test]
fn vpalignr_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPALIGNR, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM2)), operand4: Some(Literal8(60)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 101, 202, 15, 202, 60], OperandSize::Qword)
}

#[test]
fn vpalignr_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPALIGNR, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM30)), operand3: Some(IndirectDisplaced(RCX, 216193632, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(126)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 13, 193, 15, 153, 96, 218, 226, 12, 126], OperandSize::Qword)
}

