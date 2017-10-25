use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpandnq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDNQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 213, 138, 223, 255], OperandSize::Dword)
}

#[test]
fn vpandnq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDNQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, EAX, Four, 1291974613, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 229, 140, 223, 164, 129, 213, 247, 1, 77], OperandSize::Dword)
}

#[test]
fn vpandnq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDNQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectDisplaced(EBX, 602870760, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 221, 159, 223, 131, 232, 19, 239, 35], OperandSize::Dword)
}

#[test]
fn vpandnq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDNQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM8)), operand3: Some(Direct(XMM31)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 145, 189, 140, 223, 215], OperandSize::Qword)
}

#[test]
fn vpandnq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDNQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(Indirect(RAX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 229, 141, 223, 56], OperandSize::Qword)
}

#[test]
fn vpandnq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDNQ, operand1: Some(Direct(XMM12)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledDisplaced(RBX, Eight, 2074895051, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 113, 245, 158, 223, 36, 221, 203, 98, 172, 123], OperandSize::Qword)
}

#[test]
fn vpandnq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDNQ, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 245, 174, 223, 246], OperandSize::Dword)
}

#[test]
fn vpandnq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDNQ, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EBX, Eight, 641291267, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 253, 174, 223, 148, 222, 3, 84, 57, 38], OperandSize::Dword)
}

#[test]
fn vpandnq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDNQ, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexed(EBX, EDX, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 213, 189, 223, 44, 147], OperandSize::Dword)
}

#[test]
fn vpandnq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDNQ, operand1: Some(Direct(YMM17)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM20)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 161, 229, 171, 223, 204], OperandSize::Qword)
}

#[test]
fn vpandnq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDNQ, operand1: Some(Direct(YMM23)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledDisplaced(RCX, Two, 721979313, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 225, 229, 169, 223, 60, 77, 177, 135, 8, 43], OperandSize::Qword)
}

#[test]
fn vpandnq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDNQ, operand1: Some(Direct(YMM29)), operand2: Some(Direct(YMM15)), operand3: Some(IndirectDisplaced(RAX, 1877789936, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 97, 133, 186, 223, 168, 240, 204, 236, 111], OperandSize::Qword)
}

#[test]
fn vpandnq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDNQ, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 221, 203, 223, 239], OperandSize::Dword)
}

#[test]
fn vpandnq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDNQ, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectDisplaced(EAX, 2044851009, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 197, 203, 223, 184, 65, 243, 225, 121], OperandSize::Dword)
}

#[test]
fn vpandnq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDNQ, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, ECX, Four, 479559786, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 205, 218, 223, 140, 139, 106, 128, 149, 28], OperandSize::Dword)
}

#[test]
fn vpandnq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDNQ, operand1: Some(Direct(ZMM19)), operand2: Some(Direct(ZMM17)), operand3: Some(Direct(ZMM11)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 193, 245, 194, 223, 219], OperandSize::Qword)
}

#[test]
fn vpandnq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDNQ, operand1: Some(Direct(ZMM29)), operand2: Some(Direct(ZMM14)), operand3: Some(Indirect(RBX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 97, 141, 205, 223, 43], OperandSize::Qword)
}

#[test]
fn vpandnq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDNQ, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM21)), operand3: Some(Indirect(RAX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 213, 212, 223, 0], OperandSize::Qword)
}

