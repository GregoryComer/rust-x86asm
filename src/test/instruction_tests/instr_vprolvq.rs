use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vprolvq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 229, 142, 21, 233], OperandSize::Dword)
}

#[test]
fn vprolvq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectDisplaced(EAX, 945482400, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 205, 141, 21, 128, 160, 234, 90, 56], OperandSize::Dword)
}

#[test]
fn vprolvq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexed(EBX, EDX, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 213, 155, 21, 36, 83], OperandSize::Dword)
}

#[test]
fn vprolvq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVQ, operand1: Some(Direct(XMM24)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM11)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 66, 213, 140, 21, 195], OperandSize::Qword)
}

#[test]
fn vprolvq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexed(RAX, RBX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 205, 141, 21, 28, 216], OperandSize::Qword)
}

#[test]
fn vprolvq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVQ, operand1: Some(Direct(XMM27)), operand2: Some(Direct(XMM19)), operand3: Some(Indirect(RAX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 98, 229, 150, 21, 24], OperandSize::Qword)
}

#[test]
fn vprolvq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVQ, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 221, 175, 21, 249], OperandSize::Dword)
}

#[test]
fn vprolvq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVQ, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM3)), operand3: Some(Indirect(EBX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 229, 172, 21, 43], OperandSize::Dword)
}

#[test]
fn vprolvq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVQ, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexed(ECX, EAX, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 205, 189, 21, 36, 65], OperandSize::Dword)
}

#[test]
fn vprolvq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVQ, operand1: Some(Direct(YMM25)), operand2: Some(Direct(YMM23)), operand3: Some(Direct(YMM11)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 66, 197, 161, 21, 203], OperandSize::Qword)
}

#[test]
fn vprolvq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVQ, operand1: Some(Direct(YMM15)), operand2: Some(Direct(YMM25)), operand3: Some(IndirectScaledIndexed(RCX, RAX, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 114, 181, 162, 21, 60, 129], OperandSize::Qword)
}

#[test]
fn vprolvq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVQ, operand1: Some(Direct(YMM31)), operand2: Some(Direct(YMM23)), operand3: Some(IndirectScaledDisplaced(RCX, Four, 1643834655, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 98, 197, 179, 21, 60, 141, 31, 237, 250, 97], OperandSize::Qword)
}

#[test]
fn vprolvq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVQ, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 229, 205, 21, 242], OperandSize::Dword)
}

#[test]
fn vprolvq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVQ, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EDI, Eight, 191538485, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 229, 205, 21, 148, 248, 53, 165, 106, 11], OperandSize::Dword)
}

#[test]
fn vprolvq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVQ, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectDisplaced(ESI, 77485103, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 253, 223, 21, 134, 47, 84, 158, 4], OperandSize::Dword)
}

#[test]
fn vprolvq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVQ, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM13)), operand3: Some(Direct(ZMM25)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 146, 149, 207, 21, 193], OperandSize::Qword)
}

#[test]
fn vprolvq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVQ, operand1: Some(Direct(ZMM12)), operand2: Some(Direct(ZMM25)), operand3: Some(Indirect(RCX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 114, 181, 199, 21, 33], OperandSize::Qword)
}

#[test]
fn vprolvq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVQ, operand1: Some(Direct(ZMM14)), operand2: Some(Direct(ZMM25)), operand3: Some(IndirectScaledDisplaced(RAX, Eight, 930547703, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 114, 181, 215, 21, 52, 197, 247, 7, 119, 55], OperandSize::Qword)
}

