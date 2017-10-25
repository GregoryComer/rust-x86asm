use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vporq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 205, 140, 235, 240], OperandSize::Dword)
}

#[test]
fn vporq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: Some(Indirect(EDI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 253, 142, 235, 15], OperandSize::Dword)
}

#[test]
fn vporq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexed(EBX, EDI, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 245, 156, 235, 60, 251], OperandSize::Dword)
}

#[test]
fn vporq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORQ, operand1: Some(Direct(XMM9)), operand2: Some(Direct(XMM10)), operand3: Some(Direct(XMM10)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 81, 173, 143, 235, 202], OperandSize::Qword)
}

#[test]
fn vporq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORQ, operand1: Some(Direct(XMM19)), operand2: Some(Direct(XMM10)), operand3: Some(IndirectDisplaced(RDI, 1158663663, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 225, 173, 139, 235, 159, 239, 205, 15, 69], OperandSize::Qword)
}

#[test]
fn vporq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORQ, operand1: Some(Direct(XMM9)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectDisplaced(RDI, 2051323213, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 113, 221, 155, 235, 143, 77, 181, 68, 122], OperandSize::Qword)
}

#[test]
fn vporq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORQ, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 205, 175, 235, 199], OperandSize::Dword)
}

#[test]
fn vporq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORQ, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM7)), operand3: Some(Indirect(EBX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 197, 172, 235, 3], OperandSize::Dword)
}

#[test]
fn vporq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORQ, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM7)), operand3: Some(Indirect(EDX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 197, 187, 235, 10], OperandSize::Dword)
}

#[test]
fn vporq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORQ, operand1: Some(Direct(YMM19)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM8)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 193, 197, 172, 235, 216], OperandSize::Qword)
}

#[test]
fn vporq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORQ, operand1: Some(Direct(YMM26)), operand2: Some(Direct(YMM21)), operand3: Some(Indirect(RSI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 97, 213, 163, 235, 22], OperandSize::Qword)
}

#[test]
fn vporq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORQ, operand1: Some(Direct(YMM14)), operand2: Some(Direct(YMM30)), operand3: Some(Indirect(RBX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 113, 141, 183, 235, 51], OperandSize::Qword)
}

#[test]
fn vporq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORQ, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 253, 205, 235, 241], OperandSize::Dword)
}

#[test]
fn vporq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORQ, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM5)), operand3: Some(Indirect(EDI, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 213, 204, 235, 55], OperandSize::Dword)
}

#[test]
fn vporq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORQ, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledDisplaced(EDX, Four, 170224234, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 229, 222, 235, 28, 149, 106, 106, 37, 10], OperandSize::Dword)
}

#[test]
fn vporq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORQ, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM17)), operand3: Some(Direct(ZMM23)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 177, 245, 194, 235, 215], OperandSize::Qword)
}

#[test]
fn vporq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORQ, operand1: Some(Direct(ZMM27)), operand2: Some(Direct(ZMM31)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RBX, Eight, 891250630, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 97, 133, 198, 235, 156, 222, 198, 103, 31, 53], OperandSize::Qword)
}

#[test]
fn vporq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORQ, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM0)), operand3: Some(Indirect(RDX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 253, 223, 235, 10], OperandSize::Qword)
}

