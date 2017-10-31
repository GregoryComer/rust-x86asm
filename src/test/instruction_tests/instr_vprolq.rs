use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vprolq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: Some(Literal8(30)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 205, 141, 114, 206, 30], OperandSize::Dword)
}

#[test]
fn vprolq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLQ, operand1: Some(Direct(XMM4)), operand2: Some(IndirectDisplaced(EDI, 1470689785, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(76)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 221, 143, 114, 143, 249, 241, 168, 87, 76], OperandSize::Dword)
}

#[test]
fn vprolq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLQ, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, ESI, Two, 1295638431, Some(OperandSize::Qword), None)), operand3: Some(Literal8(70)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 229, 156, 114, 140, 118, 159, 223, 57, 77, 70], OperandSize::Dword)
}

#[test]
fn vprolq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM28)), operand3: Some(Literal8(70)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 145, 205, 138, 114, 204, 70], OperandSize::Qword)
}

#[test]
fn vprolq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLQ, operand1: Some(Direct(XMM21)), operand2: Some(IndirectScaledDisplaced(RAX, Eight, 1371581423, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(68)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 213, 132, 114, 12, 197, 239, 171, 192, 81, 68], OperandSize::Qword)
}

#[test]
fn vprolq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLQ, operand1: Some(Direct(XMM20)), operand2: Some(Indirect(RDX, Some(OperandSize::Qword), None)), operand3: Some(Literal8(106)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 221, 151, 114, 10, 106], OperandSize::Qword)
}

#[test]
fn vprolq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLQ, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM2)), operand3: Some(Literal8(86)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 253, 169, 114, 202, 86], OperandSize::Dword)
}

#[test]
fn vprolq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLQ, operand1: Some(Direct(YMM4)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, EBX, Four, 1477618567, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(89)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 221, 173, 114, 140, 158, 135, 171, 18, 88, 89], OperandSize::Dword)
}

#[test]
fn vprolq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLQ, operand1: Some(Direct(YMM2)), operand2: Some(IndirectScaledDisplaced(EBX, Two, 2037226770, Some(OperandSize::Qword), None)), operand3: Some(Literal8(21)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 237, 186, 114, 12, 93, 18, 157, 109, 121, 21], OperandSize::Dword)
}

#[test]
fn vprolq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLQ, operand1: Some(Direct(YMM30)), operand2: Some(Direct(YMM1)), operand3: Some(Literal8(39)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 141, 164, 114, 201, 39], OperandSize::Qword)
}

#[test]
fn vprolq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLQ, operand1: Some(Direct(YMM3)), operand2: Some(IndirectScaledIndexed(RSI, RBX, Four, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 229, 170, 114, 12, 158, 2], OperandSize::Qword)
}

#[test]
fn vprolq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLQ, operand1: Some(Direct(YMM18)), operand2: Some(IndirectDisplaced(RCX, 2000571218, Some(OperandSize::Qword), None)), operand3: Some(Literal8(91)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 237, 182, 114, 137, 82, 75, 62, 119, 91], OperandSize::Qword)
}

#[test]
fn vprolq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLQ, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM7)), operand3: Some(Literal8(72)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 245, 206, 114, 207, 72], OperandSize::Dword)
}

#[test]
fn vprolq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLQ, operand1: Some(Direct(ZMM2)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, EBX, Eight, 774180166, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(14)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 237, 202, 114, 140, 216, 70, 13, 37, 46, 14], OperandSize::Dword)
}

#[test]
fn vprolq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLQ, operand1: Some(Direct(ZMM0)), operand2: Some(IndirectScaledIndexed(EDX, EDI, Eight, Some(OperandSize::Qword), None)), operand3: Some(Literal8(35)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 253, 217, 114, 12, 250, 35], OperandSize::Dword)
}

#[test]
fn vprolq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLQ, operand1: Some(Direct(ZMM9)), operand2: Some(Direct(ZMM4)), operand3: Some(Literal8(23)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 181, 204, 114, 204, 23], OperandSize::Qword)
}

#[test]
fn vprolq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLQ, operand1: Some(Direct(ZMM25)), operand2: Some(IndirectScaledDisplaced(RAX, Eight, 1813161535, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(118)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 181, 198, 114, 12, 197, 63, 166, 18, 108, 118], OperandSize::Qword)
}

#[test]
fn vprolq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLQ, operand1: Some(Direct(ZMM4)), operand2: Some(IndirectScaledDisplaced(RSI, Two, 753974549, Some(OperandSize::Qword), None)), operand3: Some(Literal8(91)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 221, 221, 114, 12, 117, 21, 189, 240, 44, 91], OperandSize::Qword)
}

