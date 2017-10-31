use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vblendmps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 77, 142, 101, 243], OperandSize::Dword)
}

#[test]
fn vblendmps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexed(ECX, EDI, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 140, 101, 12, 185], OperandSize::Dword)
}

#[test]
fn vblendmps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledDisplaced(EAX, Eight, 548001133, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 101, 157, 101, 44, 197, 109, 213, 169, 32], OperandSize::Dword)
}

#[test]
fn vblendmps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPS, operand1: Some(Direct(XMM16)), operand2: Some(Direct(XMM8)), operand3: Some(Direct(XMM12)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 194, 61, 143, 101, 196], OperandSize::Qword)
}

#[test]
fn vblendmps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM10)), operand3: Some(IndirectDisplaced(RAX, 231512305, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 45, 139, 101, 128, 241, 152, 204, 13], OperandSize::Qword)
}

#[test]
fn vblendmps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPS, operand1: Some(Direct(XMM21)), operand2: Some(Direct(XMM17)), operand3: Some(Indirect(RSI, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 226, 117, 147, 101, 46], OperandSize::Qword)
}

#[test]
fn vblendmps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 109, 175, 101, 237], OperandSize::Dword)
}

#[test]
fn vblendmps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledDisplaced(ECX, Eight, 313421148, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 77, 171, 101, 60, 205, 92, 109, 174, 18], OperandSize::Dword)
}

#[test]
fn vblendmps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectDisplaced(EDX, 319423090, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 117, 188, 101, 178, 114, 2, 10, 19], OperandSize::Dword)
}

#[test]
fn vblendmps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPS, operand1: Some(Direct(YMM8)), operand2: Some(Direct(YMM23)), operand3: Some(Direct(YMM20)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 50, 69, 166, 101, 196], OperandSize::Qword)
}

#[test]
fn vblendmps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPS, operand1: Some(Direct(YMM26)), operand2: Some(Direct(YMM25)), operand3: Some(IndirectDisplaced(RSI, 1915287288, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 98, 53, 166, 101, 150, 248, 246, 40, 114], OperandSize::Qword)
}

#[test]
fn vblendmps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPS, operand1: Some(Direct(YMM31)), operand2: Some(Direct(YMM12)), operand3: Some(IndirectDisplaced(RSI, 1893813823, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 98, 29, 186, 101, 190, 63, 78, 225, 112], OperandSize::Qword)
}

#[test]
fn vblendmps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPS, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 85, 201, 101, 243], OperandSize::Dword)
}

#[test]
fn vblendmps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPS, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM1)), operand3: Some(Indirect(ECX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 117, 202, 101, 41], OperandSize::Dword)
}

#[test]
fn vblendmps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPS, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledDisplaced(ESI, Two, 106332604, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 85, 219, 101, 12, 117, 188, 129, 86, 6], OperandSize::Dword)
}

#[test]
fn vblendmps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPS, operand1: Some(Direct(ZMM25)), operand2: Some(Direct(ZMM11)), operand3: Some(Direct(ZMM21)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 34, 37, 205, 101, 205], OperandSize::Qword)
}

#[test]
fn vblendmps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPS, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM3)), operand3: Some(Indirect(RAX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 101, 207, 101, 0], OperandSize::Qword)
}

#[test]
fn vblendmps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPS, operand1: Some(Direct(ZMM31)), operand2: Some(Direct(ZMM13)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RDX, Four, 150765436, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 98, 21, 222, 101, 188, 146, 124, 127, 252, 8], OperandSize::Qword)
}

