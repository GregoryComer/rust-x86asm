use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpandnq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDNQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 197, 141, 223, 252], OperandSize::Dword)
}

#[test]
fn vpandnq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDNQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EDX, Four, 605312159, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 213, 137, 223, 172, 147, 159, 84, 20, 36], OperandSize::Dword)
}

#[test]
fn vpandnq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDNQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledDisplaced(ESI, Four, 798030616, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 205, 154, 223, 60, 181, 24, 251, 144, 47], OperandSize::Dword)
}

#[test]
fn vpandnq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDNQ, operand1: Some(Direct(XMM11)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 113, 229, 140, 223, 222], OperandSize::Qword)
}

#[test]
fn vpandnq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDNQ, operand1: Some(Direct(XMM16)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RDX, Eight, 1901849842, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 225, 213, 138, 223, 132, 208, 242, 236, 91, 113], OperandSize::Qword)
}

#[test]
fn vpandnq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDNQ, operand1: Some(Direct(XMM19)), operand2: Some(Direct(XMM29)), operand3: Some(IndirectScaledIndexed(RSI, RAX, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 225, 149, 150, 223, 28, 198], OperandSize::Qword)
}

#[test]
fn vpandnq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDNQ, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 197, 172, 223, 238], OperandSize::Dword)
}

#[test]
fn vpandnq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDNQ, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM6)), operand3: Some(Indirect(ECX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 205, 169, 223, 33], OperandSize::Dword)
}

#[test]
fn vpandnq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDNQ, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM7)), operand3: Some(Indirect(ESI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 197, 191, 223, 6], OperandSize::Dword)
}

#[test]
fn vpandnq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDNQ, operand1: Some(Direct(YMM17)), operand2: Some(Direct(YMM9)), operand3: Some(Direct(YMM11)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 193, 181, 172, 223, 203], OperandSize::Qword)
}

#[test]
fn vpandnq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDNQ, operand1: Some(Direct(YMM31)), operand2: Some(Direct(YMM11)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RAX, Two, 1603792003, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 97, 165, 171, 223, 188, 67, 131, 236, 151, 95], OperandSize::Qword)
}

#[test]
fn vpandnq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDNQ, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexed(RBX, RCX, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 221, 186, 223, 12, 139], OperandSize::Qword)
}

#[test]
fn vpandnq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDNQ, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 197, 204, 223, 238], OperandSize::Dword)
}

#[test]
fn vpandnq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDNQ, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectDisplaced(EBX, 98639556, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 213, 205, 223, 171, 196, 30, 225, 5], OperandSize::Dword)
}

#[test]
fn vpandnq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDNQ, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM1)), operand3: Some(Indirect(EDX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 245, 222, 223, 10], OperandSize::Dword)
}

#[test]
fn vpandnq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDNQ, operand1: Some(Direct(ZMM28)), operand2: Some(Direct(ZMM12)), operand3: Some(Direct(ZMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 97, 157, 203, 223, 227], OperandSize::Qword)
}

#[test]
fn vpandnq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDNQ, operand1: Some(Direct(ZMM21)), operand2: Some(Direct(ZMM15)), operand3: Some(IndirectScaledIndexed(RCX, RDI, Four, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 225, 133, 201, 223, 44, 185], OperandSize::Qword)
}

#[test]
fn vpandnq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDNQ, operand1: Some(Direct(ZMM26)), operand2: Some(Direct(ZMM31)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RBX, Four, 1092598754, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 97, 133, 213, 223, 148, 155, 226, 187, 31, 65], OperandSize::Qword)
}

