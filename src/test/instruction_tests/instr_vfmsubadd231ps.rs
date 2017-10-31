use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmsubadd231ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 183, 251], OperandSize::Dword)
}

#[test]
fn vfmsubadd231ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, ESI, Two, 415464669, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 183, 164, 114, 221, 124, 195, 24], OperandSize::Dword)
}

#[test]
fn vfmsubadd231ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 183, 202], OperandSize::Qword)
}

#[test]
fn vfmsubadd231ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: Some(Indirect(RDI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 183, 23], OperandSize::Qword)
}

#[test]
fn vfmsubadd231ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 109, 183, 235], OperandSize::Dword)
}

#[test]
fn vfmsubadd231ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, ECX, Eight, 79494882, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 117, 183, 156, 201, 226, 254, 188, 4], OperandSize::Dword)
}

#[test]
fn vfmsubadd231ps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 109, 183, 199], OperandSize::Qword)
}

#[test]
fn vfmsubadd231ps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledDisplaced(RBX, Eight, 1408582722, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 183, 36, 221, 66, 68, 245, 83], OperandSize::Qword)
}

#[test]
fn vfmsubadd231ps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 109, 140, 183, 238], OperandSize::Dword)
}

#[test]
fn vfmsubadd231ps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexed(EAX, ECX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 69, 141, 183, 20, 72], OperandSize::Dword)
}

#[test]
fn vfmsubadd231ps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledDisplaced(EDI, Four, 95635647, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 109, 153, 183, 52, 189, 191, 72, 179, 5], OperandSize::Dword)
}

#[test]
fn vfmsubadd231ps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PS, operand1: Some(Direct(XMM16)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 226, 77, 141, 183, 199], OperandSize::Qword)
}

#[test]
fn vfmsubadd231ps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM19)), operand3: Some(IndirectScaledDisplaced(RCX, Eight, 1704694983, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 101, 131, 183, 52, 205, 199, 148, 155, 101], OperandSize::Qword)
}

#[test]
fn vfmsubadd231ps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PS, operand1: Some(Direct(XMM18)), operand2: Some(Direct(XMM28)), operand3: Some(Indirect(RSI, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 226, 29, 151, 183, 22], OperandSize::Qword)
}

#[test]
fn vfmsubadd231ps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 93, 173, 183, 238], OperandSize::Dword)
}

#[test]
fn vfmsubadd231ps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EAX, Four, 105943102, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 93, 173, 183, 156, 131, 62, 144, 80, 6], OperandSize::Dword)
}

#[test]
fn vfmsubadd231ps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectDisplaced(EDI, 1360600071, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 117, 189, 183, 143, 7, 28, 25, 81], OperandSize::Dword)
}

#[test]
fn vfmsubadd231ps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PS, operand1: Some(Direct(YMM21)), operand2: Some(Direct(YMM30)), operand3: Some(Direct(YMM20)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 162, 13, 165, 183, 236], OperandSize::Qword)
}

#[test]
fn vfmsubadd231ps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PS, operand1: Some(Direct(YMM18)), operand2: Some(Direct(YMM11)), operand3: Some(IndirectDisplaced(RAX, 155098460, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 226, 37, 170, 183, 144, 92, 157, 62, 9], OperandSize::Qword)
}

#[test]
fn vfmsubadd231ps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PS, operand1: Some(Direct(YMM29)), operand2: Some(Direct(YMM12)), operand3: Some(IndirectScaledDisplaced(RSI, Two, 334763950, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 98, 29, 185, 183, 44, 117, 174, 23, 244, 19], OperandSize::Qword)
}

#[test]
fn vfmsubadd231ps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PS, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM5)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 69, 153, 183, 205], OperandSize::Dword)
}

#[test]
fn vfmsubadd231ps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PS, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM5)), operand3: Some(Indirect(EDX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 85, 203, 183, 50], OperandSize::Dword)
}

#[test]
fn vfmsubadd231ps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PS, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM7)), operand3: Some(Indirect(EAX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 69, 222, 183, 8], OperandSize::Dword)
}

#[test]
fn vfmsubadd231ps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PS, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM27)), operand3: Some(Direct(ZMM12)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 210, 37, 209, 183, 228], OperandSize::Qword)
}

#[test]
fn vfmsubadd231ps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PS, operand1: Some(Direct(ZMM11)), operand2: Some(Direct(ZMM14)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RAX, Two, 1949934324, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 114, 13, 206, 183, 156, 70, 244, 162, 57, 116], OperandSize::Qword)
}

#[test]
fn vfmsubadd231ps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PS, operand1: Some(Direct(ZMM12)), operand2: Some(Direct(ZMM2)), operand3: Some(Indirect(RDI, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 114, 109, 220, 183, 39], OperandSize::Qword)
}

