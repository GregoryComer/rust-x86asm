use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpermi2pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 237, 140, 119, 212], OperandSize::Dword)
}

#[test]
fn vpermi2pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EAX, Two, 1327851258, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 213, 139, 119, 180, 66, 250, 102, 37, 79], OperandSize::Dword)
}

#[test]
fn vpermi2pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectDisplaced(EAX, 1071712969, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 229, 159, 119, 176, 201, 10, 225, 63], OperandSize::Dword)
}

#[test]
fn vpermi2pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PD, operand1: Some(Direct(XMM11)), operand2: Some(Direct(XMM9)), operand3: Some(Direct(XMM13)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 82, 181, 139, 119, 221], OperandSize::Qword)
}

#[test]
fn vpermi2pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PD, operand1: Some(Direct(XMM12)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexed(RDX, RBX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 114, 237, 140, 119, 36, 154], OperandSize::Qword)
}

#[test]
fn vpermi2pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PD, operand1: Some(Direct(XMM24)), operand2: Some(Direct(XMM17)), operand3: Some(IndirectScaledDisplaced(RDI, Four, 476340231, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 98, 245, 146, 119, 4, 189, 7, 96, 100, 28], OperandSize::Qword)
}

#[test]
fn vpermi2pd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 253, 171, 119, 228], OperandSize::Dword)
}

#[test]
fn vpermi2pd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledDisplaced(EAX, Two, 1015998193, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 221, 173, 119, 4, 69, 241, 230, 142, 60], OperandSize::Dword)
}

#[test]
fn vpermi2pd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledDisplaced(EDX, Eight, 954487023, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 221, 187, 119, 20, 213, 239, 80, 228, 56], OperandSize::Dword)
}

#[test]
fn vpermi2pd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM17)), operand3: Some(Direct(YMM28)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 146, 245, 165, 119, 252], OperandSize::Qword)
}

#[test]
fn vpermi2pd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM28)), operand3: Some(IndirectDisplaced(RAX, 1106394325, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 157, 165, 119, 128, 213, 60, 242, 65], OperandSize::Qword)
}

#[test]
fn vpermi2pd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PD, operand1: Some(Direct(YMM8)), operand2: Some(Direct(YMM15)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RCX, Four, 1134215286, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 114, 133, 185, 119, 132, 142, 118, 192, 154, 67], OperandSize::Qword)
}

#[test]
fn vpermi2pd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(ZMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 237, 204, 119, 243], OperandSize::Dword)
}

#[test]
fn vpermi2pd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, ECX, Four, 728510810, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 253, 201, 119, 164, 143, 90, 49, 108, 43], OperandSize::Dword)
}

#[test]
fn vpermi2pd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledIndexed(EDI, EDX, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 237, 223, 119, 28, 215], OperandSize::Dword)
}

#[test]
fn vpermi2pd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PD, operand1: Some(Direct(ZMM21)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM19)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 162, 229, 206, 119, 235], OperandSize::Qword)
}

#[test]
fn vpermi2pd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PD, operand1: Some(Direct(ZMM26)), operand2: Some(Direct(ZMM13)), operand3: Some(IndirectDisplaced(RAX, 1714732172, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 98, 149, 201, 119, 144, 140, 188, 52, 102], OperandSize::Qword)
}

#[test]
fn vpermi2pd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledIndexed(RBX, RBX, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 197, 221, 119, 12, 91], OperandSize::Qword)
}

