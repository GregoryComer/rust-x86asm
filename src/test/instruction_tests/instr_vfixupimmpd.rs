use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfixupimmpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM7)), operand4: Some(Literal8(109)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 253, 139, 84, 199, 109], OperandSize::Dword)
}

#[test]
fn vfixupimmpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectDisplaced(ESI, 934776286, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(51)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 237, 138, 84, 134, 222, 141, 183, 55, 51], OperandSize::Dword)
}

#[test]
fn vfixupimmpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledDisplaced(ESI, Four, 799726258, Some(OperandSize::Qword), None)), operand4: Some(Literal8(37)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 243, 237, 155, 84, 20, 181, 178, 218, 170, 47, 37], OperandSize::Dword)
}

#[test]
fn vfixupimmpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPD, operand1: Some(Direct(XMM18)), operand2: Some(Direct(XMM10)), operand3: Some(Direct(XMM20)), operand4: Some(Literal8(72)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 163, 173, 137, 84, 212, 72], OperandSize::Qword)
}

#[test]
fn vfixupimmpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM25)), operand3: Some(IndirectScaledIndexed(RDI, RCX, Four, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(11)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 181, 134, 84, 60, 143, 11], OperandSize::Qword)
}

#[test]
fn vfixupimmpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPD, operand1: Some(Direct(XMM24)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectDisplaced(RDX, 1385657550, Some(OperandSize::Qword), None)), operand4: Some(Literal8(69)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 99, 197, 158, 84, 130, 206, 116, 151, 82, 69], OperandSize::Qword)
}

#[test]
fn vfixupimmpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM5)), operand4: Some(Literal8(6)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 197, 169, 84, 205, 6], OperandSize::Dword)
}

#[test]
fn vfixupimmpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexed(EDI, EDX, Eight, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(30)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 253, 174, 84, 20, 215, 30], OperandSize::Dword)
}

#[test]
fn vfixupimmpd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledDisplaced(ECX, Eight, 1965993757, Some(OperandSize::Qword), None)), operand4: Some(Literal8(49)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 229, 185, 84, 52, 205, 29, 175, 46, 117, 49], OperandSize::Dword)
}

#[test]
fn vfixupimmpd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPD, operand1: Some(Direct(YMM28)), operand2: Some(Direct(YMM27)), operand3: Some(Direct(YMM5)), operand4: Some(Literal8(8)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 99, 165, 163, 84, 229, 8], OperandSize::Qword)
}

#[test]
fn vfixupimmpd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPD, operand1: Some(Direct(YMM29)), operand2: Some(Direct(YMM21)), operand3: Some(IndirectDisplaced(RCX, 154425060, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(108)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 99, 213, 165, 84, 169, 228, 86, 52, 9, 108], OperandSize::Qword)
}

#[test]
fn vfixupimmpd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPD, operand1: Some(Direct(YMM16)), operand2: Some(Direct(YMM28)), operand3: Some(IndirectDisplaced(RCX, 1971353331, Some(OperandSize::Qword), None)), operand4: Some(Literal8(70)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 227, 157, 179, 84, 129, 243, 118, 128, 117, 70], OperandSize::Qword)
}

#[test]
fn vfixupimmpd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM0)), operand4: Some(Literal8(4)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 197, 158, 84, 232, 4], OperandSize::Dword)
}

#[test]
fn vfixupimmpd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectScaledIndexed(EBX, ESI, Two, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(81)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 253, 202, 84, 12, 115, 81], OperandSize::Dword)
}

#[test]
fn vfixupimmpd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM6)), operand3: Some(Indirect(EBX, Some(OperandSize::Qword), None)), operand4: Some(Literal8(106)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 205, 222, 84, 51, 106], OperandSize::Dword)
}

#[test]
fn vfixupimmpd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPD, operand1: Some(Direct(ZMM10)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM5)), operand4: Some(Literal8(26)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K5), broadcast: None }, &[98, 115, 221, 157, 84, 213, 26], OperandSize::Qword)
}

#[test]
fn vfixupimmpd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM14)), operand3: Some(IndirectScaledIndexed(RSI, RAX, Two, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(103)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 141, 207, 84, 20, 70, 103], OperandSize::Qword)
}

#[test]
fn vfixupimmpd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPD, operand1: Some(Direct(ZMM30)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RBX, Eight, 1204194090, Some(OperandSize::Qword), None)), operand4: Some(Literal8(26)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 99, 205, 223, 84, 180, 216, 42, 139, 198, 71, 26], OperandSize::Qword)
}

