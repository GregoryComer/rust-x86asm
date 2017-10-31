use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpminsq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 237, 140, 57, 220], OperandSize::Dword)
}

#[test]
fn vpminsq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledDisplaced(EAX, Eight, 900711003, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 205, 137, 57, 28, 197, 91, 194, 175, 53], OperandSize::Dword)
}

#[test]
fn vpminsq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectDisplaced(EBX, 658839250, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 253, 155, 57, 163, 210, 22, 69, 39], OperandSize::Dword)
}

#[test]
fn vpminsq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSQ, operand1: Some(Direct(XMM30)), operand2: Some(Direct(XMM31)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 98, 133, 133, 57, 244], OperandSize::Qword)
}

#[test]
fn vpminsq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSQ, operand1: Some(Direct(XMM9)), operand2: Some(Direct(XMM3)), operand3: Some(Indirect(RSI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 114, 229, 140, 57, 14], OperandSize::Qword)
}

#[test]
fn vpminsq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSQ, operand1: Some(Direct(XMM11)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexed(RBX, RBX, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 114, 213, 154, 57, 28, 155], OperandSize::Qword)
}

#[test]
fn vpminsq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSQ, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 253, 170, 57, 238], OperandSize::Dword)
}

#[test]
fn vpminsq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSQ, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledDisplaced(EDI, Four, 1358363962, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 205, 169, 57, 52, 189, 58, 253, 246, 80], OperandSize::Dword)
}

#[test]
fn vpminsq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSQ, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledDisplaced(EAX, Two, 956278855, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 253, 191, 57, 36, 69, 71, 168, 255, 56], OperandSize::Dword)
}

#[test]
fn vpminsq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSQ, operand1: Some(Direct(YMM9)), operand2: Some(Direct(YMM13)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 114, 149, 173, 57, 205], OperandSize::Qword)
}

#[test]
fn vpminsq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSQ, operand1: Some(Direct(YMM22)), operand2: Some(Direct(YMM29)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RCX, Eight, 310958699, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 226, 149, 167, 57, 180, 203, 107, 218, 136, 18], OperandSize::Qword)
}

#[test]
fn vpminsq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSQ, operand1: Some(Direct(YMM16)), operand2: Some(Direct(YMM13)), operand3: Some(IndirectScaledIndexed(RAX, RDX, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 226, 149, 189, 57, 4, 208], OperandSize::Qword)
}

#[test]
fn vpminsq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSQ, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM1)), operand3: Some(Direct(ZMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 245, 206, 57, 245], OperandSize::Dword)
}

#[test]
fn vpminsq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSQ, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EAX, Two, 1511427653, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 253, 205, 57, 140, 67, 69, 142, 22, 90], OperandSize::Dword)
}

#[test]
fn vpminsq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSQ, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EDI, Two, 1851272833, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 221, 223, 57, 132, 126, 129, 46, 88, 110], OperandSize::Dword)
}

#[test]
fn vpminsq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSQ, operand1: Some(Direct(ZMM27)), operand2: Some(Direct(ZMM29)), operand3: Some(Direct(ZMM20)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 34, 149, 198, 57, 220], OperandSize::Qword)
}

#[test]
fn vpminsq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSQ, operand1: Some(Direct(ZMM9)), operand2: Some(Direct(ZMM10)), operand3: Some(IndirectScaledIndexed(RSI, RSI, Four, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 114, 173, 203, 57, 12, 182], OperandSize::Qword)
}

#[test]
fn vpminsq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSQ, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM11)), operand3: Some(IndirectScaledDisplaced(RAX, Two, 175687933, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 165, 219, 57, 36, 69, 253, 200, 120, 10], OperandSize::Qword)
}

