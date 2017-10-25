use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vptestnmq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMQ, operand1: Some(Direct(K7)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 230, 12, 39, 255], OperandSize::Dword)
}

#[test]
fn vptestnmq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMQ, operand1: Some(Direct(K5)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(ECX, 55386429, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 246, 10, 39, 169, 61, 33, 77, 3], OperandSize::Dword)
}

#[test]
fn vptestnmq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMQ, operand1: Some(Direct(K2)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledDisplaced(ESI, Four, 1226621058, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 198, 31, 39, 20, 181, 130, 192, 28, 73], OperandSize::Dword)
}

#[test]
fn vptestnmq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMQ, operand1: Some(Direct(K2)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM24)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 146, 230, 11, 39, 208], OperandSize::Qword)
}

#[test]
fn vptestnmq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMQ, operand1: Some(Direct(K1)), operand2: Some(Direct(XMM12)), operand3: Some(Indirect(RDI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 158, 9, 39, 15], OperandSize::Qword)
}

#[test]
fn vptestnmq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMQ, operand1: Some(Direct(K7)), operand2: Some(Direct(XMM25)), operand3: Some(IndirectDisplaced(RDI, 1369537732, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 182, 19, 39, 191, 196, 124, 161, 81], OperandSize::Qword)
}

#[test]
fn vptestnmq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMQ, operand1: Some(Direct(K6)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 230, 45, 39, 245], OperandSize::Dword)
}

#[test]
fn vptestnmq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMQ, operand1: Some(Direct(K2)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexed(EDI, ECX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 214, 44, 39, 20, 207], OperandSize::Dword)
}

#[test]
fn vptestnmq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMQ, operand1: Some(Direct(K3)), operand2: Some(Direct(YMM3)), operand3: Some(Indirect(ESI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 230, 63, 39, 30], OperandSize::Dword)
}

#[test]
fn vptestnmq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMQ, operand1: Some(Direct(K2)), operand2: Some(Direct(YMM24)), operand3: Some(Direct(YMM20)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 178, 190, 39, 39, 212], OperandSize::Qword)
}

#[test]
fn vptestnmq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMQ, operand1: Some(Direct(K7)), operand2: Some(Direct(YMM19)), operand3: Some(Indirect(RCX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 230, 39, 39, 57], OperandSize::Qword)
}

#[test]
fn vptestnmq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMQ, operand1: Some(Direct(K5)), operand2: Some(Direct(YMM12)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RCX, Eight, 279558545, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 158, 57, 39, 172, 206, 145, 185, 169, 16], OperandSize::Qword)
}

#[test]
fn vptestnmq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMQ, operand1: Some(Direct(K7)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 222, 77, 39, 253], OperandSize::Dword)
}

#[test]
fn vptestnmq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMQ, operand1: Some(Direct(K7)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectDisplaced(EDX, 764005322, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 246, 78, 39, 186, 202, 203, 137, 45], OperandSize::Dword)
}

#[test]
fn vptestnmq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMQ, operand1: Some(Direct(K1)), operand2: Some(Direct(ZMM5)), operand3: Some(Indirect(ESI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 214, 92, 39, 14], OperandSize::Dword)
}

#[test]
fn vptestnmq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMQ, operand1: Some(Direct(K1)), operand2: Some(Direct(ZMM10)), operand3: Some(Direct(ZMM23)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 178, 174, 74, 39, 207], OperandSize::Qword)
}

#[test]
fn vptestnmq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMQ, operand1: Some(Direct(K7)), operand2: Some(Direct(ZMM9)), operand3: Some(Indirect(RSI, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 182, 74, 39, 62], OperandSize::Qword)
}

#[test]
fn vptestnmq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMQ, operand1: Some(Direct(K6)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledDisplaced(RDX, Eight, 499209185, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 198, 94, 39, 52, 213, 225, 83, 193, 29], OperandSize::Qword)
}

