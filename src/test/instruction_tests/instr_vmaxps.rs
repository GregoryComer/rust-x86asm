use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmaxps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 224, 95, 231], OperandSize::Dword)
}

#[test]
fn vmaxps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, EDI, Four, 1223270500, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 216, 95, 188, 185, 100, 160, 233, 72], OperandSize::Dword)
}

#[test]
fn vmaxps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 95, 194], OperandSize::Qword)
}

#[test]
fn vmaxps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(Indirect(RAX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 208, 95, 48], OperandSize::Qword)
}

#[test]
fn vmaxps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 244, 95, 193], OperandSize::Dword)
}

#[test]
fn vmaxps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectDisplaced(EDX, 290973819, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 236, 95, 178, 123, 232, 87, 17], OperandSize::Dword)
}

#[test]
fn vmaxps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 244, 95, 200], OperandSize::Qword)
}

#[test]
fn vmaxps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexed(RBX, RSI, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 228, 95, 12, 179], OperandSize::Qword)
}

#[test]
fn vmaxps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 124, 139, 95, 250], OperandSize::Dword)
}

#[test]
fn vmaxps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: Some(Indirect(EDX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 76, 137, 95, 18], OperandSize::Dword)
}

#[test]
fn vmaxps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: Some(Indirect(EDI, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 108, 157, 95, 39], OperandSize::Dword)
}

#[test]
fn vmaxps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPS, operand1: Some(Direct(XMM16)), operand2: Some(Direct(XMM20)), operand3: Some(Direct(XMM16)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 161, 92, 130, 95, 192], OperandSize::Qword)
}

#[test]
fn vmaxps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPS, operand1: Some(Direct(XMM11)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledDisplaced(RSI, Two, 30988958, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 113, 124, 142, 95, 28, 117, 158, 218, 216, 1], OperandSize::Qword)
}

#[test]
fn vmaxps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPS, operand1: Some(Direct(XMM9)), operand2: Some(Direct(XMM3)), operand3: Some(Indirect(RAX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 113, 100, 159, 95, 8], OperandSize::Qword)
}

#[test]
fn vmaxps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 100, 170, 95, 252], OperandSize::Dword)
}

#[test]
fn vmaxps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectDisplaced(EDI, 666842447, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 116, 170, 95, 135, 79, 53, 191, 39], OperandSize::Dword)
}

#[test]
fn vmaxps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EBX, Eight, 1023369474, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 124, 191, 95, 148, 218, 2, 97, 255, 60], OperandSize::Dword)
}

#[test]
fn vmaxps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPS, operand1: Some(Direct(YMM25)), operand2: Some(Direct(YMM25)), operand3: Some(Direct(YMM17)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 33, 52, 164, 95, 201], OperandSize::Qword)
}

#[test]
fn vmaxps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPS, operand1: Some(Direct(YMM18)), operand2: Some(Direct(YMM18)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RDX, Four, 460769617, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 225, 108, 161, 95, 148, 146, 81, 201, 118, 27], OperandSize::Qword)
}

#[test]
fn vmaxps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM12)), operand3: Some(IndirectScaledIndexed(RSI, RAX, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 28, 188, 95, 20, 70], OperandSize::Qword)
}

#[test]
fn vmaxps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPS, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 84, 158, 95, 243], OperandSize::Dword)
}

#[test]
fn vmaxps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPS, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledDisplaced(EAX, Four, 166787144, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 92, 205, 95, 12, 133, 72, 248, 240, 9], OperandSize::Dword)
}

#[test]
fn vmaxps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPS, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EBX, Two, 1274980490, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 68, 219, 95, 180, 91, 138, 168, 254, 75], OperandSize::Dword)
}

#[test]
fn vmaxps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPS, operand1: Some(Direct(ZMM12)), operand2: Some(Direct(ZMM12)), operand3: Some(Direct(ZMM14)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K6), broadcast: None }, &[98, 81, 28, 158, 95, 230], OperandSize::Qword)
}

#[test]
fn vmaxps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPS, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM23)), operand3: Some(Indirect(RCX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 68, 197, 95, 9], OperandSize::Qword)
}

#[test]
fn vmaxps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPS, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM21)), operand3: Some(IndirectScaledIndexed(RAX, RSI, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 84, 209, 95, 20, 176], OperandSize::Qword)
}

