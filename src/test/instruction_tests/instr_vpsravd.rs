use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpsravd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 70, 241], OperandSize::Dword)
}

#[test]
fn vpsravd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: Some(Indirect(ECX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 70, 1], OperandSize::Dword)
}

#[test]
fn vpsravd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 70, 233], OperandSize::Qword)
}

#[test]
fn vpsravd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RDI, Two, 1058526696, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 70, 172, 120, 232, 213, 23, 63], OperandSize::Qword)
}

#[test]
fn vpsravd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 117, 70, 219], OperandSize::Dword)
}

#[test]
fn vpsravd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM6)), operand3: Some(Indirect(EDI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 77, 70, 55], OperandSize::Dword)
}

#[test]
fn vpsravd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 93, 70, 239], OperandSize::Qword)
}

#[test]
fn vpsravd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectDisplaced(RDI, 1774050824, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 70, 175, 8, 222, 189, 105], OperandSize::Qword)
}

#[test]
fn vpsravd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 69, 140, 70, 247], OperandSize::Dword)
}

#[test]
fn vpsravd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectDisplaced(ECX, 131615800, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 109, 142, 70, 137, 56, 76, 216, 7], OperandSize::Dword)
}

#[test]
fn vpsravd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, EDI, Eight, 1025354959, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 85, 154, 70, 172, 249, 207, 172, 29, 61], OperandSize::Dword)
}

#[test]
fn vpsravd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVD, operand1: Some(Direct(XMM11)), operand2: Some(Direct(XMM16)), operand3: Some(Direct(XMM21)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 50, 125, 134, 70, 221], OperandSize::Qword)
}

#[test]
fn vpsravd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM19)), operand3: Some(Indirect(RBX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 101, 132, 70, 35], OperandSize::Qword)
}

#[test]
fn vpsravd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVD, operand1: Some(Direct(XMM12)), operand2: Some(Direct(XMM16)), operand3: Some(Indirect(RDX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 114, 125, 145, 70, 34], OperandSize::Qword)
}

#[test]
fn vpsravd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 109, 173, 70, 247], OperandSize::Dword)
}

#[test]
fn vpsravd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM0)), operand3: Some(Indirect(EAX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 173, 70, 40], OperandSize::Dword)
}

#[test]
fn vpsravd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexed(ECX, EAX, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 125, 189, 70, 36, 193], OperandSize::Dword)
}

#[test]
fn vpsravd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVD, operand1: Some(Direct(YMM29)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 98, 109, 171, 70, 236], OperandSize::Qword)
}

#[test]
fn vpsravd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVD, operand1: Some(Direct(YMM25)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexed(RSI, RAX, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 98, 125, 171, 70, 12, 70], OperandSize::Qword)
}

#[test]
fn vpsravd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVD, operand1: Some(Direct(YMM18)), operand2: Some(Direct(YMM19)), operand3: Some(IndirectScaledDisplaced(RBX, Four, 1133066245, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 226, 101, 179, 70, 20, 157, 5, 56, 137, 67], OperandSize::Qword)
}

#[test]
fn vpsravd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(ZMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 109, 207, 70, 240], OperandSize::Dword)
}

#[test]
fn vpsravd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM5)), operand3: Some(Indirect(EDI, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 85, 202, 70, 15], OperandSize::Dword)
}

#[test]
fn vpsravd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledIndexed(EBX, EDX, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 93, 223, 70, 28, 147], OperandSize::Dword)
}

#[test]
fn vpsravd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVD, operand1: Some(Direct(ZMM28)), operand2: Some(Direct(ZMM27)), operand3: Some(Direct(ZMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 98, 37, 199, 70, 227], OperandSize::Qword)
}

#[test]
fn vpsravd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledIndexed(RDI, RAX, Two, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 85, 206, 70, 44, 71], OperandSize::Qword)
}

#[test]
fn vpsravd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVD, operand1: Some(Direct(ZMM18)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RBX, Four, 1474679125, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 226, 117, 218, 70, 148, 155, 85, 209, 229, 87], OperandSize::Qword)
}

