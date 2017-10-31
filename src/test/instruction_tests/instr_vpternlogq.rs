use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpternlogq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM0)), operand4: Some(Literal8(72)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 237, 140, 37, 232, 72], OperandSize::Dword)
}

#[test]
fn vpternlogq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexed(ESI, EBX, Eight, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(82)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 213, 141, 37, 52, 222, 82], OperandSize::Dword)
}

#[test]
fn vpternlogq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectDisplaced(EDI, 2147058087, Some(OperandSize::Qword), None)), operand4: Some(Literal8(117)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 243, 237, 154, 37, 191, 167, 129, 249, 127, 117], OperandSize::Dword)
}

#[test]
fn vpternlogq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM13)), operand3: Some(Direct(XMM8)), operand4: Some(Literal8(47)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 211, 149, 139, 37, 248, 47], OperandSize::Qword)
}

#[test]
fn vpternlogq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM30)), operand3: Some(IndirectScaledIndexed(RBX, RDX, Two, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(0)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 141, 129, 37, 44, 83, 0], OperandSize::Qword)
}

#[test]
fn vpternlogq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGQ, operand1: Some(Direct(XMM10)), operand2: Some(Direct(XMM18)), operand3: Some(Indirect(RBX, Some(OperandSize::Qword), None)), operand4: Some(Literal8(61)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 115, 237, 147, 37, 19, 61], OperandSize::Qword)
}

#[test]
fn vpternlogq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGQ, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM6)), operand4: Some(Literal8(89)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 205, 173, 37, 254, 89], OperandSize::Dword)
}

#[test]
fn vpternlogq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGQ, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexed(ESI, EDX, Eight, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(41)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 205, 173, 37, 28, 214, 41], OperandSize::Dword)
}

#[test]
fn vpternlogq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGQ, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EBX, Four, 1926727898, Some(OperandSize::Qword), None)), operand4: Some(Literal8(114)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 205, 191, 37, 188, 159, 218, 136, 215, 114, 114], OperandSize::Dword)
}

#[test]
fn vpternlogq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGQ, operand1: Some(Direct(YMM12)), operand2: Some(Direct(YMM27)), operand3: Some(Direct(YMM4)), operand4: Some(Literal8(42)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 115, 165, 163, 37, 228, 42], OperandSize::Qword)
}

#[test]
fn vpternlogq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGQ, operand1: Some(Direct(YMM28)), operand2: Some(Direct(YMM19)), operand3: Some(IndirectDisplaced(RDX, 1578710338, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(121)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 99, 229, 161, 37, 162, 66, 53, 25, 94, 121], OperandSize::Qword)
}

#[test]
fn vpternlogq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGQ, operand1: Some(Direct(YMM15)), operand2: Some(Direct(YMM28)), operand3: Some(IndirectDisplaced(RCX, 700867551, Some(OperandSize::Qword), None)), operand4: Some(Literal8(13)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 115, 157, 183, 37, 185, 223, 99, 198, 41, 13], OperandSize::Qword)
}

#[test]
fn vpternlogq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGQ, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM3)), operand4: Some(Literal8(33)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 205, 201, 37, 227, 33], OperandSize::Dword)
}

#[test]
fn vpternlogq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGQ, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EDI, Eight, 1824511231, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(83)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 245, 202, 37, 188, 250, 255, 212, 191, 108, 83], OperandSize::Dword)
}

#[test]
fn vpternlogq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGQ, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectDisplaced(EAX, 1869264079, Some(OperandSize::Qword), None)), operand4: Some(Literal8(87)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 197, 222, 37, 176, 207, 180, 106, 111, 87], OperandSize::Dword)
}

#[test]
fn vpternlogq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGQ, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM21)), operand3: Some(Direct(ZMM31)), operand4: Some(Literal8(119)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 147, 213, 195, 37, 215, 119], OperandSize::Qword)
}

#[test]
fn vpternlogq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGQ, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM20)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RAX, Four, 1191220776, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(108)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 221, 194, 37, 172, 131, 40, 150, 0, 71, 108], OperandSize::Qword)
}

#[test]
fn vpternlogq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGQ, operand1: Some(Direct(ZMM14)), operand2: Some(Direct(ZMM17)), operand3: Some(Indirect(RDI, Some(OperandSize::Qword), None)), operand4: Some(Literal8(91)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 115, 245, 215, 37, 55, 91], OperandSize::Qword)
}

