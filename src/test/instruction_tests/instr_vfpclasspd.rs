use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfpclasspd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPD, operand1: Some(Direct(K2)), operand2: Some(Direct(XMM1)), operand3: Some(Literal8(63)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 253, 12, 102, 209, 63], OperandSize::Dword)
}

#[test]
fn vfpclasspd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPD, operand1: Some(Direct(K6)), operand2: Some(IndirectDisplaced(EAX, 455407523, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(27)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 253, 14, 102, 176, 163, 247, 36, 27, 27], OperandSize::Dword)
}

#[test]
fn vfpclasspd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPD, operand1: Some(Direct(K3)), operand2: Some(Indirect(EDX, Some(OperandSize::Qword), None)), operand3: Some(Literal8(58)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 243, 253, 25, 102, 26, 58], OperandSize::Dword)
}

#[test]
fn vfpclasspd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPD, operand1: Some(Direct(K3)), operand2: Some(Direct(XMM5)), operand3: Some(Literal8(55)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 253, 14, 102, 221, 55], OperandSize::Qword)
}

#[test]
fn vfpclasspd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPD, operand1: Some(Direct(K2)), operand2: Some(IndirectScaledIndexed(RDX, RDI, Four, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(30)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 253, 13, 102, 20, 186, 30], OperandSize::Qword)
}

#[test]
fn vfpclasspd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPD, operand1: Some(Direct(K3)), operand2: Some(IndirectScaledDisplaced(RDI, Eight, 674703166, Some(OperandSize::Qword), None)), operand3: Some(Literal8(57)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 243, 253, 29, 102, 28, 253, 62, 39, 55, 40, 57], OperandSize::Qword)
}

#[test]
fn vfpclasspd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPD, operand1: Some(Direct(K2)), operand2: Some(Direct(YMM5)), operand3: Some(Literal8(56)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 253, 41, 102, 213, 56], OperandSize::Dword)
}

#[test]
fn vfpclasspd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPD, operand1: Some(Direct(K4)), operand2: Some(Indirect(ESI, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(116)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 253, 44, 102, 38, 116], OperandSize::Dword)
}

#[test]
fn vfpclasspd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPD, operand1: Some(Direct(K2)), operand2: Some(IndirectScaledIndexed(EDX, EAX, Eight, Some(OperandSize::Qword), None)), operand3: Some(Literal8(27)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 253, 63, 102, 20, 194, 27], OperandSize::Dword)
}

#[test]
fn vfpclasspd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPD, operand1: Some(Direct(K5)), operand2: Some(Direct(YMM23)), operand3: Some(Literal8(35)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 179, 253, 45, 102, 239, 35], OperandSize::Qword)
}

#[test]
fn vfpclasspd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPD, operand1: Some(Direct(K2)), operand2: Some(IndirectScaledIndexed(RSI, RDX, Four, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(107)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 253, 42, 102, 20, 150, 107], OperandSize::Qword)
}

#[test]
fn vfpclasspd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPD, operand1: Some(Direct(K4)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RCX, Two, 1073605011, Some(OperandSize::Qword), None)), operand3: Some(Literal8(51)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 253, 63, 102, 164, 79, 147, 233, 253, 63, 51], OperandSize::Qword)
}

#[test]
fn vfpclasspd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPD, operand1: Some(Direct(K6)), operand2: Some(Direct(ZMM6)), operand3: Some(Literal8(90)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 253, 73, 102, 246, 90], OperandSize::Dword)
}

#[test]
fn vfpclasspd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPD, operand1: Some(Direct(K3)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, EAX, Two, 1795299885, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(67)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 253, 78, 102, 156, 70, 45, 26, 2, 107, 67], OperandSize::Dword)
}

#[test]
fn vfpclasspd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPD, operand1: Some(Direct(K3)), operand2: Some(IndirectScaledIndexed(EAX, EDI, Four, Some(OperandSize::Qword), None)), operand3: Some(Literal8(54)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 253, 90, 102, 28, 184, 54], OperandSize::Dword)
}

#[test]
fn vfpclasspd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPD, operand1: Some(Direct(K3)), operand2: Some(Direct(ZMM16)), operand3: Some(Literal8(42)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 179, 253, 75, 102, 216, 42], OperandSize::Qword)
}

#[test]
fn vfpclasspd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPD, operand1: Some(Direct(K1)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RDI, Four, 851072497, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(109)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 253, 74, 102, 140, 191, 241, 85, 186, 50, 109], OperandSize::Qword)
}

#[test]
fn vfpclasspd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPD, operand1: Some(Direct(K1)), operand2: Some(IndirectDisplaced(RDI, 1220825387, Some(OperandSize::Qword), None)), operand3: Some(Literal8(18)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 253, 91, 102, 143, 43, 81, 196, 72, 18], OperandSize::Qword)
}

