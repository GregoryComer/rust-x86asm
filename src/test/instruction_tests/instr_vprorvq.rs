use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vprorvq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 205, 141, 20, 201], OperandSize::Dword)
}

#[test]
fn vprorvq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexed(ESI, EDI, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 221, 140, 20, 28, 254], OperandSize::Dword)
}

#[test]
fn vprorvq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledDisplaced(ESI, Eight, 880211619, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 205, 158, 20, 12, 245, 163, 246, 118, 52], OperandSize::Dword)
}

#[test]
fn vprorvq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVQ, operand1: Some(Direct(XMM21)), operand2: Some(Direct(XMM17)), operand3: Some(Direct(XMM11)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 194, 245, 134, 20, 235], OperandSize::Qword)
}

#[test]
fn vprorvq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM26)), operand3: Some(Indirect(RDI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 173, 129, 20, 55], OperandSize::Qword)
}

#[test]
fn vprorvq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVQ, operand1: Some(Direct(XMM23)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexed(RCX, RBX, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 226, 245, 154, 20, 60, 89], OperandSize::Qword)
}

#[test]
fn vprorvq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVQ, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 205, 175, 20, 240], OperandSize::Dword)
}

#[test]
fn vprorvq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVQ, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectDisplaced(EBX, 181009994, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 213, 172, 20, 155, 74, 254, 201, 10], OperandSize::Dword)
}

#[test]
fn vprorvq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVQ, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, ECX, Four, 772727543, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 245, 187, 20, 140, 136, 247, 226, 14, 46], OperandSize::Dword)
}

#[test]
fn vprorvq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVQ, operand1: Some(Direct(YMM15)), operand2: Some(Direct(YMM18)), operand3: Some(Direct(YMM21)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 50, 237, 167, 20, 253], OperandSize::Qword)
}

#[test]
fn vprorvq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVQ, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM14)), operand3: Some(IndirectScaledDisplaced(RDX, Four, 1940946922, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 141, 171, 20, 28, 149, 234, 127, 176, 115], OperandSize::Qword)
}

#[test]
fn vprorvq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVQ, operand1: Some(Direct(YMM24)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RCX, Eight, 2006056060, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 98, 237, 191, 20, 132, 207, 124, 252, 145, 119], OperandSize::Qword)
}

#[test]
fn vprorvq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVQ, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 253, 206, 20, 225], OperandSize::Dword)
}

#[test]
fn vprorvq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVQ, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, EBX, Four, 1531044232, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 245, 205, 20, 140, 153, 136, 225, 65, 91], OperandSize::Dword)
}

#[test]
fn vprorvq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVQ, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectScaledIndexed(ESI, ESI, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 253, 222, 20, 60, 246], OperandSize::Dword)
}

#[test]
fn vprorvq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVQ, operand1: Some(Direct(ZMM24)), operand2: Some(Direct(ZMM25)), operand3: Some(Direct(ZMM22)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 34, 181, 199, 20, 198], OperandSize::Qword)
}

#[test]
fn vprorvq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVQ, operand1: Some(Direct(ZMM19)), operand2: Some(Direct(ZMM18)), operand3: Some(Indirect(RDI, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 226, 237, 194, 20, 31], OperandSize::Qword)
}

#[test]
fn vprorvq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVQ, operand1: Some(Direct(ZMM10)), operand2: Some(Direct(ZMM20)), operand3: Some(IndirectScaledDisplaced(RDI, Four, 1274099391, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 114, 221, 213, 20, 20, 189, 191, 54, 241, 75], OperandSize::Qword)
}

