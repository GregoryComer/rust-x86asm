use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vptestmq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMQ, operand1: Some(Direct(K1)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 237, 15, 39, 204], OperandSize::Dword)
}

#[test]
fn vptestmq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMQ, operand1: Some(Direct(K4)), operand2: Some(Direct(XMM2)), operand3: Some(Indirect(EBX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 237, 11, 39, 35], OperandSize::Dword)
}

#[test]
fn vptestmq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMQ, operand1: Some(Direct(K3)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectDisplaced(ESI, 2111805138, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 205, 30, 39, 158, 210, 150, 223, 125], OperandSize::Dword)
}

#[test]
fn vptestmq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMQ, operand1: Some(Direct(K3)), operand2: Some(Direct(XMM8)), operand3: Some(Direct(XMM9)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 210, 189, 14, 39, 217], OperandSize::Qword)
}

#[test]
fn vptestmq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMQ, operand1: Some(Direct(K1)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RBX, Eight, 1740255753, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 237, 11, 39, 140, 222, 9, 50, 186, 103], OperandSize::Qword)
}

#[test]
fn vptestmq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMQ, operand1: Some(Direct(K6)), operand2: Some(Direct(XMM27)), operand3: Some(IndirectScaledIndexed(RCX, RDI, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 165, 17, 39, 52, 185], OperandSize::Qword)
}

#[test]
fn vptestmq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMQ, operand1: Some(Direct(K5)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 221, 44, 39, 239], OperandSize::Dword)
}

#[test]
fn vptestmq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMQ, operand1: Some(Direct(K3)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledDisplaced(EDX, Eight, 1844982343, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 213, 42, 39, 28, 213, 71, 50, 248, 109], OperandSize::Dword)
}

#[test]
fn vptestmq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMQ, operand1: Some(Direct(K6)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledDisplaced(EDX, Four, 1855632536, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 229, 59, 39, 52, 149, 152, 180, 154, 110], OperandSize::Dword)
}

#[test]
fn vptestmq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMQ, operand1: Some(Direct(K7)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM8)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 210, 205, 47, 39, 248], OperandSize::Qword)
}

#[test]
fn vptestmq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMQ, operand1: Some(Direct(K4)), operand2: Some(Direct(YMM29)), operand3: Some(IndirectScaledDisplaced(RDI, Two, 927298906, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 149, 34, 39, 36, 125, 90, 117, 69, 55], OperandSize::Qword)
}

#[test]
fn vptestmq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMQ, operand1: Some(Direct(K5)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledDisplaced(RCX, Two, 1706731134, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 205, 61, 39, 44, 77, 126, 166, 186, 101], OperandSize::Qword)
}

#[test]
fn vptestmq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMQ, operand1: Some(Direct(K2)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(ZMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 237, 79, 39, 208], OperandSize::Dword)
}

#[test]
fn vptestmq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMQ, operand1: Some(Direct(K1)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledIndexed(ESI, EBX, Eight, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 205, 78, 39, 12, 222], OperandSize::Dword)
}

#[test]
fn vptestmq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMQ, operand1: Some(Direct(K7)), operand2: Some(Direct(ZMM7)), operand3: Some(Indirect(EAX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 197, 90, 39, 56], OperandSize::Dword)
}

#[test]
fn vptestmq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMQ, operand1: Some(Direct(K7)), operand2: Some(Direct(ZMM18)), operand3: Some(Direct(ZMM12)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 210, 237, 71, 39, 252], OperandSize::Qword)
}

#[test]
fn vptestmq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMQ, operand1: Some(Direct(K7)), operand2: Some(Direct(ZMM27)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RDX, Four, 1474351606, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 165, 69, 39, 188, 147, 246, 209, 224, 87], OperandSize::Qword)
}

#[test]
fn vptestmq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMQ, operand1: Some(Direct(K2)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RBX, Eight, 911731140, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 245, 94, 39, 148, 218, 196, 233, 87, 54], OperandSize::Qword)
}

