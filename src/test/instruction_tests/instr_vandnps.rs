use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vandnps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 200, 85, 193], OperandSize::Dword)
}

#[test]
fn vandnps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, ECX, Two, 1921688232, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 192, 85, 164, 73, 168, 162, 138, 114], OperandSize::Dword)
}

#[test]
fn vandnps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 85, 234], OperandSize::Qword)
}

#[test]
fn vandnps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RSI, Eight, 681842099, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 192, 85, 188, 243, 179, 21, 164, 40], OperandSize::Qword)
}

#[test]
fn vandnps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 236, 85, 195], OperandSize::Dword)
}

#[test]
fn vandnps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexed(EBX, EBX, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 196, 85, 52, 91], OperandSize::Dword)
}

#[test]
fn vandnps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 236, 85, 222], OperandSize::Qword)
}

#[test]
fn vandnps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectDisplaced(RAX, 1574271261, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 204, 85, 176, 29, 121, 213, 93], OperandSize::Qword)
}

#[test]
fn vandnps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 68, 137, 85, 248], OperandSize::Dword)
}

#[test]
fn vandnps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectDisplaced(EDX, 1997641946, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 108, 140, 85, 178, 218, 152, 17, 119], OperandSize::Dword)
}

#[test]
fn vandnps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectDisplaced(EBX, 1890506844, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 100, 154, 85, 139, 92, 216, 174, 112], OperandSize::Dword)
}

#[test]
fn vandnps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPS, operand1: Some(Direct(XMM9)), operand2: Some(Direct(XMM13)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 113, 20, 141, 85, 200], OperandSize::Qword)
}

#[test]
fn vandnps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPS, operand1: Some(Direct(XMM15)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RBX, Four, 1290746213, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 113, 92, 140, 85, 188, 159, 101, 57, 239, 76], OperandSize::Qword)
}

#[test]
fn vandnps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPS, operand1: Some(Direct(XMM21)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexed(RSI, RCX, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 225, 100, 156, 85, 44, 78], OperandSize::Qword)
}

#[test]
fn vandnps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 76, 172, 85, 254], OperandSize::Dword)
}

#[test]
fn vandnps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledDisplaced(EDX, Two, 1555839728, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 108, 174, 85, 4, 85, 240, 58, 188, 92], OperandSize::Dword)
}

#[test]
fn vandnps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledDisplaced(EAX, Eight, 979021679, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 84, 186, 85, 44, 197, 111, 175, 90, 58], OperandSize::Dword)
}

#[test]
fn vandnps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPS, operand1: Some(Direct(YMM23)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 225, 124, 172, 85, 249], OperandSize::Qword)
}

#[test]
fn vandnps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPS, operand1: Some(Direct(YMM20)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledDisplaced(RCX, Two, 1474688237, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 225, 68, 171, 85, 36, 77, 237, 244, 229, 87], OperandSize::Qword)
}

#[test]
fn vandnps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPS, operand1: Some(Direct(YMM31)), operand2: Some(Direct(YMM7)), operand3: Some(Indirect(RBX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 97, 68, 187, 85, 59], OperandSize::Qword)
}

#[test]
fn vandnps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPS, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 124, 205, 85, 204], OperandSize::Dword)
}

#[test]
fn vandnps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPS, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledDisplaced(EAX, Four, 509224012, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 76, 204, 85, 12, 133, 76, 36, 90, 30], OperandSize::Dword)
}

#[test]
fn vandnps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPS, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM6)), operand3: Some(Indirect(EDI, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 76, 221, 85, 31], OperandSize::Dword)
}

#[test]
fn vandnps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPS, operand1: Some(Direct(ZMM24)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 97, 124, 203, 85, 192], OperandSize::Qword)
}

#[test]
fn vandnps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPS, operand1: Some(Direct(ZMM21)), operand2: Some(Direct(ZMM21)), operand3: Some(IndirectDisplaced(RBX, 1678506354, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 225, 84, 197, 85, 171, 114, 249, 11, 100], OperandSize::Qword)
}

#[test]
fn vandnps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPS, operand1: Some(Direct(ZMM22)), operand2: Some(Direct(ZMM21)), operand3: Some(IndirectDisplaced(RDX, 344634766, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 225, 84, 209, 85, 178, 142, 181, 138, 20], OperandSize::Qword)
}

