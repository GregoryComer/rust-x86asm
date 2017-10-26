use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpblendmq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 229, 143, 100, 237], OperandSize::Dword)
}

#[test]
fn vpblendmq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectDisplaced(EBX, 881449763, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 205, 140, 100, 187, 35, 219, 137, 52], OperandSize::Dword)
}

#[test]
fn vpblendmq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, ECX, Eight, 1452751775, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 213, 156, 100, 172, 201, 159, 59, 151, 86], OperandSize::Dword)
}

#[test]
fn vpblendmq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMQ, operand1: Some(Direct(XMM27)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM27)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 2, 221, 142, 100, 219], OperandSize::Qword)
}

#[test]
fn vpblendmq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM27)), operand3: Some(Indirect(RCX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 165, 133, 100, 57], OperandSize::Qword)
}

#[test]
fn vpblendmq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMQ, operand1: Some(Direct(XMM19)), operand2: Some(Direct(XMM19)), operand3: Some(IndirectScaledIndexed(RCX, RAX, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 226, 229, 150, 100, 28, 65], OperandSize::Qword)
}

#[test]
fn vpblendmq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMQ, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 253, 172, 100, 207], OperandSize::Dword)
}

#[test]
fn vpblendmq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMQ, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexed(ECX, EAX, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 205, 175, 100, 12, 129], OperandSize::Dword)
}

#[test]
fn vpblendmq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMQ, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, ESI, Four, 1884943091, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 229, 185, 100, 132, 179, 243, 242, 89, 112], OperandSize::Dword)
}

#[test]
fn vpblendmq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMQ, operand1: Some(Direct(YMM8)), operand2: Some(Direct(YMM17)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 114, 245, 166, 100, 192], OperandSize::Qword)
}

#[test]
fn vpblendmq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMQ, operand1: Some(Direct(YMM13)), operand2: Some(Direct(YMM21)), operand3: Some(IndirectScaledIndexed(RBX, RDI, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 114, 213, 164, 100, 44, 123], OperandSize::Qword)
}

#[test]
fn vpblendmq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMQ, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM17)), operand3: Some(IndirectScaledIndexed(RCX, RDX, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 245, 179, 100, 28, 81], OperandSize::Qword)
}

#[test]
fn vpblendmq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMQ, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 229, 203, 100, 227], OperandSize::Dword)
}

#[test]
fn vpblendmq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMQ, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM0)), operand3: Some(Indirect(EDX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 253, 204, 100, 2], OperandSize::Dword)
}

#[test]
fn vpblendmq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMQ, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledIndexed(EDI, ECX, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 245, 221, 100, 4, 143], OperandSize::Dword)
}

#[test]
fn vpblendmq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMQ, operand1: Some(Direct(ZMM27)), operand2: Some(Direct(ZMM15)), operand3: Some(Direct(ZMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 98, 133, 201, 100, 221], OperandSize::Qword)
}

#[test]
fn vpblendmq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMQ, operand1: Some(Direct(ZMM9)), operand2: Some(Direct(ZMM23)), operand3: Some(IndirectScaledDisplaced(RDI, Four, 815906705, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 114, 197, 197, 100, 12, 189, 145, 191, 161, 48], OperandSize::Qword)
}

#[test]
fn vpblendmq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMQ, operand1: Some(Direct(ZMM13)), operand2: Some(Direct(ZMM22)), operand3: Some(Indirect(RDX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 114, 205, 210, 100, 42], OperandSize::Qword)
}

