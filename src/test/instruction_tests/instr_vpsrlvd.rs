use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpsrlvd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 69, 193], OperandSize::Dword)
}

#[test]
fn vpsrlvd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexed(ESI, ECX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 69, 36, 78], OperandSize::Dword)
}

#[test]
fn vpsrlvd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 69, 226], OperandSize::Qword)
}

#[test]
fn vpsrlvd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectDisplaced(RDX, 1030658026, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 69, 178, 234, 151, 110, 61], OperandSize::Qword)
}

#[test]
fn vpsrlvd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 117, 69, 220], OperandSize::Dword)
}

#[test]
fn vpsrlvd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM6)), operand3: Some(Indirect(EDX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 77, 69, 2], OperandSize::Dword)
}

#[test]
fn vpsrlvd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 69, 210], OperandSize::Qword)
}

#[test]
fn vpsrlvd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RSI, Four, 2106703908, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 117, 69, 132, 182, 36, 192, 145, 125], OperandSize::Qword)
}

#[test]
fn vpsrlvd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 109, 140, 69, 202], OperandSize::Dword)
}

#[test]
fn vpsrlvd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: Some(Indirect(EBX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 139, 69, 43], OperandSize::Dword)
}

#[test]
fn vpsrlvd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EDX, Eight, 2016739640, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 93, 153, 69, 180, 211, 56, 1, 53, 120], OperandSize::Dword)
}

#[test]
fn vpsrlvd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVD, operand1: Some(Direct(XMM14)), operand2: Some(Direct(XMM9)), operand3: Some(Direct(XMM30)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 18, 53, 137, 69, 246], OperandSize::Qword)
}

#[test]
fn vpsrlvd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVD, operand1: Some(Direct(XMM14)), operand2: Some(Direct(XMM10)), operand3: Some(Indirect(RDX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 114, 45, 137, 69, 50], OperandSize::Qword)
}

#[test]
fn vpsrlvd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVD, operand1: Some(Direct(XMM28)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RCX, Four, 1578519463, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 98, 109, 159, 69, 164, 138, 167, 75, 22, 94], OperandSize::Qword)
}

#[test]
fn vpsrlvd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 85, 171, 69, 223], OperandSize::Dword)
}

#[test]
fn vpsrlvd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectDisplaced(EDX, 1016985143, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 93, 171, 69, 170, 55, 246, 157, 60], OperandSize::Dword)
}

#[test]
fn vpsrlvd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, ECX, Four, 462386958, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 85, 189, 69, 188, 136, 14, 119, 143, 27], OperandSize::Dword)
}

#[test]
fn vpsrlvd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM9)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 53, 171, 69, 222], OperandSize::Qword)
}

#[test]
fn vpsrlvd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVD, operand1: Some(Direct(YMM18)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledDisplaced(RBX, Four, 400773878, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 226, 117, 170, 69, 20, 157, 246, 82, 227, 23], OperandSize::Qword)
}

#[test]
fn vpsrlvd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVD, operand1: Some(Direct(YMM31)), operand2: Some(Direct(YMM24)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RDX, Eight, 2021759340, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 98, 61, 181, 69, 188, 210, 108, 153, 129, 120], OperandSize::Qword)
}

#[test]
fn vpsrlvd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 109, 206, 69, 254], OperandSize::Dword)
}

#[test]
fn vpsrlvd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledIndexed(ESI, EAX, Eight, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 77, 206, 69, 36, 198], OperandSize::Dword)
}

#[test]
fn vpsrlvd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledDisplaced(EBX, Eight, 1164255786, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 69, 219, 69, 20, 221, 42, 34, 101, 69], OperandSize::Dword)
}

#[test]
fn vpsrlvd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVD, operand1: Some(Direct(ZMM31)), operand2: Some(Direct(ZMM18)), operand3: Some(Direct(ZMM28)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 2, 109, 198, 69, 252], OperandSize::Qword)
}

#[test]
fn vpsrlvd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVD, operand1: Some(Direct(ZMM18)), operand2: Some(Direct(ZMM26)), operand3: Some(IndirectDisplaced(RDI, 504525282, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 226, 45, 197, 69, 151, 226, 113, 18, 30], OperandSize::Qword)
}

#[test]
fn vpsrlvd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVD, operand1: Some(Direct(ZMM23)), operand2: Some(Direct(ZMM8)), operand3: Some(IndirectScaledIndexed(RAX, RCX, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 226, 61, 219, 69, 60, 72], OperandSize::Qword)
}

