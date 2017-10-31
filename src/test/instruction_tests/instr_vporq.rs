use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vporq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 197, 137, 235, 213], OperandSize::Dword)
}

#[test]
fn vporq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: Some(Indirect(ECX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 253, 140, 235, 57], OperandSize::Dword)
}

#[test]
fn vporq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectDisplaced(ECX, 2015271411, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 205, 153, 235, 161, 243, 153, 30, 120], OperandSize::Dword)
}

#[test]
fn vporq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORQ, operand1: Some(Direct(XMM29)), operand2: Some(Direct(XMM30)), operand3: Some(Direct(XMM21)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 33, 141, 135, 235, 237], OperandSize::Qword)
}

#[test]
fn vporq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORQ, operand1: Some(Direct(XMM19)), operand2: Some(Direct(XMM22)), operand3: Some(IndirectScaledIndexed(RDI, RSI, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 225, 205, 135, 235, 28, 119], OperandSize::Qword)
}

#[test]
fn vporq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORQ, operand1: Some(Direct(XMM8)), operand2: Some(Direct(XMM22)), operand3: Some(IndirectScaledDisplaced(RSI, Eight, 1277053892, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 113, 205, 149, 235, 4, 245, 196, 75, 30, 76], OperandSize::Qword)
}

#[test]
fn vporq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORQ, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 197, 170, 235, 199], OperandSize::Dword)
}

#[test]
fn vporq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORQ, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexed(ECX, EDX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 213, 172, 235, 36, 209], OperandSize::Dword)
}

#[test]
fn vporq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORQ, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM2)), operand3: Some(Indirect(EAX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 237, 186, 235, 56], OperandSize::Dword)
}

#[test]
fn vporq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORQ, operand1: Some(Direct(YMM26)), operand2: Some(Direct(YMM12)), operand3: Some(Direct(YMM15)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 65, 157, 171, 235, 215], OperandSize::Qword)
}

#[test]
fn vporq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORQ, operand1: Some(Direct(YMM17)), operand2: Some(Direct(YMM26)), operand3: Some(IndirectScaledDisplaced(RAX, Eight, 1484140265, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 225, 173, 166, 235, 12, 197, 233, 46, 118, 88], OperandSize::Qword)
}

#[test]
fn vporq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORQ, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM29)), operand3: Some(IndirectScaledIndexed(RDI, RAX, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 149, 180, 235, 60, 199], OperandSize::Qword)
}

#[test]
fn vporq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORQ, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 221, 202, 235, 205], OperandSize::Dword)
}

#[test]
fn vporq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORQ, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledDisplaced(ECX, Two, 970154403, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 221, 206, 235, 60, 77, 163, 97, 211, 57], OperandSize::Dword)
}

#[test]
fn vporq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORQ, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectDisplaced(EAX, 936128653, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 229, 221, 235, 168, 141, 48, 204, 55], OperandSize::Dword)
}

#[test]
fn vporq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORQ, operand1: Some(Direct(ZMM19)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 225, 197, 202, 235, 218], OperandSize::Qword)
}

#[test]
fn vporq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORQ, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM16)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RDX, Eight, 1677391300, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 253, 196, 235, 156, 209, 196, 245, 250, 99], OperandSize::Qword)
}

#[test]
fn vporq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORQ, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM14)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RAX, Eight, 1616130478, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 141, 219, 235, 132, 198, 174, 49, 84, 96], OperandSize::Qword)
}

