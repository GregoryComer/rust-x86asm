use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vxorps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 87, 201], OperandSize::Dword)
}

#[test]
fn vxorps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledDisplaced(ESI, Two, 1349756485, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 192, 87, 20, 117, 69, 166, 115, 80], OperandSize::Dword)
}

#[test]
fn vxorps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 200, 87, 237], OperandSize::Qword)
}

#[test]
fn vxorps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexed(RDX, RAX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 216, 87, 12, 130], OperandSize::Qword)
}

#[test]
fn vxorps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 212, 87, 252], OperandSize::Dword)
}

#[test]
fn vxorps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexed(EDI, EAX, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 212, 87, 60, 135], OperandSize::Dword)
}

#[test]
fn vxorps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 204, 87, 253], OperandSize::Qword)
}

#[test]
fn vxorps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM4)), operand3: Some(Indirect(RAX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 220, 87, 40], OperandSize::Qword)
}

#[test]
fn vxorps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 92, 141, 87, 233], OperandSize::Dword)
}

#[test]
fn vxorps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 76, 139, 87, 22], OperandSize::Dword)
}

#[test]
fn vxorps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EDI, Eight, 1989829893, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 124, 157, 87, 164, 254, 5, 101, 154, 118], OperandSize::Dword)
}

#[test]
fn vxorps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPS, operand1: Some(Direct(XMM20)), operand2: Some(Direct(XMM21)), operand3: Some(Direct(XMM23)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 161, 84, 130, 87, 231], OperandSize::Qword)
}

#[test]
fn vxorps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPS, operand1: Some(Direct(XMM24)), operand2: Some(Direct(XMM27)), operand3: Some(IndirectDisplaced(RDI, 1432725399, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 97, 36, 133, 87, 135, 151, 167, 101, 85], OperandSize::Qword)
}

#[test]
fn vxorps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPS, operand1: Some(Direct(XMM21)), operand2: Some(Direct(XMM26)), operand3: Some(Indirect(RDX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 225, 44, 147, 87, 42], OperandSize::Qword)
}

#[test]
fn vxorps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 76, 170, 87, 201], OperandSize::Dword)
}

#[test]
fn vxorps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectDisplaced(EAX, 1651901019, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 68, 175, 87, 144, 91, 2, 118, 98], OperandSize::Dword)
}

#[test]
fn vxorps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM7)), operand3: Some(Indirect(ESI, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 68, 188, 87, 38], OperandSize::Dword)
}

#[test]
fn vxorps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPS, operand1: Some(Direct(YMM29)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 97, 108, 174, 87, 233], OperandSize::Qword)
}

#[test]
fn vxorps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPS, operand1: Some(Direct(YMM17)), operand2: Some(Direct(YMM27)), operand3: Some(IndirectScaledDisplaced(RCX, Four, 819264509, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 225, 36, 162, 87, 12, 141, 253, 251, 212, 48], OperandSize::Qword)
}

#[test]
fn vxorps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPS, operand1: Some(Direct(YMM22)), operand2: Some(Direct(YMM13)), operand3: Some(Indirect(RAX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 225, 20, 191, 87, 48], OperandSize::Qword)
}

#[test]
fn vxorps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPS, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 68, 204, 87, 192], OperandSize::Dword)
}

#[test]
fn vxorps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPS, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM4)), operand3: Some(Indirect(ECX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 92, 206, 87, 57], OperandSize::Dword)
}

#[test]
fn vxorps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPS, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledIndexed(EDI, ESI, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 100, 222, 87, 4, 247], OperandSize::Dword)
}

#[test]
fn vxorps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPS, operand1: Some(Direct(ZMM20)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM23)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 161, 100, 205, 87, 231], OperandSize::Qword)
}

#[test]
fn vxorps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPS, operand1: Some(Direct(ZMM27)), operand2: Some(Direct(ZMM27)), operand3: Some(IndirectScaledDisplaced(RBX, Two, 605554253, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 97, 36, 193, 87, 28, 93, 77, 6, 24, 36], OperandSize::Qword)
}

#[test]
fn vxorps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPS, operand1: Some(Direct(ZMM25)), operand2: Some(Direct(ZMM23)), operand3: Some(IndirectScaledDisplaced(RDI, Four, 645717335, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 97, 68, 214, 87, 12, 189, 87, 221, 124, 38], OperandSize::Qword)
}

