use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpminsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 57, 196], OperandSize::Dword)
}

#[test]
fn vpminsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(Indirect(EDX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 57, 34], OperandSize::Dword)
}

#[test]
fn vpminsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 57, 253], OperandSize::Qword)
}

#[test]
fn vpminsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexed(RBX, RCX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 57, 20, 203], OperandSize::Qword)
}

#[test]
fn vpminsd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 109, 57, 204], OperandSize::Dword)
}

#[test]
fn vpminsd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectDisplaced(EAX, 562429735, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 77, 57, 136, 39, 255, 133, 33], OperandSize::Dword)
}

#[test]
fn vpminsd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 57, 209], OperandSize::Qword)
}

#[test]
fn vpminsd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM1)), operand3: Some(Indirect(RDI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 117, 57, 23], OperandSize::Qword)
}

#[test]
fn vpminsd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 140, 57, 211], OperandSize::Dword)
}

#[test]
fn vpminsd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EBX, Eight, 1264227026, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 93, 138, 57, 180, 218, 210, 146, 90, 75], OperandSize::Dword)
}

#[test]
fn vpminsd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: Some(Indirect(ESI, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 101, 157, 57, 38], OperandSize::Dword)
}

#[test]
fn vpminsd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSD, operand1: Some(Direct(XMM23)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM22)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 162, 93, 139, 57, 254], OperandSize::Qword)
}

#[test]
fn vpminsd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM13)), operand3: Some(IndirectDisplaced(RBX, 1534177622, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 21, 139, 57, 139, 86, 177, 113, 91], OperandSize::Qword)
}

#[test]
fn vpminsd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSD, operand1: Some(Direct(XMM24)), operand2: Some(Direct(XMM15)), operand3: Some(IndirectScaledDisplaced(RAX, Four, 1304522224, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 98, 5, 154, 57, 4, 133, 240, 109, 193, 77], OperandSize::Qword)
}

#[test]
fn vpminsd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 109, 175, 57, 241], OperandSize::Dword)
}

#[test]
fn vpminsd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM3)), operand3: Some(Indirect(EAX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 101, 174, 57, 0], OperandSize::Dword)
}

#[test]
fn vpminsd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EAX, Four, 1714544833, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 125, 185, 57, 172, 134, 193, 224, 49, 102], OperandSize::Dword)
}

#[test]
fn vpminsd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSD, operand1: Some(Direct(YMM9)), operand2: Some(Direct(YMM16)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 114, 125, 164, 57, 207], OperandSize::Qword)
}

#[test]
fn vpminsd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM27)), operand3: Some(IndirectDisplaced(RSI, 10799738, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 37, 167, 57, 142, 122, 202, 164, 0], OperandSize::Qword)
}

#[test]
fn vpminsd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSD, operand1: Some(Direct(YMM29)), operand2: Some(Direct(YMM18)), operand3: Some(IndirectScaledDisplaced(RSI, Four, 1767503400, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 98, 109, 179, 57, 44, 181, 40, 246, 89, 105], OperandSize::Qword)
}

#[test]
fn vpminsd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 93, 207, 57, 222], OperandSize::Dword)
}

#[test]
fn vpminsd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectScaledDisplaced(EBX, Four, 68824277, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 207, 57, 60, 157, 213, 44, 26, 4], OperandSize::Dword)
}

#[test]
fn vpminsd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledIndexed(EBX, EBX, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 69, 223, 57, 60, 219], OperandSize::Dword)
}

#[test]
fn vpminsd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSD, operand1: Some(Direct(ZMM28)), operand2: Some(Direct(ZMM31)), operand3: Some(Direct(ZMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 98, 5, 193, 57, 228], OperandSize::Qword)
}

#[test]
fn vpminsd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSD, operand1: Some(Direct(ZMM29)), operand2: Some(Direct(ZMM26)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RSI, Eight, 1369963194, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 98, 45, 199, 57, 172, 246, 186, 250, 167, 81], OperandSize::Qword)
}

#[test]
fn vpminsd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSD, operand1: Some(Direct(ZMM23)), operand2: Some(Direct(ZMM21)), operand3: Some(IndirectScaledDisplaced(RSI, Four, 1857955425, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 226, 85, 210, 57, 60, 181, 97, 38, 190, 110], OperandSize::Qword)
}

