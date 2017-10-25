use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmsubadd213ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 167, 235], OperandSize::Dword)
}

#[test]
fn vfmsubadd213ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EAX, Four, 90556032, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 167, 188, 131, 128, 198, 101, 5], OperandSize::Dword)
}

#[test]
fn vfmsubadd213ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 167, 200], OperandSize::Qword)
}

#[test]
fn vfmsubadd213ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectDisplaced(RAX, 76778131, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 167, 144, 147, 138, 147, 4], OperandSize::Qword)
}

#[test]
fn vfmsubadd213ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 109, 167, 220], OperandSize::Dword)
}

#[test]
fn vfmsubadd213ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectDisplaced(ECX, 1167310132, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 117, 167, 161, 52, 189, 147, 69], OperandSize::Dword)
}

#[test]
fn vfmsubadd213ps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 109, 167, 234], OperandSize::Qword)
}

#[test]
fn vfmsubadd213ps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM4)), operand3: Some(Indirect(RDI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 93, 167, 23], OperandSize::Qword)
}

#[test]
fn vfmsubadd213ps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 85, 140, 167, 218], OperandSize::Dword)
}

#[test]
fn vfmsubadd213ps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectDisplaced(ECX, 683181925, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 93, 141, 167, 153, 101, 135, 184, 40], OperandSize::Dword)
}

#[test]
fn vfmsubadd213ps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: Some(Indirect(ESI, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 77, 159, 167, 38], OperandSize::Dword)
}

#[test]
fn vfmsubadd213ps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PS, operand1: Some(Direct(XMM16)), operand2: Some(Direct(XMM9)), operand3: Some(Direct(XMM16)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 162, 53, 142, 167, 192], OperandSize::Qword)
}

#[test]
fn vfmsubadd213ps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PS, operand1: Some(Direct(XMM31)), operand2: Some(Direct(XMM29)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RCX, Eight, 709791487, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 98, 21, 129, 167, 188, 200, 255, 142, 78, 42], OperandSize::Qword)
}

#[test]
fn vfmsubadd213ps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PS, operand1: Some(Direct(XMM10)), operand2: Some(Direct(XMM15)), operand3: Some(IndirectScaledIndexed(RBX, RDI, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 114, 5, 155, 167, 20, 187], OperandSize::Qword)
}

#[test]
fn vfmsubadd213ps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 173, 167, 216], OperandSize::Dword)
}

#[test]
fn vfmsubadd213ps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EAX, Eight, 989503535, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 69, 170, 167, 164, 195, 47, 160, 250, 58], OperandSize::Dword)
}

#[test]
fn vfmsubadd213ps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectDisplaced(EDX, 593407055, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 125, 189, 167, 162, 79, 172, 94, 35], OperandSize::Dword)
}

#[test]
fn vfmsubadd213ps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PS, operand1: Some(Direct(YMM14)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM24)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 18, 117, 169, 167, 240], OperandSize::Qword)
}

#[test]
fn vfmsubadd213ps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM31)), operand3: Some(IndirectScaledIndexed(RBX, RBX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 5, 163, 167, 20, 219], OperandSize::Qword)
}

#[test]
fn vfmsubadd213ps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM21)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RCX, Two, 2105840564, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 85, 177, 167, 164, 74, 180, 147, 132, 125], OperandSize::Qword)
}

#[test]
fn vfmsubadd213ps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PS, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 101, 157, 167, 230], OperandSize::Dword)
}

#[test]
fn vfmsubadd213ps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PS, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM0)), operand3: Some(Indirect(EDX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 201, 167, 26], OperandSize::Dword)
}

#[test]
fn vfmsubadd213ps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PS, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectDisplaced(EAX, 1772496537, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 85, 222, 167, 176, 153, 38, 166, 105], OperandSize::Dword)
}

#[test]
fn vfmsubadd213ps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PS, operand1: Some(Direct(ZMM27)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM13)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 66, 93, 159, 167, 221], OperandSize::Qword)
}

#[test]
fn vfmsubadd213ps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PS, operand1: Some(Direct(ZMM26)), operand2: Some(Direct(ZMM11)), operand3: Some(IndirectScaledDisplaced(RSI, Two, 1156599040, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 98, 37, 204, 167, 20, 117, 0, 77, 240, 68], OperandSize::Qword)
}

#[test]
fn vfmsubadd213ps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PS, operand1: Some(Direct(ZMM8)), operand2: Some(Direct(ZMM16)), operand3: Some(IndirectScaledDisplaced(RBX, Two, 2055351172, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 114, 125, 209, 167, 4, 93, 132, 43, 130, 122], OperandSize::Qword)
}

