use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vblendmps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 85, 141, 101, 215], OperandSize::Dword)
}

#[test]
fn vblendmps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexed(EBX, ESI, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 85, 139, 101, 44, 115], OperandSize::Dword)
}

#[test]
fn vblendmps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(EDX, 548723924, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 117, 158, 101, 186, 212, 220, 180, 32], OperandSize::Dword)
}

#[test]
fn vblendmps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM24)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 146, 109, 143, 101, 200], OperandSize::Qword)
}

#[test]
fn vblendmps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPS, operand1: Some(Direct(XMM9)), operand2: Some(Direct(XMM27)), operand3: Some(IndirectScaledDisplaced(RDX, Two, 217269692, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 114, 37, 130, 101, 12, 85, 188, 69, 243, 12], OperandSize::Qword)
}

#[test]
fn vblendmps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM18)), operand3: Some(IndirectDisplaced(RCX, 1211723174, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 109, 149, 101, 145, 166, 109, 57, 72], OperandSize::Qword)
}

#[test]
fn vblendmps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 101, 169, 101, 245], OperandSize::Dword)
}

#[test]
fn vblendmps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, ESI, Four, 804417563, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 85, 172, 101, 132, 179, 27, 112, 242, 47], OperandSize::Dword)
}

#[test]
fn vblendmps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM2)), operand3: Some(Indirect(EBX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 109, 186, 101, 43], OperandSize::Dword)
}

#[test]
fn vblendmps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPS, operand1: Some(Direct(YMM22)), operand2: Some(Direct(YMM15)), operand3: Some(Direct(YMM10)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 194, 5, 170, 101, 242], OperandSize::Qword)
}

#[test]
fn vblendmps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPS, operand1: Some(Direct(YMM15)), operand2: Some(Direct(YMM12)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RBX, Four, 905712672, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 114, 29, 170, 101, 188, 152, 32, 20, 252, 53], OperandSize::Qword)
}

#[test]
fn vblendmps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPS, operand1: Some(Direct(YMM31)), operand2: Some(Direct(YMM30)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RDI, Four, 1773380422, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 98, 13, 177, 101, 188, 187, 70, 163, 179, 105], OperandSize::Qword)
}

#[test]
fn vblendmps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPS, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 93, 206, 101, 252], OperandSize::Dword)
}

#[test]
fn vblendmps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPS, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledDisplaced(ECX, Eight, 1894038637, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 85, 206, 101, 36, 205, 109, 188, 228, 112], OperandSize::Dword)
}

#[test]
fn vblendmps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPS, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledIndexed(EAX, EAX, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 69, 219, 101, 52, 128], OperandSize::Dword)
}

#[test]
fn vblendmps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPS, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM24)), operand3: Some(Direct(ZMM13)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 210, 61, 196, 101, 229], OperandSize::Qword)
}

#[test]
fn vblendmps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPS, operand1: Some(Direct(ZMM17)), operand2: Some(Direct(ZMM31)), operand3: Some(IndirectDisplaced(RSI, 311241636, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 226, 5, 196, 101, 142, 164, 43, 141, 18], OperandSize::Qword)
}

#[test]
fn vblendmps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPS, operand1: Some(Direct(ZMM15)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RDX, Eight, 1244212491, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 114, 109, 223, 101, 188, 215, 11, 45, 41, 74], OperandSize::Qword)
}

