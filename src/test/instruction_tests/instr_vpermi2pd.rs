use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpermi2pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 221, 137, 119, 219], OperandSize::Dword)
}

#[test]
fn vpermi2pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EBX, Two, 58246781, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 221, 139, 119, 180, 90, 125, 198, 120, 3], OperandSize::Dword)
}

#[test]
fn vpermi2pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexed(EDX, ECX, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 221, 157, 119, 44, 202], OperandSize::Dword)
}

#[test]
fn vpermi2pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM18)), operand3: Some(Direct(XMM9)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 210, 237, 134, 119, 233], OperandSize::Qword)
}

#[test]
fn vpermi2pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM18)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RCX, Eight, 1090151808, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 237, 129, 119, 172, 206, 128, 101, 250, 64], OperandSize::Qword)
}

#[test]
fn vpermi2pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM8)), operand3: Some(IndirectScaledIndexed(RBX, RBX, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 189, 155, 119, 28, 155], OperandSize::Qword)
}

#[test]
fn vpermi2pd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 237, 175, 119, 234], OperandSize::Dword)
}

#[test]
fn vpermi2pd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexed(EDI, EDX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 229, 175, 119, 60, 215], OperandSize::Dword)
}

#[test]
fn vpermi2pd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectDisplaced(EDX, 701786224, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 205, 190, 119, 162, 112, 104, 212, 41], OperandSize::Dword)
}

#[test]
fn vpermi2pd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM9)), operand3: Some(Direct(YMM26)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 146, 181, 170, 119, 194], OperandSize::Qword)
}

#[test]
fn vpermi2pd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexed(RBX, RCX, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 221, 171, 119, 44, 75], OperandSize::Qword)
}

#[test]
fn vpermi2pd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM23)), operand3: Some(IndirectDisplaced(RCX, 1685297705, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 197, 177, 119, 185, 41, 154, 115, 100], OperandSize::Qword)
}

#[test]
fn vpermi2pd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 253, 202, 119, 199], OperandSize::Dword)
}

#[test]
fn vpermi2pd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledIndexed(EDX, EBX, Eight, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 245, 202, 119, 12, 218], OperandSize::Dword)
}

#[test]
fn vpermi2pd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EDX, Four, 1641540231, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 197, 218, 119, 156, 146, 135, 234, 215, 97], OperandSize::Dword)
}

#[test]
fn vpermi2pd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM19)), operand3: Some(Direct(ZMM21)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 178, 229, 196, 119, 213], OperandSize::Qword)
}

#[test]
fn vpermi2pd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM20)), operand3: Some(Indirect(RSI, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 221, 198, 119, 22], OperandSize::Qword)
}

#[test]
fn vpermi2pd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM9)), operand3: Some(IndirectDisplaced(RDI, 1733533641, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 181, 219, 119, 159, 201, 159, 83, 103], OperandSize::Qword)
}

