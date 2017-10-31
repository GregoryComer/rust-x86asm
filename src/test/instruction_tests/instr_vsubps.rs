use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vsubps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 208, 92, 235], OperandSize::Dword)
}

#[test]
fn vsubps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexed(ECX, EAX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 216, 92, 28, 193], OperandSize::Dword)
}

#[test]
fn vsubps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 232, 92, 232], OperandSize::Qword)
}

#[test]
fn vsubps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: Some(Indirect(RDX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 240, 92, 58], OperandSize::Qword)
}

#[test]
fn vsubps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 220, 92, 207], OperandSize::Dword)
}

#[test]
fn vsubps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, ECX, Two, 1189039817, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 92, 148, 75, 201, 78, 223, 70], OperandSize::Dword)
}

#[test]
fn vsubps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 196, 92, 215], OperandSize::Qword)
}

#[test]
fn vsubps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectDisplaced(RSI, 1858820508, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 92, 182, 156, 89, 203, 110], OperandSize::Qword)
}

#[test]
fn vsubps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 76, 140, 92, 236], OperandSize::Dword)
}

#[test]
fn vsubps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectDisplaced(ECX, 232748105, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 100, 138, 92, 153, 73, 116, 223, 13], OperandSize::Dword)
}

#[test]
fn vsubps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectDisplaced(ESI, 2092268513, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 100, 156, 92, 182, 225, 123, 181, 124], OperandSize::Dword)
}

#[test]
fn vsubps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPS, operand1: Some(Direct(XMM25)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM14)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 65, 92, 140, 92, 206], OperandSize::Qword)
}

#[test]
fn vsubps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM21)), operand3: Some(IndirectScaledIndexed(RAX, RBX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 84, 135, 92, 28, 216], OperandSize::Qword)
}

#[test]
fn vsubps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPS, operand1: Some(Direct(XMM29)), operand2: Some(Direct(XMM24)), operand3: Some(IndirectScaledIndexed(RCX, RCX, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 97, 60, 145, 92, 44, 201], OperandSize::Qword)
}

#[test]
fn vsubps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 76, 169, 92, 224], OperandSize::Dword)
}

#[test]
fn vsubps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexed(EAX, EBX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 84, 169, 92, 52, 216], OperandSize::Dword)
}

#[test]
fn vsubps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EAX, Four, 952660295, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 124, 191, 92, 132, 130, 71, 113, 200, 56], OperandSize::Dword)
}

#[test]
fn vsubps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 68, 173, 92, 255], OperandSize::Qword)
}

#[test]
fn vsubps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPS, operand1: Some(Direct(YMM31)), operand2: Some(Direct(YMM17)), operand3: Some(IndirectDisplaced(RCX, 962975963, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 97, 116, 163, 92, 185, 219, 216, 101, 57], OperandSize::Qword)
}

#[test]
fn vsubps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPS, operand1: Some(Direct(YMM27)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectDisplaced(RDI, 2109256242, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 97, 108, 187, 92, 159, 50, 178, 184, 125], OperandSize::Qword)
}

#[test]
fn vsubps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPS, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM1)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 124, 219, 92, 241], OperandSize::Dword)
}

#[test]
fn vsubps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPS, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EDI, Eight, 343386940, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 124, 203, 92, 148, 251, 60, 171, 119, 20], OperandSize::Dword)
}

#[test]
fn vsubps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPS, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EDI, Eight, 1195349302, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 108, 218, 92, 188, 254, 54, 149, 63, 71], OperandSize::Dword)
}

#[test]
fn vsubps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPS, operand1: Some(Direct(ZMM24)), operand2: Some(Direct(ZMM21)), operand3: Some(Direct(ZMM29)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 1, 84, 178, 92, 197], OperandSize::Qword)
}

#[test]
fn vsubps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPS, operand1: Some(Direct(ZMM15)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectDisplaced(RSI, 1220990015, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 113, 84, 201, 92, 190, 63, 212, 198, 72], OperandSize::Qword)
}

#[test]
fn vsubps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPS, operand1: Some(Direct(ZMM28)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectDisplaced(RBX, 204466090, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 97, 84, 223, 92, 163, 170, 231, 47, 12], OperandSize::Qword)
}

