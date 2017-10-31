use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vunpcklps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 20, 215], OperandSize::Dword)
}

#[test]
fn vunpcklps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectDisplaced(ECX, 1995296193, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 232, 20, 177, 193, 205, 237, 118], OperandSize::Dword)
}

#[test]
fn vunpcklps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 240, 20, 210], OperandSize::Qword)
}

#[test]
fn vunpcklps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RSI, Two, 1121226083, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 216, 20, 164, 113, 99, 141, 212, 66], OperandSize::Qword)
}

#[test]
fn vunpcklps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 196, 20, 193], OperandSize::Dword)
}

#[test]
fn vunpcklps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexed(ESI, ECX, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 236, 20, 28, 142], OperandSize::Dword)
}

#[test]
fn vunpcklps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 228, 20, 225], OperandSize::Qword)
}

#[test]
fn vunpcklps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexed(RCX, RAX, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 204, 20, 20, 65], OperandSize::Qword)
}

#[test]
fn vunpcklps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 68, 138, 20, 247], OperandSize::Dword)
}

#[test]
fn vunpcklps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectDisplaced(EAX, 436765121, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 124, 143, 20, 128, 193, 129, 8, 26], OperandSize::Dword)
}

#[test]
fn vunpcklps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectDisplaced(EDI, 1119075592, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 76, 158, 20, 191, 8, 189, 179, 66], OperandSize::Dword)
}

#[test]
fn vunpcklps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPS, operand1: Some(Direct(XMM28)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM12)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 65, 116, 143, 20, 228], OperandSize::Qword)
}

#[test]
fn vunpcklps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM12)), operand3: Some(IndirectScaledIndexed(RDI, RSI, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 28, 138, 20, 36, 247], OperandSize::Qword)
}

#[test]
fn vunpcklps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM30)), operand3: Some(IndirectScaledIndexed(RCX, RDX, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 12, 147, 20, 60, 81], OperandSize::Qword)
}

#[test]
fn vunpcklps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 100, 171, 20, 214], OperandSize::Dword)
}

#[test]
fn vunpcklps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexed(EDI, ECX, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 100, 171, 20, 60, 79], OperandSize::Dword)
}

#[test]
fn vunpcklps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM4)), operand3: Some(Indirect(EDX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 92, 190, 20, 42], OperandSize::Dword)
}

#[test]
fn vunpcklps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM10)), operand3: Some(Direct(YMM27)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 145, 44, 173, 20, 195], OperandSize::Qword)
}

#[test]
fn vunpcklps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPS, operand1: Some(Direct(YMM13)), operand2: Some(Direct(YMM21)), operand3: Some(IndirectScaledIndexed(RBX, RBX, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 113, 84, 166, 20, 44, 155], OperandSize::Qword)
}

#[test]
fn vunpcklps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPS, operand1: Some(Direct(YMM30)), operand2: Some(Direct(YMM15)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RCX, Four, 206666638, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 97, 4, 185, 20, 180, 137, 142, 123, 81, 12], OperandSize::Qword)
}

#[test]
fn vunpcklps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPS, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 100, 202, 20, 239], OperandSize::Dword)
}

#[test]
fn vunpcklps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPS, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EDX, Four, 1091443834, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 84, 202, 20, 188, 151, 122, 28, 14, 65], OperandSize::Dword)
}

#[test]
fn vunpcklps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPS, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM0)), operand3: Some(Indirect(EDI, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 124, 218, 20, 55], OperandSize::Dword)
}

#[test]
fn vunpcklps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPS, operand1: Some(Direct(ZMM18)), operand2: Some(Direct(ZMM10)), operand3: Some(Direct(ZMM20)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 161, 44, 201, 20, 212], OperandSize::Qword)
}

#[test]
fn vunpcklps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPS, operand1: Some(Direct(ZMM19)), operand2: Some(Direct(ZMM28)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RCX, Eight, 2099683971, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 225, 28, 199, 20, 156, 201, 131, 162, 38, 125], OperandSize::Qword)
}

#[test]
fn vunpcklps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPS, operand1: Some(Direct(ZMM27)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledDisplaced(RDX, Eight, 868806599, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 97, 116, 218, 20, 28, 213, 199, 239, 200, 51], OperandSize::Qword)
}

