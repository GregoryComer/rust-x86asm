use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfpclasspd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPD, operand1: Some(Direct(K5)), operand2: Some(Direct(XMM2)), operand3: Some(Literal8(113)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 253, 12, 102, 234, 113], OperandSize::Dword)
}

#[test]
fn vfpclasspd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPD, operand1: Some(Direct(K5)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, ESI, Eight, 1014619343, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(104)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 253, 12, 102, 172, 240, 207, 220, 121, 60, 104], OperandSize::Dword)
}

#[test]
fn vfpclasspd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPD, operand1: Some(Direct(K7)), operand2: Some(IndirectScaledDisplaced(EAX, Four, 735365462, Some(OperandSize::Qword), None)), operand3: Some(Literal8(86)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 243, 253, 30, 102, 60, 133, 86, 201, 212, 43, 86], OperandSize::Dword)
}

#[test]
fn vfpclasspd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPD, operand1: Some(Direct(K7)), operand2: Some(Direct(XMM19)), operand3: Some(Literal8(70)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 179, 253, 10, 102, 251, 70], OperandSize::Qword)
}

#[test]
fn vfpclasspd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPD, operand1: Some(Direct(K4)), operand2: Some(IndirectScaledIndexed(RDX, RBX, Eight, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(57)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 253, 9, 102, 36, 218, 57], OperandSize::Qword)
}

#[test]
fn vfpclasspd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPD, operand1: Some(Direct(K5)), operand2: Some(Indirect(RSI, Some(OperandSize::Qword), None)), operand3: Some(Literal8(100)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 243, 253, 29, 102, 46, 100], OperandSize::Qword)
}

#[test]
fn vfpclasspd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPD, operand1: Some(Direct(K2)), operand2: Some(Direct(YMM6)), operand3: Some(Literal8(12)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 253, 42, 102, 214, 12], OperandSize::Dword)
}

#[test]
fn vfpclasspd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPD, operand1: Some(Direct(K1)), operand2: Some(IndirectScaledIndexed(EDI, EDI, Eight, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(72)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 253, 44, 102, 12, 255, 72], OperandSize::Dword)
}

#[test]
fn vfpclasspd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPD, operand1: Some(Direct(K7)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, EDX, Eight, 1480775970, Some(OperandSize::Qword), None)), operand3: Some(Literal8(36)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 253, 58, 102, 188, 211, 34, 217, 66, 88, 36], OperandSize::Dword)
}

#[test]
fn vfpclasspd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPD, operand1: Some(Direct(K2)), operand2: Some(Direct(YMM7)), operand3: Some(Literal8(118)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 253, 44, 102, 215, 118], OperandSize::Qword)
}

#[test]
fn vfpclasspd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPD, operand1: Some(Direct(K5)), operand2: Some(IndirectScaledIndexed(RDI, RAX, Eight, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(87)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 253, 46, 102, 44, 199, 87], OperandSize::Qword)
}

#[test]
fn vfpclasspd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPD, operand1: Some(Direct(K5)), operand2: Some(IndirectScaledDisplaced(RCX, Eight, 2119233807, Some(OperandSize::Qword), None)), operand3: Some(Literal8(109)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 253, 57, 102, 44, 205, 15, 241, 80, 126, 109], OperandSize::Qword)
}

#[test]
fn vfpclasspd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPD, operand1: Some(Direct(K3)), operand2: Some(Direct(ZMM2)), operand3: Some(Literal8(115)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 253, 75, 102, 218, 115], OperandSize::Dword)
}

#[test]
fn vfpclasspd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPD, operand1: Some(Direct(K3)), operand2: Some(IndirectScaledIndexed(EAX, EDI, Four, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(53)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 253, 76, 102, 28, 184, 53], OperandSize::Dword)
}

#[test]
fn vfpclasspd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPD, operand1: Some(Direct(K6)), operand2: Some(IndirectDisplaced(EAX, 1247280911, Some(OperandSize::Qword), None)), operand3: Some(Literal8(90)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 253, 90, 102, 176, 15, 255, 87, 74, 90], OperandSize::Dword)
}

#[test]
fn vfpclasspd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPD, operand1: Some(Direct(K4)), operand2: Some(Direct(ZMM13)), operand3: Some(Literal8(3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 211, 253, 73, 102, 229, 3], OperandSize::Qword)
}

#[test]
fn vfpclasspd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPD, operand1: Some(Direct(K4)), operand2: Some(IndirectDisplaced(RCX, 1060758168, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(55)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 253, 77, 102, 161, 152, 226, 57, 63, 55], OperandSize::Qword)
}

#[test]
fn vfpclasspd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPD, operand1: Some(Direct(K5)), operand2: Some(IndirectScaledDisplaced(RAX, Two, 1237295927, Some(OperandSize::Qword), None)), operand3: Some(Literal8(38)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 253, 93, 102, 44, 69, 55, 163, 191, 73, 38], OperandSize::Qword)
}

