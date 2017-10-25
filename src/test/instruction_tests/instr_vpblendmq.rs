use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpblendmq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 221, 140, 100, 224], OperandSize::Dword)
}

#[test]
fn vpblendmq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectDisplaced(EAX, 17134557, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 253, 139, 100, 144, 221, 115, 5, 1], OperandSize::Dword)
}

#[test]
fn vpblendmq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledDisplaced(EDI, Eight, 1576565746, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 221, 155, 100, 28, 253, 242, 123, 248, 93], OperandSize::Dword)
}

#[test]
fn vpblendmq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM16)), operand3: Some(Direct(XMM31)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 146, 253, 134, 100, 215], OperandSize::Qword)
}

#[test]
fn vpblendmq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMQ, operand1: Some(Direct(XMM11)), operand2: Some(Direct(XMM18)), operand3: Some(Indirect(RDX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 114, 237, 129, 100, 26], OperandSize::Qword)
}

#[test]
fn vpblendmq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMQ, operand1: Some(Direct(XMM14)), operand2: Some(Direct(XMM21)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RCX, Four, 805365095, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 114, 213, 151, 100, 180, 143, 103, 229, 0, 48], OperandSize::Qword)
}

#[test]
fn vpblendmq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMQ, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 197, 172, 100, 193], OperandSize::Dword)
}

#[test]
fn vpblendmq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMQ, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledDisplaced(EBX, Eight, 1086453447, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 229, 175, 100, 60, 221, 199, 246, 193, 64], OperandSize::Dword)
}

#[test]
fn vpblendmq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMQ, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledDisplaced(ESI, Eight, 1231555219, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 237, 189, 100, 20, 245, 147, 10, 104, 73], OperandSize::Dword)
}

#[test]
fn vpblendmq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMQ, operand1: Some(Direct(YMM31)), operand2: Some(Direct(YMM28)), operand3: Some(Direct(YMM19)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 34, 157, 164, 100, 251], OperandSize::Qword)
}

#[test]
fn vpblendmq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMQ, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM25)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RSI, Four, 670335536, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 181, 161, 100, 132, 179, 48, 130, 244, 39], OperandSize::Qword)
}

#[test]
fn vpblendmq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMQ, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RCX, Eight, 675484482, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 245, 191, 100, 164, 200, 66, 19, 67, 40], OperandSize::Qword)
}

#[test]
fn vpblendmq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMQ, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(ZMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 237, 202, 100, 193], OperandSize::Dword)
}

#[test]
fn vpblendmq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMQ, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledIndexed(EBX, EDI, Two, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 221, 205, 100, 4, 123], OperandSize::Dword)
}

#[test]
fn vpblendmq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMQ, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectDisplaced(EAX, 1895270550, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 237, 222, 100, 168, 150, 136, 247, 112], OperandSize::Dword)
}

#[test]
fn vpblendmq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMQ, operand1: Some(Direct(ZMM11)), operand2: Some(Direct(ZMM18)), operand3: Some(Direct(ZMM16)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 50, 237, 195, 100, 216], OperandSize::Qword)
}

#[test]
fn vpblendmq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMQ, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM16)), operand3: Some(IndirectDisplaced(RBX, 545302238, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 253, 193, 100, 163, 222, 166, 128, 32], OperandSize::Qword)
}

#[test]
fn vpblendmq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMQ, operand1: Some(Direct(ZMM13)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledIndexed(RBX, RBX, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 114, 197, 219, 100, 44, 91], OperandSize::Qword)
}

