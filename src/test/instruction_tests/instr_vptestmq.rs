use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vptestmq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMQ, operand1: Some(Direct(K3)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 253, 12, 39, 220], OperandSize::Dword)
}

#[test]
fn vptestmq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMQ, operand1: Some(Direct(K3)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(EAX, 10390974, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 245, 10, 39, 152, 190, 141, 158, 0], OperandSize::Dword)
}

#[test]
fn vptestmq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMQ, operand1: Some(Direct(K3)), operand2: Some(Direct(XMM6)), operand3: Some(Indirect(ESI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 205, 28, 39, 30], OperandSize::Dword)
}

#[test]
fn vptestmq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMQ, operand1: Some(Direct(K2)), operand2: Some(Direct(XMM24)), operand3: Some(Direct(XMM31)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 146, 189, 7, 39, 215], OperandSize::Qword)
}

#[test]
fn vptestmq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMQ, operand1: Some(Direct(K1)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RSI, Eight, 2028655367, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 221, 14, 39, 140, 247, 7, 211, 234, 120], OperandSize::Qword)
}

#[test]
fn vptestmq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMQ, operand1: Some(Direct(K4)), operand2: Some(Direct(XMM9)), operand3: Some(IndirectScaledDisplaced(RDI, Eight, 1342068752, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 181, 28, 39, 36, 253, 16, 88, 254, 79], OperandSize::Qword)
}

#[test]
fn vptestmq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMQ, operand1: Some(Direct(K1)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 237, 43, 39, 206], OperandSize::Dword)
}

#[test]
fn vptestmq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMQ, operand1: Some(Direct(K7)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EAX, Four, 287469054, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 245, 41, 39, 188, 128, 254, 109, 34, 17], OperandSize::Dword)
}

#[test]
fn vptestmq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMQ, operand1: Some(Direct(K6)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexed(EAX, EDI, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 237, 59, 39, 52, 184], OperandSize::Dword)
}

#[test]
fn vptestmq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMQ, operand1: Some(Direct(K5)), operand2: Some(Direct(YMM16)), operand3: Some(Direct(YMM26)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 146, 253, 38, 39, 234], OperandSize::Qword)
}

#[test]
fn vptestmq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMQ, operand1: Some(Direct(K4)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledDisplaced(RDX, Eight, 870766587, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 197, 46, 39, 36, 213, 251, 215, 230, 51], OperandSize::Qword)
}

#[test]
fn vptestmq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMQ, operand1: Some(Direct(K4)), operand2: Some(Direct(YMM6)), operand3: Some(Indirect(RSI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 205, 59, 39, 38], OperandSize::Qword)
}

#[test]
fn vptestmq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMQ, operand1: Some(Direct(K2)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(ZMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 237, 76, 39, 213], OperandSize::Dword)
}

#[test]
fn vptestmq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMQ, operand1: Some(Direct(K6)), operand2: Some(Direct(ZMM1)), operand3: Some(Indirect(EBX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 245, 75, 39, 51], OperandSize::Dword)
}

#[test]
fn vptestmq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMQ, operand1: Some(Direct(K3)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EDX, Four, 937431512, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 221, 94, 39, 156, 147, 216, 17, 224, 55], OperandSize::Dword)
}

#[test]
fn vptestmq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMQ, operand1: Some(Direct(K5)), operand2: Some(Direct(ZMM31)), operand3: Some(Direct(ZMM30)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 146, 133, 69, 39, 238], OperandSize::Qword)
}

#[test]
fn vptestmq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMQ, operand1: Some(Direct(K5)), operand2: Some(Direct(ZMM25)), operand3: Some(Indirect(RAX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 181, 65, 39, 40], OperandSize::Qword)
}

#[test]
fn vptestmq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMQ, operand1: Some(Direct(K5)), operand2: Some(Direct(ZMM29)), operand3: Some(Indirect(RSI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 149, 82, 39, 46], OperandSize::Qword)
}

