use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpalignr_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPALIGNR, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM4)), operand4: Some(Literal8(116)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 15, 244, 116], OperandSize::Dword)
}

#[test]
fn vpalignr_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPALIGNR, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(Indirect(ECX, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(50)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 89, 15, 25, 50], OperandSize::Dword)
}

#[test]
fn vpalignr_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPALIGNR, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM6)), operand4: Some(Literal8(35)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 105, 15, 246, 35], OperandSize::Qword)
}

#[test]
fn vpalignr_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPALIGNR, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RDI, Four, 936601994, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(84)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 89, 15, 132, 187, 138, 105, 211, 55, 84], OperandSize::Qword)
}

#[test]
fn vpalignr_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPALIGNR, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM7)), operand4: Some(Literal8(48)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 85, 15, 247, 48], OperandSize::Dword)
}

#[test]
fn vpalignr_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPALIGNR, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledDisplaced(ESI, Two, 458994026, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(108)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 101, 15, 52, 117, 106, 177, 91, 27, 108], OperandSize::Dword)
}

#[test]
fn vpalignr_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPALIGNR, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM3)), operand4: Some(Literal8(20)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 117, 15, 251, 20], OperandSize::Qword)
}

#[test]
fn vpalignr_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPALIGNR, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexed(RCX, RAX, Four, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(14)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 93, 15, 36, 129, 14], OperandSize::Qword)
}

#[test]
fn vpalignr_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPALIGNR, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM7)), operand4: Some(Literal8(125)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 101, 137, 15, 223, 125], OperandSize::Dword)
}

#[test]
fn vpalignr_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPALIGNR, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledDisplaced(EAX, Two, 40297675, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(18)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 77, 143, 15, 52, 69, 203, 228, 102, 2, 18], OperandSize::Dword)
}

#[test]
fn vpalignr_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPALIGNR, operand1: Some(Direct(XMM20)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM25)), operand4: Some(Literal8(28)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 131, 69, 142, 15, 225, 28], OperandSize::Qword)
}

#[test]
fn vpalignr_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPALIGNR, operand1: Some(Direct(XMM12)), operand2: Some(Direct(XMM22)), operand3: Some(IndirectScaledDisplaced(RDX, Two, 1592046692, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(96)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 115, 77, 132, 15, 36, 85, 100, 180, 228, 94, 96], OperandSize::Qword)
}

#[test]
fn vpalignr_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPALIGNR, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM0)), operand4: Some(Literal8(46)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 125, 174, 15, 232, 46], OperandSize::Dword)
}

#[test]
fn vpalignr_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPALIGNR, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, ECX, Two, 908793607, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(80)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 125, 171, 15, 180, 78, 7, 23, 43, 54, 80], OperandSize::Dword)
}

#[test]
fn vpalignr_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPALIGNR, operand1: Some(Direct(YMM16)), operand2: Some(Direct(YMM29)), operand3: Some(Direct(YMM12)), operand4: Some(Literal8(21)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 195, 21, 167, 15, 196, 21], OperandSize::Qword)
}

#[test]
fn vpalignr_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPALIGNR, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM7)), operand3: Some(Indirect(RBX, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(45)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 69, 171, 15, 35, 45], OperandSize::Qword)
}

#[test]
fn vpalignr_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPALIGNR, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM7)), operand4: Some(Literal8(24)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 101, 205, 15, 223, 24], OperandSize::Dword)
}

#[test]
fn vpalignr_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPALIGNR, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM2)), operand3: Some(Indirect(EAX, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(123)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 109, 202, 15, 48, 123], OperandSize::Dword)
}

#[test]
fn vpalignr_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPALIGNR, operand1: Some(Direct(ZMM21)), operand2: Some(Direct(ZMM14)), operand3: Some(Direct(ZMM22)), operand4: Some(Literal8(87)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 163, 13, 202, 15, 238, 87], OperandSize::Qword)
}

#[test]
fn vpalignr_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPALIGNR, operand1: Some(Direct(ZMM20)), operand2: Some(Direct(ZMM8)), operand3: Some(Indirect(RDI, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(99)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 227, 61, 204, 15, 39, 99], OperandSize::Qword)
}

