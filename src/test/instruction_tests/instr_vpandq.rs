use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpandq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 229, 141, 219, 222], OperandSize::Dword)
}

#[test]
fn vpandq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexed(EAX, ESI, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 221, 143, 219, 44, 240], OperandSize::Dword)
}

#[test]
fn vpandq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EBX, Four, 1239618834, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 229, 156, 219, 180, 154, 18, 21, 227, 73], OperandSize::Dword)
}

#[test]
fn vpandq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDQ, operand1: Some(Direct(XMM10)), operand2: Some(Direct(XMM27)), operand3: Some(Direct(XMM26)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 17, 165, 131, 219, 210], OperandSize::Qword)
}

#[test]
fn vpandq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDQ, operand1: Some(Direct(XMM30)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledDisplaced(RBX, Two, 1648585362, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 97, 229, 140, 219, 52, 93, 146, 106, 67, 98], OperandSize::Qword)
}

#[test]
fn vpandq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDQ, operand1: Some(Direct(XMM14)), operand2: Some(Direct(XMM18)), operand3: Some(Indirect(RDX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 113, 237, 147, 219, 50], OperandSize::Qword)
}

#[test]
fn vpandq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDQ, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 253, 174, 219, 249], OperandSize::Dword)
}

#[test]
fn vpandq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDQ, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledDisplaced(ESI, Two, 1789177588, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 253, 169, 219, 4, 117, 244, 174, 164, 106], OperandSize::Dword)
}

#[test]
fn vpandq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDQ, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, ESI, Eight, 1010254578, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 197, 188, 219, 140, 242, 242, 66, 55, 60], OperandSize::Dword)
}

#[test]
fn vpandq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDQ, operand1: Some(Direct(YMM21)), operand2: Some(Direct(YMM12)), operand3: Some(Direct(YMM16)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 161, 157, 174, 219, 232], OperandSize::Qword)
}

#[test]
fn vpandq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDQ, operand1: Some(Direct(YMM18)), operand2: Some(Direct(YMM2)), operand3: Some(Indirect(RBX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 225, 237, 172, 219, 19], OperandSize::Qword)
}

#[test]
fn vpandq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDQ, operand1: Some(Direct(YMM15)), operand2: Some(Direct(YMM7)), operand3: Some(Indirect(RBX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 113, 197, 190, 219, 59], OperandSize::Qword)
}

#[test]
fn vpandq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDQ, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 221, 203, 219, 215], OperandSize::Dword)
}

#[test]
fn vpandq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDQ, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, ESI, Two, 523588987, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 197, 206, 219, 172, 118, 123, 85, 53, 31], OperandSize::Dword)
}

#[test]
fn vpandq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDQ, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, ESI, Eight, 2100588710, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 205, 220, 219, 180, 240, 166, 112, 52, 125], OperandSize::Dword)
}

#[test]
fn vpandq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDQ, operand1: Some(Direct(ZMM11)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM21)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 49, 229, 204, 219, 221], OperandSize::Qword)
}

#[test]
fn vpandq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDQ, operand1: Some(Direct(ZMM22)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledIndexed(RDX, RAX, Four, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 225, 213, 204, 219, 52, 130], OperandSize::Qword)
}

#[test]
fn vpandq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDQ, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM19)), operand3: Some(Indirect(RDI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 229, 215, 219, 31], OperandSize::Qword)
}

