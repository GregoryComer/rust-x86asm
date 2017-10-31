use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpalignr_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPALIGNR, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM1)), operand4: Some(Literal8(24)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 81, 15, 193, 24], OperandSize::Dword)
}

#[test]
fn vpalignr_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPALIGNR, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EAX, Eight, 1062719567, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(40)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 89, 15, 148, 194, 79, 208, 87, 63, 40], OperandSize::Dword)
}

#[test]
fn vpalignr_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPALIGNR, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM2)), operand4: Some(Literal8(47)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 15, 210, 47], OperandSize::Qword)
}

#[test]
fn vpalignr_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPALIGNR, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RDX, Two, 1095904325, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(21)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 15, 180, 86, 69, 44, 82, 65, 21], OperandSize::Qword)
}

#[test]
fn vpalignr_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPALIGNR, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM2)), operand4: Some(Literal8(82)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 109, 15, 250, 82], OperandSize::Dword)
}

#[test]
fn vpalignr_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPALIGNR, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexed(EDI, ECX, Eight, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(54)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 109, 15, 20, 207, 54], OperandSize::Dword)
}

#[test]
fn vpalignr_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPALIGNR, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM6)), operand4: Some(Literal8(76)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 77, 15, 246, 76], OperandSize::Qword)
}

#[test]
fn vpalignr_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPALIGNR, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectDisplaced(RDX, 561811290, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(39)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 93, 15, 186, 90, 143, 124, 33, 39], OperandSize::Qword)
}

#[test]
fn vpalignr_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPALIGNR, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM6)), operand4: Some(Literal8(50)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 125, 142, 15, 222, 50], OperandSize::Dword)
}

#[test]
fn vpalignr_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPALIGNR, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectDisplaced(EDI, 1432527474, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(94)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 69, 141, 15, 159, 114, 162, 98, 85, 94], OperandSize::Dword)
}

#[test]
fn vpalignr_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPALIGNR, operand1: Some(Direct(XMM26)), operand2: Some(Direct(XMM14)), operand3: Some(Direct(XMM2)), operand4: Some(Literal8(122)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 99, 13, 142, 15, 210, 122], OperandSize::Qword)
}

#[test]
fn vpalignr_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPALIGNR, operand1: Some(Direct(XMM10)), operand2: Some(Direct(XMM16)), operand3: Some(IndirectScaledDisplaced(RBX, Four, 816972954, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(17)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 115, 125, 135, 15, 20, 157, 154, 4, 178, 48, 17], OperandSize::Qword)
}

#[test]
fn vpalignr_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPALIGNR, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM7)), operand4: Some(Literal8(13)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 85, 172, 15, 239, 13], OperandSize::Dword)
}

#[test]
fn vpalignr_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPALIGNR, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexed(ESI, EDX, Eight, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(34)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 117, 172, 15, 60, 214, 34], OperandSize::Dword)
}

#[test]
fn vpalignr_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPALIGNR, operand1: Some(Direct(YMM10)), operand2: Some(Direct(YMM8)), operand3: Some(Direct(YMM24)), operand4: Some(Literal8(44)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 19, 61, 172, 15, 208, 44], OperandSize::Qword)
}

#[test]
fn vpalignr_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPALIGNR, operand1: Some(Direct(YMM9)), operand2: Some(Direct(YMM31)), operand3: Some(IndirectScaledDisplaced(RDX, Four, 120016544, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(124)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 115, 5, 163, 15, 12, 149, 160, 78, 39, 7, 124], OperandSize::Qword)
}

#[test]
fn vpalignr_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPALIGNR, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM6)), operand4: Some(Literal8(25)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 77, 205, 15, 214, 25], OperandSize::Dword)
}

#[test]
fn vpalignr_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPALIGNR, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectDisplaced(EDI, 1738246378, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(3)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 109, 207, 15, 183, 234, 136, 155, 103, 3], OperandSize::Dword)
}

#[test]
fn vpalignr_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPALIGNR, operand1: Some(Direct(ZMM21)), operand2: Some(Direct(ZMM24)), operand3: Some(Direct(ZMM31)), operand4: Some(Literal8(86)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 131, 61, 197, 15, 239, 86], OperandSize::Qword)
}

#[test]
fn vpalignr_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPALIGNR, operand1: Some(Direct(ZMM17)), operand2: Some(Direct(ZMM17)), operand3: Some(IndirectScaledDisplaced(RDX, Eight, 1956087719, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(55)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 227, 117, 193, 15, 12, 213, 167, 135, 151, 116, 55], OperandSize::Qword)
}

