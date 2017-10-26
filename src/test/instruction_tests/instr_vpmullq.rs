use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmullq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 197, 137, 64, 254], OperandSize::Dword)
}

#[test]
fn vpmullq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectDisplaced(EBX, 1181436746, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 205, 140, 64, 179, 74, 75, 107, 70], OperandSize::Dword)
}

#[test]
fn vpmullq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexed(EAX, EAX, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 197, 158, 64, 4, 192], OperandSize::Dword)
}

#[test]
fn vpmullq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLQ, operand1: Some(Direct(XMM12)), operand2: Some(Direct(XMM22)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 114, 205, 130, 64, 225], OperandSize::Qword)
}

#[test]
fn vpmullq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM29)), operand3: Some(IndirectScaledIndexed(RDX, RCX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 149, 131, 64, 4, 74], OperandSize::Qword)
}

#[test]
fn vpmullq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLQ, operand1: Some(Direct(XMM31)), operand2: Some(Direct(XMM9)), operand3: Some(IndirectDisplaced(RDI, 279135131, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 98, 181, 154, 64, 191, 155, 67, 163, 16], OperandSize::Qword)
}

#[test]
fn vpmullq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLQ, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 221, 172, 64, 250], OperandSize::Dword)
}

#[test]
fn vpmullq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLQ, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexed(EBX, EAX, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 237, 174, 64, 52, 67], OperandSize::Dword)
}

#[test]
fn vpmullq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLQ, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM7)), operand3: Some(Indirect(ECX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 197, 187, 64, 17], OperandSize::Dword)
}

#[test]
fn vpmullq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLQ, operand1: Some(Direct(YMM8)), operand2: Some(Direct(YMM16)), operand3: Some(Direct(YMM27)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 18, 253, 161, 64, 195], OperandSize::Qword)
}

#[test]
fn vpmullq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLQ, operand1: Some(Direct(YMM30)), operand2: Some(Direct(YMM26)), operand3: Some(Indirect(RDI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 98, 173, 166, 64, 55], OperandSize::Qword)
}

#[test]
fn vpmullq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLQ, operand1: Some(Direct(YMM16)), operand2: Some(Direct(YMM22)), operand3: Some(IndirectScaledDisplaced(RBX, Four, 1539078520, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 226, 205, 183, 64, 4, 157, 120, 121, 188, 91], OperandSize::Qword)
}

#[test]
fn vpmullq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLQ, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 197, 201, 64, 221], OperandSize::Dword)
}

#[test]
fn vpmullq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLQ, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledDisplaced(ESI, Four, 812096282, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 221, 204, 64, 20, 181, 26, 155, 103, 48], OperandSize::Dword)
}

#[test]
fn vpmullq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLQ, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledDisplaced(EDX, Eight, 1018080843, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 221, 222, 64, 4, 213, 75, 174, 174, 60], OperandSize::Dword)
}

#[test]
fn vpmullq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLQ, operand1: Some(Direct(ZMM13)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(ZMM31)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 18, 237, 202, 64, 239], OperandSize::Qword)
}

#[test]
fn vpmullq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLQ, operand1: Some(Direct(ZMM28)), operand2: Some(Direct(ZMM21)), operand3: Some(Indirect(RBX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 98, 213, 193, 64, 35], OperandSize::Qword)
}

#[test]
fn vpmullq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULLQ, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM1)), operand3: Some(Indirect(RBX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 245, 218, 64, 27], OperandSize::Qword)
}

