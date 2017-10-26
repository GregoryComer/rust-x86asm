use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpxorq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 213, 142, 239, 206], OperandSize::Dword)
}

#[test]
fn vpxorq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(EDX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 197, 137, 239, 18], OperandSize::Dword)
}

#[test]
fn vpxorq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexed(ESI, ESI, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 205, 156, 239, 4, 246], OperandSize::Dword)
}

#[test]
fn vpxorq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORQ, operand1: Some(Direct(XMM23)), operand2: Some(Direct(XMM11)), operand3: Some(Direct(XMM23)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 161, 165, 140, 239, 255], OperandSize::Qword)
}

#[test]
fn vpxorq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORQ, operand1: Some(Direct(XMM28)), operand2: Some(Direct(XMM11)), operand3: Some(IndirectScaledDisplaced(RBX, Eight, 1690534468, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 97, 165, 137, 239, 36, 221, 68, 130, 195, 100], OperandSize::Qword)
}

#[test]
fn vpxorq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORQ, operand1: Some(Direct(XMM14)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RDI, Four, 34404319, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 113, 245, 154, 239, 180, 184, 223, 247, 12, 2], OperandSize::Qword)
}

#[test]
fn vpxorq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORQ, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 253, 173, 239, 193], OperandSize::Dword)
}

#[test]
fn vpxorq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORQ, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectDisplaced(ESI, 1995148206, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 253, 172, 239, 150, 174, 139, 235, 118], OperandSize::Dword)
}

#[test]
fn vpxorq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORQ, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexed(ECX, EDI, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 221, 189, 239, 12, 121], OperandSize::Dword)
}

#[test]
fn vpxorq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORQ, operand1: Some(Direct(YMM31)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM10)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 65, 229, 174, 239, 250], OperandSize::Qword)
}

#[test]
fn vpxorq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORQ, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM22)), operand3: Some(IndirectDisplaced(RBX, 948105586, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 205, 163, 239, 155, 114, 241, 130, 56], OperandSize::Qword)
}

#[test]
fn vpxorq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORQ, operand1: Some(Direct(YMM16)), operand2: Some(Direct(YMM17)), operand3: Some(Indirect(RSI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 225, 245, 180, 239, 6], OperandSize::Qword)
}

#[test]
fn vpxorq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORQ, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM1)), operand3: Some(Direct(ZMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 245, 202, 239, 197], OperandSize::Dword)
}

#[test]
fn vpxorq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORQ, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledDisplaced(ESI, Two, 789960755, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 205, 202, 239, 12, 117, 51, 216, 21, 47], OperandSize::Dword)
}

#[test]
fn vpxorq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORQ, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledIndexed(EAX, EDI, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 205, 220, 239, 28, 120], OperandSize::Dword)
}

#[test]
fn vpxorq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORQ, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM16)), operand3: Some(Direct(ZMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 253, 193, 239, 235], OperandSize::Qword)
}

#[test]
fn vpxorq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORQ, operand1: Some(Direct(ZMM15)), operand2: Some(Direct(ZMM13)), operand3: Some(IndirectScaledDisplaced(RSI, Two, 1051338015, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 113, 149, 203, 239, 60, 117, 31, 37, 170, 62], OperandSize::Qword)
}

#[test]
fn vpxorq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORQ, operand1: Some(Direct(ZMM18)), operand2: Some(Direct(ZMM3)), operand3: Some(Indirect(RBX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 225, 229, 219, 239, 19], OperandSize::Qword)
}

