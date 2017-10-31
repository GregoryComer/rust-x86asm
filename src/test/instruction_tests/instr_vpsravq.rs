use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpsravq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 213, 138, 70, 239], OperandSize::Dword)
}

#[test]
fn vpsravq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexed(EBX, EBX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 245, 138, 70, 36, 91], OperandSize::Dword)
}

#[test]
fn vpsravq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EDX, Two, 142826454, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 197, 159, 70, 172, 80, 214, 91, 131, 8], OperandSize::Dword)
}

#[test]
fn vpsravq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM22)), operand3: Some(Direct(XMM20)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 178, 205, 130, 70, 196], OperandSize::Qword)
}

#[test]
fn vpsravq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVQ, operand1: Some(Direct(XMM23)), operand2: Some(Direct(XMM18)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RDI, Two, 1152087541, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 226, 237, 132, 70, 188, 123, 245, 117, 171, 68], OperandSize::Qword)
}

#[test]
fn vpsravq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVQ, operand1: Some(Direct(XMM21)), operand2: Some(Direct(XMM26)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RAX, Four, 1445526583, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 226, 173, 147, 70, 172, 128, 55, 252, 40, 86], OperandSize::Qword)
}

#[test]
fn vpsravq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVQ, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 237, 169, 70, 227], OperandSize::Dword)
}

#[test]
fn vpsravq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVQ, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectDisplaced(EDX, 1513644252, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 237, 175, 70, 170, 220, 96, 56, 90], OperandSize::Dword)
}

#[test]
fn vpsravq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVQ, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledDisplaced(EDI, Eight, 1103731492, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 229, 190, 70, 4, 253, 36, 155, 201, 65], OperandSize::Dword)
}

#[test]
fn vpsravq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVQ, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM19)), operand3: Some(Direct(YMM8)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 210, 229, 161, 70, 232], OperandSize::Qword)
}

#[test]
fn vpsravq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVQ, operand1: Some(Direct(YMM26)), operand2: Some(Direct(YMM5)), operand3: Some(Indirect(RAX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 98, 213, 170, 70, 16], OperandSize::Qword)
}

#[test]
fn vpsravq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVQ, operand1: Some(Direct(YMM11)), operand2: Some(Direct(YMM15)), operand3: Some(IndirectScaledDisplaced(RDX, Four, 107971838, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 114, 133, 187, 70, 28, 149, 254, 132, 111, 6], OperandSize::Qword)
}

#[test]
fn vpsravq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVQ, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 253, 202, 70, 215], OperandSize::Dword)
}

#[test]
fn vpsravq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVQ, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM4)), operand3: Some(Indirect(EBX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 221, 207, 70, 43], OperandSize::Dword)
}

#[test]
fn vpsravq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVQ, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledIndexed(EDI, ECX, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 237, 219, 70, 52, 79], OperandSize::Dword)
}

#[test]
fn vpsravq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVQ, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM17)), operand3: Some(Direct(ZMM22)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 178, 245, 198, 70, 206], OperandSize::Qword)
}

#[test]
fn vpsravq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVQ, operand1: Some(Direct(ZMM27)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledDisplaced(RAX, Two, 1106216252, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 98, 205, 206, 70, 28, 69, 60, 133, 239, 65], OperandSize::Qword)
}

#[test]
fn vpsravq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVQ, operand1: Some(Direct(ZMM15)), operand2: Some(Direct(ZMM19)), operand3: Some(IndirectScaledDisplaced(RBX, Four, 387301181, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 114, 229, 209, 70, 60, 157, 61, 191, 21, 23], OperandSize::Qword)
}

