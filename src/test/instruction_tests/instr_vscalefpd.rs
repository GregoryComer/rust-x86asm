use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vscalefpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 229, 142, 44, 253], OperandSize::Dword)
}

#[test]
fn vscalefpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, ECX, Eight, 1700656963, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 213, 141, 44, 156, 207, 67, 247, 93, 101], OperandSize::Dword)
}

#[test]
fn vscalefpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, EDI, Eight, 891772488, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 253, 155, 44, 180, 249, 72, 94, 39, 53], OperandSize::Dword)
}

#[test]
fn vscalefpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPD, operand1: Some(Direct(XMM10)), operand2: Some(Direct(XMM14)), operand3: Some(Direct(XMM23)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 50, 141, 143, 44, 215], OperandSize::Qword)
}

#[test]
fn vscalefpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM13)), operand3: Some(IndirectDisplaced(RDX, 1650888037, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 149, 142, 44, 178, 101, 141, 102, 98], OperandSize::Qword)
}

#[test]
fn vscalefpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM20)), operand3: Some(IndirectDisplaced(RAX, 1958106069, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 221, 145, 44, 168, 213, 83, 182, 116], OperandSize::Qword)
}

#[test]
fn vscalefpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 229, 171, 44, 196], OperandSize::Dword)
}

#[test]
fn vscalefpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledDisplaced(ECX, Eight, 301467927, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 205, 175, 44, 44, 205, 23, 9, 248, 17], OperandSize::Dword)
}

#[test]
fn vscalefpd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectDisplaced(ECX, 55943304, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 213, 191, 44, 137, 136, 160, 85, 3], OperandSize::Dword)
}

#[test]
fn vscalefpd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM24)), operand3: Some(Direct(YMM11)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 210, 189, 161, 44, 227], OperandSize::Qword)
}

#[test]
fn vscalefpd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPD, operand1: Some(Direct(YMM28)), operand2: Some(Direct(YMM24)), operand3: Some(IndirectScaledDisplaced(RDI, Four, 1666219486, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 98, 189, 164, 44, 36, 189, 222, 125, 80, 99], OperandSize::Qword)
}

#[test]
fn vscalefpd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPD, operand1: Some(Direct(YMM17)), operand2: Some(Direct(YMM26)), operand3: Some(IndirectDisplaced(RCX, 843282907, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 226, 173, 178, 44, 137, 219, 121, 67, 50], OperandSize::Qword)
}

#[test]
fn vscalefpd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM3)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 229, 223, 44, 251], OperandSize::Dword)
}

#[test]
fn vscalefpd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledDisplaced(EAX, Eight, 518277540, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 197, 203, 44, 60, 197, 164, 73, 228, 30], OperandSize::Dword)
}

#[test]
fn vscalefpd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledDisplaced(ECX, Two, 891378662, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 221, 219, 44, 4, 77, 230, 91, 33, 53], OperandSize::Dword)
}

#[test]
fn vscalefpd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPD, operand1: Some(Direct(ZMM23)), operand2: Some(Direct(ZMM18)), operand3: Some(Direct(ZMM5)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 226, 237, 180, 44, 253], OperandSize::Qword)
}

#[test]
fn vscalefpd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM21)), operand3: Some(IndirectScaledDisplaced(RDI, Eight, 939203028, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 213, 195, 44, 4, 253, 212, 25, 251, 55], OperandSize::Qword)
}

#[test]
fn vscalefpd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPD, operand1: Some(Direct(ZMM27)), operand2: Some(Direct(ZMM13)), operand3: Some(Indirect(RSI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 98, 149, 222, 44, 30], OperandSize::Qword)
}

