use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vunpckhps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 200, 21, 252], OperandSize::Dword)
}

#[test]
fn vunpckhps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectDisplaced(ECX, 572239806, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 208, 21, 145, 190, 175, 27, 34], OperandSize::Dword)
}

#[test]
fn vunpckhps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 208, 21, 219], OperandSize::Qword)
}

#[test]
fn vunpckhps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexed(RDX, RDI, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 232, 21, 4, 250], OperandSize::Qword)
}

#[test]
fn vunpckhps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 220, 21, 226], OperandSize::Dword)
}

#[test]
fn vunpckhps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexed(EDI, ESI, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 236, 21, 52, 119], OperandSize::Dword)
}

#[test]
fn vunpckhps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 21, 237], OperandSize::Qword)
}

#[test]
fn vunpckhps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RBX, Two, 1469200531, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 21, 140, 90, 147, 56, 146, 87], OperandSize::Qword)
}

#[test]
fn vunpckhps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 84, 143, 21, 199], OperandSize::Dword)
}

#[test]
fn vunpckhps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: Some(Indirect(EDX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 124, 137, 21, 18], OperandSize::Dword)
}

#[test]
fn vunpckhps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledDisplaced(ECX, Two, 2094060973, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 116, 154, 21, 20, 77, 173, 213, 208, 124], OperandSize::Dword)
}

#[test]
fn vunpckhps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPS, operand1: Some(Direct(XMM8)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM8)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 81, 92, 143, 21, 192], OperandSize::Qword)
}

#[test]
fn vunpckhps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPS, operand1: Some(Direct(XMM20)), operand2: Some(Direct(XMM21)), operand3: Some(Indirect(RDX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 225, 84, 129, 21, 34], OperandSize::Qword)
}

#[test]
fn vunpckhps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPS, operand1: Some(Direct(XMM22)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledDisplaced(RDI, Four, 742690672, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 225, 84, 154, 21, 52, 189, 112, 143, 68, 44], OperandSize::Qword)
}

#[test]
fn vunpckhps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 68, 173, 21, 219], OperandSize::Dword)
}

#[test]
fn vunpckhps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM5)), operand3: Some(Indirect(EDX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 84, 175, 21, 10], OperandSize::Dword)
}

#[test]
fn vunpckhps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectDisplaced(EDI, 987542126, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 92, 188, 21, 135, 110, 178, 220, 58], OperandSize::Dword)
}

#[test]
fn vunpckhps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPS, operand1: Some(Direct(YMM22)), operand2: Some(Direct(YMM23)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 225, 68, 165, 21, 244], OperandSize::Qword)
}

#[test]
fn vunpckhps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPS, operand1: Some(Direct(YMM12)), operand2: Some(Direct(YMM30)), operand3: Some(IndirectScaledIndexed(RBX, RBX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 113, 12, 163, 21, 36, 219], OperandSize::Qword)
}

#[test]
fn vunpckhps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPS, operand1: Some(Direct(YMM31)), operand2: Some(Direct(YMM30)), operand3: Some(IndirectDisplaced(RSI, 2120396709, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 97, 12, 181, 21, 190, 165, 175, 98, 126], OperandSize::Qword)
}

#[test]
fn vunpckhps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPS, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(ZMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 108, 201, 21, 239], OperandSize::Dword)
}

#[test]
fn vunpckhps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPS, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledDisplaced(ECX, Two, 277869031, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 100, 207, 21, 44, 77, 231, 241, 143, 16], OperandSize::Dword)
}

#[test]
fn vunpckhps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPS, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM1)), operand3: Some(Indirect(EDX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 116, 221, 21, 2], OperandSize::Dword)
}

#[test]
fn vunpckhps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPS, operand1: Some(Direct(ZMM29)), operand2: Some(Direct(ZMM31)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 97, 4, 194, 21, 238], OperandSize::Qword)
}

#[test]
fn vunpckhps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPS, operand1: Some(Direct(ZMM10)), operand2: Some(Direct(ZMM10)), operand3: Some(Indirect(RDX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 113, 44, 203, 21, 18], OperandSize::Qword)
}

#[test]
fn vunpckhps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPS, operand1: Some(Direct(ZMM17)), operand2: Some(Direct(ZMM8)), operand3: Some(IndirectScaledDisplaced(RCX, Two, 528080690, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 225, 60, 218, 21, 12, 77, 50, 223, 121, 31], OperandSize::Qword)
}

