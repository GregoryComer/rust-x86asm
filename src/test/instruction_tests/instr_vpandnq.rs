use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpandnq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDNQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 205, 140, 223, 200], OperandSize::Dword)
}

#[test]
fn vpandnq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDNQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(Indirect(EBX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 229, 142, 223, 59], OperandSize::Dword)
}

#[test]
fn vpandnq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDNQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexed(EDX, ESI, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 229, 156, 223, 12, 178], OperandSize::Dword)
}

#[test]
fn vpandnq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDNQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM20)), operand3: Some(Direct(XMM25)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 145, 221, 135, 223, 233], OperandSize::Qword)
}

#[test]
fn vpandnq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDNQ, operand1: Some(Direct(XMM22)), operand2: Some(Direct(XMM30)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RDI, Eight, 1951925888, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 225, 141, 129, 223, 180, 249, 128, 6, 88, 116], OperandSize::Qword)
}

#[test]
fn vpandnq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDNQ, operand1: Some(Direct(XMM28)), operand2: Some(Direct(XMM2)), operand3: Some(Indirect(RSI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 97, 237, 158, 223, 38], OperandSize::Qword)
}

#[test]
fn vpandnq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDNQ, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 197, 170, 223, 245], OperandSize::Dword)
}

#[test]
fn vpandnq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDNQ, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexed(EAX, ESI, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 213, 175, 223, 20, 240], OperandSize::Dword)
}

#[test]
fn vpandnq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDNQ, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, ECX, Four, 1944919324, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 229, 188, 223, 140, 142, 28, 29, 237, 115], OperandSize::Dword)
}

#[test]
fn vpandnq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDNQ, operand1: Some(Direct(YMM30)), operand2: Some(Direct(YMM11)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 97, 165, 172, 223, 244], OperandSize::Qword)
}

#[test]
fn vpandnq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDNQ, operand1: Some(Direct(YMM18)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledDisplaced(RSI, Eight, 1333419516, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 225, 197, 169, 223, 20, 245, 252, 93, 122, 79], OperandSize::Qword)
}

#[test]
fn vpandnq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDNQ, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM14)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RBX, Four, 20324013, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 141, 187, 223, 140, 153, 173, 30, 54, 1], OperandSize::Qword)
}

#[test]
fn vpandnq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDNQ, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 205, 205, 223, 231], OperandSize::Dword)
}

#[test]
fn vpandnq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDNQ, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM4)), operand3: Some(Indirect(EBX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 221, 207, 223, 51], OperandSize::Dword)
}

#[test]
fn vpandnq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDNQ, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM6)), operand3: Some(Indirect(ECX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 205, 217, 223, 49], OperandSize::Dword)
}

#[test]
fn vpandnq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDNQ, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM16)), operand3: Some(Direct(ZMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 253, 198, 223, 236], OperandSize::Qword)
}

#[test]
fn vpandnq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDNQ, operand1: Some(Direct(ZMM12)), operand2: Some(Direct(ZMM23)), operand3: Some(IndirectScaledIndexed(RBX, RDX, Eight, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 113, 197, 197, 223, 36, 211], OperandSize::Qword)
}

#[test]
fn vpandnq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDNQ, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM18)), operand3: Some(IndirectScaledIndexed(RAX, RDX, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 237, 213, 223, 36, 80], OperandSize::Qword)
}

