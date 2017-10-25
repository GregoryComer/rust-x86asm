use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpcmpud_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUD, operand1: Some(Direct(K4)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM6)), operand4: Some(Literal8(5)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 109, 13, 30, 230, 5], OperandSize::Dword)
}

#[test]
fn vpcmpud_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUD, operand1: Some(Direct(K7)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledDisplaced(EBX, Four, 1193275767, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(51)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 93, 9, 30, 60, 157, 119, 241, 31, 71, 51], OperandSize::Dword)
}

#[test]
fn vpcmpud_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUD, operand1: Some(Direct(K6)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, ECX, Four, 1358603298, Some(OperandSize::Dword), None)), operand4: Some(Literal8(96)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 85, 28, 30, 180, 136, 34, 164, 250, 80, 96], OperandSize::Dword)
}

#[test]
fn vpcmpud_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUD, operand1: Some(Direct(K5)), operand2: Some(Direct(XMM12)), operand3: Some(Direct(XMM24)), operand4: Some(Literal8(87)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 147, 29, 12, 30, 232, 87], OperandSize::Qword)
}

#[test]
fn vpcmpud_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUD, operand1: Some(Direct(K3)), operand2: Some(Direct(XMM12)), operand3: Some(IndirectScaledIndexed(RAX, RCX, Two, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(8)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 29, 14, 30, 28, 72, 8], OperandSize::Qword)
}

#[test]
fn vpcmpud_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUD, operand1: Some(Direct(K4)), operand2: Some(Direct(XMM2)), operand3: Some(Indirect(RCX, Some(OperandSize::Dword), None)), operand4: Some(Literal8(82)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 109, 25, 30, 33, 82], OperandSize::Qword)
}

#[test]
fn vpcmpud_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUD, operand1: Some(Direct(K5)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM0)), operand4: Some(Literal8(54)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 109, 43, 30, 232, 54], OperandSize::Dword)
}

#[test]
fn vpcmpud_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUD, operand1: Some(Direct(K4)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectDisplaced(EDX, 1106080978, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(82)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 117, 45, 30, 162, 210, 116, 237, 65, 82], OperandSize::Dword)
}

#[test]
fn vpcmpud_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUD, operand1: Some(Direct(K6)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexed(ECX, ECX, Eight, Some(OperandSize::Dword), None)), operand4: Some(Literal8(31)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 117, 59, 30, 52, 201, 31], OperandSize::Dword)
}

#[test]
fn vpcmpud_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUD, operand1: Some(Direct(K7)), operand2: Some(Direct(YMM10)), operand3: Some(Direct(YMM1)), operand4: Some(Literal8(100)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 45, 42, 30, 249, 100], OperandSize::Qword)
}

#[test]
fn vpcmpud_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUD, operand1: Some(Direct(K6)), operand2: Some(Direct(YMM17)), operand3: Some(IndirectScaledDisplaced(RDI, Eight, 1075851519, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(40)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 117, 36, 30, 52, 253, 255, 48, 32, 64, 40], OperandSize::Qword)
}

#[test]
fn vpcmpud_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUD, operand1: Some(Direct(K2)), operand2: Some(Direct(YMM27)), operand3: Some(IndirectScaledIndexed(RAX, RDI, Two, Some(OperandSize::Dword), None)), operand4: Some(Literal8(60)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 37, 54, 30, 20, 120, 60], OperandSize::Qword)
}

#[test]
fn vpcmpud_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUD, operand1: Some(Direct(K4)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM2)), operand4: Some(Literal8(84)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 69, 77, 30, 226, 84], OperandSize::Dword)
}

#[test]
fn vpcmpud_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUD, operand1: Some(Direct(K4)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EDI, Eight, 293972528, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(78)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 109, 77, 30, 164, 248, 48, 170, 133, 17, 78], OperandSize::Dword)
}

#[test]
fn vpcmpud_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUD, operand1: Some(Direct(K4)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledIndexed(ECX, EDX, Four, Some(OperandSize::Dword), None)), operand4: Some(Literal8(37)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 243, 101, 95, 30, 36, 145, 37], OperandSize::Dword)
}

#[test]
fn vpcmpud_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUD, operand1: Some(Direct(K3)), operand2: Some(Direct(ZMM13)), operand3: Some(Direct(ZMM0)), operand4: Some(Literal8(4)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 21, 79, 30, 216, 4], OperandSize::Qword)
}

#[test]
fn vpcmpud_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUD, operand1: Some(Direct(K5)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RDI, Four, 107613716, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(56)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 77, 76, 30, 172, 185, 20, 14, 106, 6, 56], OperandSize::Qword)
}

#[test]
fn vpcmpud_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUD, operand1: Some(Direct(K2)), operand2: Some(Direct(ZMM0)), operand3: Some(Indirect(RCX, Some(OperandSize::Dword), None)), operand4: Some(Literal8(70)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 243, 125, 89, 30, 17, 70], OperandSize::Qword)
}

