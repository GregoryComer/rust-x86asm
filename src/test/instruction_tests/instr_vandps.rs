use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vandps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 224, 84, 250], OperandSize::Dword)
}

#[test]
fn vandps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledDisplaced(ESI, Four, 1758982748, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 208, 84, 20, 181, 92, 242, 215, 104], OperandSize::Dword)
}

#[test]
fn vandps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 232, 84, 221], OperandSize::Qword)
}

#[test]
fn vandps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(RSI, 1685408247, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 240, 84, 142, 247, 73, 117, 100], OperandSize::Qword)
}

#[test]
fn vandps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 84, 227], OperandSize::Dword)
}

#[test]
fn vandps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexed(EDX, EBX, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 212, 84, 36, 154], OperandSize::Dword)
}

#[test]
fn vandps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 212, 84, 230], OperandSize::Qword)
}

#[test]
fn vandps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM6)), operand3: Some(Indirect(RDI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 204, 84, 39], OperandSize::Qword)
}

#[test]
fn vandps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 92, 143, 84, 217], OperandSize::Dword)
}

#[test]
fn vandps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledDisplaced(EAX, Eight, 583569253, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 124, 138, 84, 36, 197, 101, 143, 200, 34], OperandSize::Dword)
}

#[test]
fn vandps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledDisplaced(ESI, Four, 1622387297, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 68, 155, 84, 60, 181, 97, 170, 179, 96], OperandSize::Dword)
}

#[test]
fn vandps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPS, operand1: Some(Direct(XMM11)), operand2: Some(Direct(XMM30)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 113, 12, 135, 84, 221], OperandSize::Qword)
}

#[test]
fn vandps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPS, operand1: Some(Direct(XMM16)), operand2: Some(Direct(XMM2)), operand3: Some(Indirect(RDI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 225, 108, 141, 84, 7], OperandSize::Qword)
}

#[test]
fn vandps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPS, operand1: Some(Direct(XMM14)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RAX, Eight, 1222470775, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 113, 124, 156, 84, 180, 194, 119, 108, 221, 72], OperandSize::Qword)
}

#[test]
fn vandps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 84, 171, 84, 216], OperandSize::Dword)
}

#[test]
fn vandps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexed(ECX, ECX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 84, 173, 84, 4, 201], OperandSize::Dword)
}

#[test]
fn vandps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledDisplaced(EAX, Four, 475063273, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 108, 187, 84, 36, 133, 233, 227, 80, 28], OperandSize::Dword)
}

#[test]
fn vandps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 92, 174, 84, 249], OperandSize::Qword)
}

#[test]
fn vandps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM10)), operand3: Some(Indirect(RAX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 44, 170, 84, 8], OperandSize::Qword)
}

#[test]
fn vandps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPS, operand1: Some(Direct(YMM23)), operand2: Some(Direct(YMM21)), operand3: Some(IndirectScaledIndexed(RSI, RAX, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 225, 84, 179, 84, 60, 198], OperandSize::Qword)
}

#[test]
fn vandps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPS, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 76, 201, 84, 229], OperandSize::Dword)
}

#[test]
fn vandps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPS, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, ECX, Two, 1114332657, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 68, 202, 84, 132, 78, 241, 93, 107, 66], OperandSize::Dword)
}

#[test]
fn vandps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPS, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, ESI, Eight, 850708616, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 76, 220, 84, 180, 247, 136, 200, 180, 50], OperandSize::Dword)
}

#[test]
fn vandps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPS, operand1: Some(Direct(ZMM10)), operand2: Some(Direct(ZMM15)), operand3: Some(Direct(ZMM10)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 81, 4, 201, 84, 210], OperandSize::Qword)
}

#[test]
fn vandps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPS, operand1: Some(Direct(ZMM19)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledDisplaced(RSI, Eight, 772319892, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 225, 100, 206, 84, 28, 245, 148, 170, 8, 46], OperandSize::Qword)
}

#[test]
fn vandps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPS, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM23)), operand3: Some(IndirectScaledDisplaced(RSI, Four, 2037252871, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 68, 214, 84, 60, 181, 7, 3, 110, 121], OperandSize::Qword)
}

