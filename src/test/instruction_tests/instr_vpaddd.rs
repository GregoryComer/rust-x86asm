use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpaddd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 254, 232], OperandSize::Dword)
}

#[test]
fn vpaddd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EAX, Four, 1600881176, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 254, 140, 135, 24, 130, 107, 95], OperandSize::Dword)
}

#[test]
fn vpaddd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 254, 208], OperandSize::Qword)
}

#[test]
fn vpaddd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexed(RBX, RCX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 254, 44, 139], OperandSize::Qword)
}

#[test]
fn vpaddd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 254, 228], OperandSize::Dword)
}

#[test]
fn vpaddd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledDisplaced(EBX, Eight, 2106619740, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 254, 4, 221, 92, 119, 144, 125], OperandSize::Dword)
}

#[test]
fn vpaddd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 254, 230], OperandSize::Qword)
}

#[test]
fn vpaddd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexed(RAX, RBX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 254, 52, 216], OperandSize::Qword)
}

#[test]
fn vpaddd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 69, 140, 254, 205], OperandSize::Dword)
}

#[test]
fn vpaddd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EDI, Two, 105408249, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 109, 138, 254, 164, 127, 249, 102, 72, 6], OperandSize::Dword)
}

#[test]
fn vpaddd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexed(EAX, EAX, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 125, 158, 254, 28, 192], OperandSize::Dword)
}

#[test]
fn vpaddd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDD, operand1: Some(Direct(XMM23)), operand2: Some(Direct(XMM28)), operand3: Some(Direct(XMM17)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 161, 29, 130, 254, 249], OperandSize::Qword)
}

#[test]
fn vpaddd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDD, operand1: Some(Direct(XMM27)), operand2: Some(Direct(XMM29)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RSI, Two, 1548216430, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 97, 21, 130, 254, 156, 115, 110, 232, 71, 92], OperandSize::Qword)
}

#[test]
fn vpaddd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDD, operand1: Some(Direct(XMM11)), operand2: Some(Direct(XMM26)), operand3: Some(IndirectDisplaced(RDI, 28397968, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 113, 45, 148, 254, 159, 144, 81, 177, 1], OperandSize::Qword)
}

