use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpminsq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 221, 142, 57, 201], OperandSize::Dword)
}

#[test]
fn vpminsq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EDX, Two, 1360506412, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 237, 142, 57, 156, 83, 44, 174, 23, 81], OperandSize::Dword)
}

#[test]
fn vpminsq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: Some(Indirect(EDI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 213, 153, 57, 31], OperandSize::Dword)
}

#[test]
fn vpminsq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSQ, operand1: Some(Direct(XMM31)), operand2: Some(Direct(XMM12)), operand3: Some(Direct(XMM11)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 66, 157, 138, 57, 251], OperandSize::Qword)
}

#[test]
fn vpminsq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSQ, operand1: Some(Direct(XMM9)), operand2: Some(Direct(XMM15)), operand3: Some(IndirectScaledDisplaced(RBX, Eight, 729555549, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 114, 133, 137, 57, 12, 221, 93, 34, 124, 43], OperandSize::Qword)
}

#[test]
fn vpminsq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSQ, operand1: Some(Direct(XMM15)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RSI, Eight, 628645533, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 114, 221, 159, 57, 188, 246, 157, 94, 120, 37], OperandSize::Qword)
}

#[test]
fn vpminsq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSQ, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 213, 172, 57, 211], OperandSize::Dword)
}

#[test]
fn vpminsq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSQ, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledDisplaced(EDX, Two, 1810446332, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 213, 175, 57, 44, 85, 252, 55, 233, 107], OperandSize::Dword)
}

#[test]
fn vpminsq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSQ, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM0)), operand3: Some(Indirect(EBX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 253, 191, 57, 11], OperandSize::Dword)
}

#[test]
fn vpminsq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSQ, operand1: Some(Direct(YMM9)), operand2: Some(Direct(YMM31)), operand3: Some(Direct(YMM24)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 18, 133, 164, 57, 200], OperandSize::Qword)
}

#[test]
fn vpminsq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSQ, operand1: Some(Direct(YMM19)), operand2: Some(Direct(YMM17)), operand3: Some(IndirectScaledIndexed(RDX, RAX, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 226, 245, 167, 57, 28, 66], OperandSize::Qword)
}

#[test]
fn vpminsq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSQ, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM15)), operand3: Some(IndirectScaledIndexed(RSI, RAX, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 133, 190, 57, 20, 134], OperandSize::Qword)
}

#[test]
fn vpminsq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSQ, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 221, 207, 57, 255], OperandSize::Dword)
}

#[test]
fn vpminsq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSQ, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EBX, Four, 1270844571, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 245, 206, 57, 148, 155, 155, 140, 191, 75], OperandSize::Dword)
}

#[test]
fn vpminsq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSQ, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectDisplaced(EAX, 1141196010, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 253, 218, 57, 136, 234, 68, 5, 68], OperandSize::Dword)
}

#[test]
fn vpminsq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSQ, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM8)), operand3: Some(Direct(ZMM11)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 210, 189, 204, 57, 251], OperandSize::Qword)
}

#[test]
fn vpminsq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSQ, operand1: Some(Direct(ZMM27)), operand2: Some(Direct(ZMM17)), operand3: Some(Indirect(RAX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 98, 245, 195, 57, 24], OperandSize::Qword)
}

#[test]
fn vpminsq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSQ, operand1: Some(Direct(ZMM11)), operand2: Some(Direct(ZMM16)), operand3: Some(IndirectDisplaced(RCX, 2130585610, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 114, 253, 213, 57, 153, 10, 40, 254, 126], OperandSize::Qword)
}

