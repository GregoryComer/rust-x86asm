use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpblendmq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 229, 141, 100, 222], OperandSize::Dword)
}

#[test]
fn vpblendmq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectDisplaced(EDX, 1649866878, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 205, 143, 100, 130, 126, 248, 86, 98], OperandSize::Dword)
}

#[test]
fn vpblendmq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, EDX, Eight, 232813118, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 197, 155, 100, 164, 209, 62, 114, 224, 13], OperandSize::Dword)
}

#[test]
fn vpblendmq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMQ, operand1: Some(Direct(XMM16)), operand2: Some(Direct(XMM12)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 226, 157, 137, 100, 194], OperandSize::Qword)
}

#[test]
fn vpblendmq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMQ, operand1: Some(Direct(XMM23)), operand2: Some(Direct(XMM31)), operand3: Some(Indirect(RDX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 226, 133, 134, 100, 58], OperandSize::Qword)
}

#[test]
fn vpblendmq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMQ, operand1: Some(Direct(XMM14)), operand2: Some(Direct(XMM28)), operand3: Some(IndirectScaledIndexed(RAX, RSI, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 114, 157, 148, 100, 52, 240], OperandSize::Qword)
}

#[test]
fn vpblendmq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMQ, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 205, 170, 100, 209], OperandSize::Dword)
}

#[test]
fn vpblendmq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMQ, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM0)), operand3: Some(Indirect(EAX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 253, 171, 100, 8], OperandSize::Dword)
}

#[test]
fn vpblendmq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMQ, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM7)), operand3: Some(Indirect(EDI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 197, 188, 100, 63], OperandSize::Dword)
}

#[test]
fn vpblendmq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMQ, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM10)), operand3: Some(Direct(YMM15)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 210, 173, 170, 100, 207], OperandSize::Qword)
}

#[test]
fn vpblendmq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMQ, operand1: Some(Direct(YMM9)), operand2: Some(Direct(YMM18)), operand3: Some(IndirectScaledDisplaced(RDI, Two, 653180339, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 114, 237, 165, 100, 12, 125, 179, 189, 238, 38], OperandSize::Qword)
}

#[test]
fn vpblendmq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMQ, operand1: Some(Direct(YMM19)), operand2: Some(Direct(YMM19)), operand3: Some(IndirectScaledIndexed(RSI, RBX, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 226, 229, 182, 100, 28, 222], OperandSize::Qword)
}

#[test]
fn vpblendmq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMQ, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 205, 207, 100, 214], OperandSize::Dword)
}

#[test]
fn vpblendmq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMQ, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledIndexed(ECX, EDX, Eight, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 221, 202, 100, 28, 209], OperandSize::Dword)
}

#[test]
fn vpblendmq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMQ, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM1)), operand3: Some(Indirect(EDI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 245, 220, 100, 15], OperandSize::Dword)
}

#[test]
fn vpblendmq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMQ, operand1: Some(Direct(ZMM26)), operand2: Some(Direct(ZMM25)), operand3: Some(Direct(ZMM23)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 34, 181, 199, 100, 215], OperandSize::Qword)
}

#[test]
fn vpblendmq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMQ, operand1: Some(Direct(ZMM20)), operand2: Some(Direct(ZMM2)), operand3: Some(Indirect(RBX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 226, 237, 204, 100, 35], OperandSize::Qword)
}

#[test]
fn vpblendmq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMQ, operand1: Some(Direct(ZMM24)), operand2: Some(Direct(ZMM29)), operand3: Some(IndirectScaledIndexed(RSI, RBX, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 98, 149, 209, 100, 4, 222], OperandSize::Qword)
}

