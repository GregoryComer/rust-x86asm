use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmaddsub132ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 150, 217], OperandSize::Dword)
}

#[test]
fn vfmaddsub132ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 150, 14], OperandSize::Dword)
}

#[test]
fn vfmaddsub132ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 150, 248], OperandSize::Qword)
}

#[test]
fn vfmaddsub132ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexed(RBX, RDI, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 150, 4, 187], OperandSize::Qword)
}

#[test]
fn vfmaddsub132ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 150, 196], OperandSize::Dword)
}

#[test]
fn vfmaddsub132ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectDisplaced(EDI, 1158922906, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 77, 150, 143, 154, 194, 19, 69], OperandSize::Dword)
}

#[test]
fn vfmaddsub132ps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 150, 250], OperandSize::Qword)
}

#[test]
fn vfmaddsub132ps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledDisplaced(RDX, Eight, 443087073, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 117, 150, 4, 213, 225, 248, 104, 26], OperandSize::Qword)
}

#[test]
fn vfmaddsub132ps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 69, 142, 150, 253], OperandSize::Dword)
}

#[test]
fn vfmaddsub132ps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EAX, Eight, 1733601954, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 85, 139, 150, 132, 194, 162, 170, 84, 103], OperandSize::Dword)
}

#[test]
fn vfmaddsub132ps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(EBX, 2013149619, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 117, 153, 150, 155, 179, 57, 254, 119], OperandSize::Dword)
}

#[test]
fn vfmaddsub132ps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM22)), operand3: Some(Direct(XMM12)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 210, 77, 133, 150, 252], OperandSize::Qword)
}

#[test]
fn vfmaddsub132ps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PS, operand1: Some(Direct(XMM24)), operand2: Some(Direct(XMM30)), operand3: Some(IndirectScaledIndexed(RAX, RSI, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 98, 13, 129, 150, 4, 112], OperandSize::Qword)
}

#[test]
fn vfmaddsub132ps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM24)), operand3: Some(IndirectDisplaced(RBX, 651267846, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 61, 145, 150, 163, 6, 143, 209, 38], OperandSize::Qword)
}

#[test]
fn vfmaddsub132ps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 109, 170, 150, 227], OperandSize::Dword)
}

#[test]
fn vfmaddsub132ps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, EAX, Two, 1680411139, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 117, 173, 150, 180, 65, 3, 10, 41, 100], OperandSize::Dword)
}

#[test]
fn vfmaddsub132ps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EDX, Four, 631812047, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 117, 191, 150, 156, 151, 207, 175, 168, 37], OperandSize::Dword)
}

#[test]
fn vfmaddsub132ps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PS, operand1: Some(Direct(YMM26)), operand2: Some(Direct(YMM18)), operand3: Some(Direct(YMM31)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 2, 109, 167, 150, 215], OperandSize::Qword)
}

#[test]
fn vfmaddsub132ps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PS, operand1: Some(Direct(YMM10)), operand2: Some(Direct(YMM16)), operand3: Some(Indirect(RSI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 114, 125, 161, 150, 22], OperandSize::Qword)
}

#[test]
fn vfmaddsub132ps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PS, operand1: Some(Direct(YMM8)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexed(RSI, RAX, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 114, 117, 188, 150, 4, 134], OperandSize::Qword)
}

#[test]
fn vfmaddsub132ps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PS, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM4)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 85, 219, 150, 204], OperandSize::Dword)
}

#[test]
fn vfmaddsub132ps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PS, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledIndexed(EBX, ECX, Eight, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 93, 206, 150, 4, 203], OperandSize::Dword)
}

#[test]
fn vfmaddsub132ps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PS, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectDisplaced(EBX, 1694187373, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 109, 221, 150, 147, 109, 63, 251, 100], OperandSize::Dword)
}

#[test]
fn vfmaddsub132ps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PS, operand1: Some(Direct(ZMM27)), operand2: Some(Direct(ZMM18)), operand3: Some(Direct(ZMM12)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 66, 109, 149, 150, 220], OperandSize::Qword)
}

#[test]
fn vfmaddsub132ps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PS, operand1: Some(Direct(ZMM12)), operand2: Some(Direct(ZMM15)), operand3: Some(Indirect(RSI, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 114, 5, 207, 150, 38], OperandSize::Qword)
}

#[test]
fn vfmaddsub132ps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PS, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectDisplaced(RDI, 1623985692, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 69, 222, 150, 159, 28, 14, 204, 96], OperandSize::Qword)
}

