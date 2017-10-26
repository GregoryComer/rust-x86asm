use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vptestmq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMQ, operand1: Some(Direct(K6)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 221, 12, 39, 245], OperandSize::Dword)
}

#[test]
fn vptestmq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMQ, operand1: Some(Direct(K2)), operand2: Some(Direct(XMM1)), operand3: Some(Indirect(EBX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 245, 11, 39, 19], OperandSize::Dword)
}

#[test]
fn vptestmq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMQ, operand1: Some(Direct(K6)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EBX, Eight, 1161494401, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 237, 26, 39, 180, 223, 129, 255, 58, 69], OperandSize::Dword)
}

#[test]
fn vptestmq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMQ, operand1: Some(Direct(K7)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 205, 15, 39, 255], OperandSize::Qword)
}

#[test]
fn vptestmq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMQ, operand1: Some(Direct(K1)), operand2: Some(Direct(XMM22)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RDX, Two, 1615237361, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 205, 5, 39, 140, 87, 241, 144, 70, 96], OperandSize::Qword)
}

#[test]
fn vptestmq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMQ, operand1: Some(Direct(K6)), operand2: Some(Direct(XMM13)), operand3: Some(IndirectScaledDisplaced(RCX, Two, 1676813269, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 149, 26, 39, 52, 77, 213, 35, 242, 99], OperandSize::Qword)
}

#[test]
fn vptestmq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMQ, operand1: Some(Direct(K5)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 229, 44, 39, 236], OperandSize::Dword)
}

#[test]
fn vptestmq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMQ, operand1: Some(Direct(K5)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexed(ECX, EDI, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 205, 42, 39, 44, 249], OperandSize::Dword)
}

#[test]
fn vptestmq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMQ, operand1: Some(Direct(K1)), operand2: Some(Direct(YMM1)), operand3: Some(Indirect(EDX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 245, 59, 39, 10], OperandSize::Dword)
}

#[test]
fn vptestmq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMQ, operand1: Some(Direct(K2)), operand2: Some(Direct(YMM20)), operand3: Some(Direct(YMM21)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 178, 221, 38, 39, 213], OperandSize::Qword)
}

#[test]
fn vptestmq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMQ, operand1: Some(Direct(K2)), operand2: Some(Direct(YMM31)), operand3: Some(Indirect(RDI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 133, 36, 39, 23], OperandSize::Qword)
}

#[test]
fn vptestmq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMQ, operand1: Some(Direct(K5)), operand2: Some(Direct(YMM31)), operand3: Some(IndirectScaledIndexed(RDI, RSI, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 133, 50, 39, 44, 247], OperandSize::Qword)
}

#[test]
fn vptestmq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMQ, operand1: Some(Direct(K4)), operand2: Some(Direct(ZMM1)), operand3: Some(Direct(ZMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 245, 77, 39, 226], OperandSize::Dword)
}

#[test]
fn vptestmq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMQ, operand1: Some(Direct(K1)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledIndexed(ECX, EBX, Four, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 245, 73, 39, 12, 153], OperandSize::Dword)
}

#[test]
fn vptestmq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMQ, operand1: Some(Direct(K7)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectDisplaced(EDX, 1849999853, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 237, 93, 39, 186, 237, 193, 68, 110], OperandSize::Dword)
}

#[test]
fn vptestmq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMQ, operand1: Some(Direct(K3)), operand2: Some(Direct(ZMM11)), operand3: Some(Direct(ZMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 165, 76, 39, 218], OperandSize::Qword)
}

#[test]
fn vptestmq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMQ, operand1: Some(Direct(K1)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledIndexed(RDI, RBX, Two, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 213, 75, 39, 12, 95], OperandSize::Qword)
}

#[test]
fn vptestmq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMQ, operand1: Some(Direct(K7)), operand2: Some(Direct(ZMM16)), operand3: Some(IndirectDisplaced(RAX, 2054976018, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 253, 85, 39, 184, 18, 114, 124, 122], OperandSize::Qword)
}

