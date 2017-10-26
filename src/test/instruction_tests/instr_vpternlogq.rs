use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpternlogq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM1)), operand4: Some(Literal8(35)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 253, 142, 37, 193, 35], OperandSize::Dword)
}

#[test]
fn vpternlogq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledDisplaced(EAX, Two, 865726378, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(20)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 253, 143, 37, 12, 69, 170, 239, 153, 51, 20], OperandSize::Dword)
}

#[test]
fn vpternlogq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledDisplaced(ESI, Four, 799823547, Some(OperandSize::Qword), None)), operand4: Some(Literal8(121)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 243, 213, 153, 37, 12, 181, 187, 86, 172, 47, 121], OperandSize::Dword)
}

#[test]
fn vpternlogq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM29)), operand4: Some(Literal8(112)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 147, 229, 138, 37, 197, 112], OperandSize::Qword)
}

#[test]
fn vpternlogq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGQ, operand1: Some(Direct(XMM9)), operand2: Some(Direct(XMM20)), operand3: Some(IndirectScaledIndexed(RDX, RSI, Two, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(78)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 115, 221, 135, 37, 12, 114, 78], OperandSize::Qword)
}

#[test]
fn vpternlogq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGQ, operand1: Some(Direct(XMM31)), operand2: Some(Direct(XMM10)), operand3: Some(IndirectScaledIndexed(RSI, RBX, Four, Some(OperandSize::Qword), None)), operand4: Some(Literal8(11)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 99, 173, 157, 37, 60, 158, 11], OperandSize::Qword)
}

#[test]
fn vpternlogq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGQ, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM2)), operand4: Some(Literal8(91)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 229, 170, 37, 234, 91], OperandSize::Dword)
}

#[test]
fn vpternlogq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGQ, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EDI, Four, 337714776, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(49)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 197, 171, 37, 164, 191, 88, 30, 33, 20, 49], OperandSize::Dword)
}

#[test]
fn vpternlogq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGQ, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM2)), operand3: Some(Indirect(EDX, Some(OperandSize::Qword), None)), operand4: Some(Literal8(101)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 237, 185, 37, 50, 101], OperandSize::Dword)
}

#[test]
fn vpternlogq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGQ, operand1: Some(Direct(YMM31)), operand2: Some(Direct(YMM16)), operand3: Some(Direct(YMM28)), operand4: Some(Literal8(65)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 3, 253, 163, 37, 252, 65], OperandSize::Qword)
}

#[test]
fn vpternlogq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGQ, operand1: Some(Direct(YMM9)), operand2: Some(Direct(YMM21)), operand3: Some(Indirect(RSI, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(115)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 115, 213, 164, 37, 14, 115], OperandSize::Qword)
}

#[test]
fn vpternlogq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGQ, operand1: Some(Direct(YMM28)), operand2: Some(Direct(YMM14)), operand3: Some(IndirectScaledIndexed(RBX, RBX, Eight, Some(OperandSize::Qword), None)), operand4: Some(Literal8(125)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 99, 141, 189, 37, 36, 219, 125], OperandSize::Qword)
}

#[test]
fn vpternlogq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGQ, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM2)), operand4: Some(Literal8(120)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 253, 203, 37, 242, 120], OperandSize::Dword)
}

#[test]
fn vpternlogq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGQ, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EBX, Four, 643164846, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(45)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 205, 204, 37, 140, 152, 174, 234, 85, 38, 45], OperandSize::Dword)
}

#[test]
fn vpternlogq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGQ, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledDisplaced(ESI, Two, 296928766, Some(OperandSize::Qword), None)), operand4: Some(Literal8(93)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 197, 220, 37, 12, 117, 254, 197, 178, 17, 93], OperandSize::Dword)
}

#[test]
fn vpternlogq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGQ, operand1: Some(Direct(ZMM30)), operand2: Some(Direct(ZMM28)), operand3: Some(Direct(ZMM22)), operand4: Some(Literal8(120)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 35, 157, 194, 37, 246, 120], OperandSize::Qword)
}

#[test]
fn vpternlogq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGQ, operand1: Some(Direct(ZMM29)), operand2: Some(Direct(ZMM16)), operand3: Some(Indirect(RDI, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(52)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 99, 253, 199, 37, 47, 52], OperandSize::Qword)
}

#[test]
fn vpternlogq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGQ, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM16)), operand3: Some(IndirectDisplaced(RDI, 1682819897, Some(OperandSize::Qword), None)), operand4: Some(Literal8(18)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 253, 213, 37, 191, 57, 203, 77, 100, 18], OperandSize::Qword)
}

