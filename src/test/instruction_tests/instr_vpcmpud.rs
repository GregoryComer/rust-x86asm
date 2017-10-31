use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpcmpud_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUD, operand1: Some(Direct(K1)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM2)), operand4: Some(Literal8(8)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 109, 11, 30, 202, 8], OperandSize::Dword)
}

#[test]
fn vpcmpud_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUD, operand1: Some(Direct(K4)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledDisplaced(EDX, Four, 1047907543, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(116)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 69, 10, 30, 36, 149, 215, 204, 117, 62, 116], OperandSize::Dword)
}

#[test]
fn vpcmpud_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUD, operand1: Some(Direct(K6)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, ESI, Eight, 503140956, Some(OperandSize::Dword), None)), operand4: Some(Literal8(99)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 77, 26, 30, 180, 240, 92, 82, 253, 29, 99], OperandSize::Dword)
}

#[test]
fn vpcmpud_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUD, operand1: Some(Direct(K6)), operand2: Some(Direct(XMM13)), operand3: Some(Direct(XMM4)), operand4: Some(Literal8(109)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 21, 10, 30, 244, 109], OperandSize::Qword)
}

#[test]
fn vpcmpud_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUD, operand1: Some(Direct(K4)), operand2: Some(Direct(XMM28)), operand3: Some(IndirectScaledDisplaced(RDX, Two, 1413811961, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(39)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 29, 2, 30, 36, 85, 249, 14, 69, 84, 39], OperandSize::Qword)
}

#[test]
fn vpcmpud_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUD, operand1: Some(Direct(K1)), operand2: Some(Direct(XMM13)), operand3: Some(IndirectScaledDisplaced(RDX, Eight, 637003592, Some(OperandSize::Dword), None)), operand4: Some(Literal8(41)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 21, 25, 30, 12, 213, 72, 231, 247, 37, 41], OperandSize::Qword)
}

#[test]
fn vpcmpud_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUD, operand1: Some(Direct(K4)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM4)), operand4: Some(Literal8(50)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 77, 46, 30, 228, 50], OperandSize::Dword)
}

#[test]
fn vpcmpud_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUD, operand1: Some(Direct(K4)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledDisplaced(EDI, Two, 1049724347, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(91)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 85, 47, 30, 36, 125, 187, 133, 145, 62, 91], OperandSize::Dword)
}

#[test]
fn vpcmpud_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUD, operand1: Some(Direct(K2)), operand2: Some(Direct(YMM1)), operand3: Some(Indirect(EBX, Some(OperandSize::Dword), None)), operand4: Some(Literal8(63)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 117, 58, 30, 19, 63], OperandSize::Dword)
}

#[test]
fn vpcmpud_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUD, operand1: Some(Direct(K7)), operand2: Some(Direct(YMM17)), operand3: Some(Direct(YMM18)), operand4: Some(Literal8(83)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 179, 117, 37, 30, 250, 83], OperandSize::Qword)
}

#[test]
fn vpcmpud_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUD, operand1: Some(Direct(K1)), operand2: Some(Direct(YMM31)), operand3: Some(Indirect(RDI, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(105)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 5, 38, 30, 15, 105], OperandSize::Qword)
}

#[test]
fn vpcmpud_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUD, operand1: Some(Direct(K6)), operand2: Some(Direct(YMM9)), operand3: Some(IndirectScaledIndexed(RDI, RBX, Two, Some(OperandSize::Dword), None)), operand4: Some(Literal8(54)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 53, 57, 30, 52, 95, 54], OperandSize::Qword)
}

#[test]
fn vpcmpud_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUD, operand1: Some(Direct(K1)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM4)), operand4: Some(Literal8(21)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 101, 78, 30, 204, 21], OperandSize::Dword)
}

#[test]
fn vpcmpud_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUD, operand1: Some(Direct(K7)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledDisplaced(ECX, Two, 1450164834, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(25)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 69, 75, 30, 60, 77, 98, 194, 111, 86, 25], OperandSize::Dword)
}

#[test]
fn vpcmpud_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUD, operand1: Some(Direct(K1)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledIndexed(ECX, EAX, Eight, Some(OperandSize::Dword), None)), operand4: Some(Literal8(70)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 243, 85, 92, 30, 12, 193, 70], OperandSize::Dword)
}

#[test]
fn vpcmpud_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUD, operand1: Some(Direct(K4)), operand2: Some(Direct(ZMM26)), operand3: Some(Direct(ZMM0)), operand4: Some(Literal8(44)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 45, 68, 30, 224, 44], OperandSize::Qword)
}

#[test]
fn vpcmpud_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUD, operand1: Some(Direct(K4)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectDisplaced(RBX, 1614252437, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(100)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 125, 78, 30, 163, 149, 137, 55, 96, 100], OperandSize::Qword)
}

#[test]
fn vpcmpud_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUD, operand1: Some(Direct(K3)), operand2: Some(Direct(ZMM17)), operand3: Some(IndirectDisplaced(RDI, 1398814026, Some(OperandSize::Dword), None)), operand4: Some(Literal8(80)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 243, 117, 87, 30, 159, 74, 53, 96, 83, 80], OperandSize::Qword)
}

