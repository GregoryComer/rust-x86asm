use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vptestnmq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMQ, operand1: Some(Direct(K3)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 214, 11, 39, 221], OperandSize::Dword)
}

#[test]
fn vptestnmq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMQ, operand1: Some(Direct(K7)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectDisplaced(EDX, 544958863, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 198, 14, 39, 186, 143, 105, 123, 32], OperandSize::Dword)
}

#[test]
fn vptestnmq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMQ, operand1: Some(Direct(K1)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectDisplaced(EDI, 765100362, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 222, 27, 39, 143, 74, 129, 154, 45], OperandSize::Dword)
}

#[test]
fn vptestnmq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMQ, operand1: Some(Direct(K3)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM8)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 210, 222, 9, 39, 216], OperandSize::Qword)
}

#[test]
fn vptestnmq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMQ, operand1: Some(Direct(K2)), operand2: Some(Direct(XMM26)), operand3: Some(IndirectScaledIndexed(RAX, RAX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 174, 5, 39, 20, 128], OperandSize::Qword)
}

#[test]
fn vptestnmq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMQ, operand1: Some(Direct(K6)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectDisplaced(RBX, 470828302, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 254, 27, 39, 179, 14, 69, 16, 28], OperandSize::Qword)
}

#[test]
fn vptestnmq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMQ, operand1: Some(Direct(K7)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 222, 47, 39, 250], OperandSize::Dword)
}

#[test]
fn vptestnmq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMQ, operand1: Some(Direct(K2)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledDisplaced(EDX, Eight, 1226901472, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 206, 42, 39, 20, 213, 224, 7, 33, 73], OperandSize::Dword)
}

#[test]
fn vptestnmq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMQ, operand1: Some(Direct(K1)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EAX, Four, 1761938955, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 222, 59, 39, 140, 135, 11, 14, 5, 105], OperandSize::Dword)
}

#[test]
fn vptestnmq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMQ, operand1: Some(Direct(K1)), operand2: Some(Direct(YMM13)), operand3: Some(Direct(YMM30)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 146, 150, 42, 39, 206], OperandSize::Qword)
}

#[test]
fn vptestnmq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMQ, operand1: Some(Direct(K5)), operand2: Some(Direct(YMM28)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RDX, Two, 1114665365, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 158, 37, 39, 172, 83, 149, 113, 112, 66], OperandSize::Qword)
}

#[test]
fn vptestnmq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMQ, operand1: Some(Direct(K5)), operand2: Some(Direct(YMM0)), operand3: Some(Indirect(RSI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 254, 59, 39, 46], OperandSize::Qword)
}

#[test]
fn vptestnmq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMQ, operand1: Some(Direct(K4)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 206, 74, 39, 225], OperandSize::Dword)
}

#[test]
fn vptestnmq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMQ, operand1: Some(Direct(K3)), operand2: Some(Direct(ZMM5)), operand3: Some(Indirect(EDX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 214, 77, 39, 26], OperandSize::Dword)
}

#[test]
fn vptestnmq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMQ, operand1: Some(Direct(K2)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectScaledIndexed(EBX, ESI, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 254, 95, 39, 20, 179], OperandSize::Dword)
}

#[test]
fn vptestnmq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMQ, operand1: Some(Direct(K5)), operand2: Some(Direct(ZMM20)), operand3: Some(Direct(ZMM12)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 210, 222, 69, 39, 236], OperandSize::Qword)
}

#[test]
fn vptestnmq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMQ, operand1: Some(Direct(K7)), operand2: Some(Direct(ZMM16)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RBX, Two, 1328539734, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 254, 71, 39, 188, 95, 86, 232, 47, 79], OperandSize::Qword)
}

#[test]
fn vptestnmq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMQ, operand1: Some(Direct(K5)), operand2: Some(Direct(ZMM11)), operand3: Some(IndirectScaledDisplaced(RDX, Four, 611847275, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 166, 95, 39, 44, 149, 107, 12, 120, 36], OperandSize::Qword)
}

