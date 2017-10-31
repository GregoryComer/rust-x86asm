use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vptestnmq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMQ, operand1: Some(Direct(K6)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 246, 13, 39, 243], OperandSize::Dword)
}

#[test]
fn vptestnmq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMQ, operand1: Some(Direct(K1)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexed(ESI, EDI, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 214, 14, 39, 12, 254], OperandSize::Dword)
}

#[test]
fn vptestnmq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMQ, operand1: Some(Direct(K6)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectDisplaced(ESI, 856838500, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 214, 31, 39, 182, 100, 81, 18, 51], OperandSize::Dword)
}

#[test]
fn vptestnmq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMQ, operand1: Some(Direct(K7)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 214, 10, 39, 252], OperandSize::Qword)
}

#[test]
fn vptestnmq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMQ, operand1: Some(Direct(K6)), operand2: Some(Direct(XMM14)), operand3: Some(Indirect(RAX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 142, 9, 39, 48], OperandSize::Qword)
}

#[test]
fn vptestnmq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMQ, operand1: Some(Direct(K1)), operand2: Some(Direct(XMM23)), operand3: Some(Indirect(RCX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 198, 20, 39, 9], OperandSize::Qword)
}

#[test]
fn vptestnmq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMQ, operand1: Some(Direct(K1)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 222, 47, 39, 204], OperandSize::Dword)
}

#[test]
fn vptestnmq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMQ, operand1: Some(Direct(K6)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectDisplaced(EAX, 966475869, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 230, 44, 39, 176, 93, 64, 155, 57], OperandSize::Dword)
}

#[test]
fn vptestnmq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMQ, operand1: Some(Direct(K6)), operand2: Some(Direct(YMM0)), operand3: Some(Indirect(ECX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 254, 60, 39, 49], OperandSize::Dword)
}

#[test]
fn vptestnmq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMQ, operand1: Some(Direct(K3)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM13)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 210, 246, 44, 39, 221], OperandSize::Qword)
}

#[test]
fn vptestnmq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMQ, operand1: Some(Direct(K7)), operand2: Some(Direct(YMM21)), operand3: Some(IndirectScaledDisplaced(RSI, Eight, 1982494388, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 214, 36, 39, 60, 245, 180, 118, 42, 118], OperandSize::Qword)
}

#[test]
fn vptestnmq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMQ, operand1: Some(Direct(K4)), operand2: Some(Direct(YMM9)), operand3: Some(IndirectScaledIndexed(RCX, RAX, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 182, 62, 39, 36, 129], OperandSize::Qword)
}

#[test]
fn vptestnmq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMQ, operand1: Some(Direct(K4)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 214, 76, 39, 226], OperandSize::Dword)
}

#[test]
fn vptestnmq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMQ, operand1: Some(Direct(K2)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, ESI, Eight, 75219502, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 214, 73, 39, 148, 243, 46, 194, 123, 4], OperandSize::Dword)
}

#[test]
fn vptestnmq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMQ, operand1: Some(Direct(K6)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledIndexed(EBX, ECX, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 238, 92, 39, 52, 75], OperandSize::Dword)
}

#[test]
fn vptestnmq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMQ, operand1: Some(Direct(K7)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM27)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 146, 214, 74, 39, 251], OperandSize::Qword)
}

#[test]
fn vptestnmq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMQ, operand1: Some(Direct(K3)), operand2: Some(Direct(ZMM19)), operand3: Some(IndirectScaledDisplaced(RSI, Eight, 665318156, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 230, 69, 39, 28, 245, 12, 243, 167, 39], OperandSize::Qword)
}

#[test]
fn vptestnmq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMQ, operand1: Some(Direct(K4)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledDisplaced(RBX, Eight, 2028596764, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 246, 91, 39, 36, 221, 28, 238, 233, 120], OperandSize::Qword)
}

