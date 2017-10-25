use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmaxsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 61, 250], OperandSize::Dword)
}

#[test]
fn vpmaxsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexed(EDI, ECX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 61, 60, 143], OperandSize::Dword)
}

#[test]
fn vpmaxsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 61, 241], OperandSize::Qword)
}

#[test]
fn vpmaxsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RCX, Two, 1295481042, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 61, 172, 74, 210, 120, 55, 77], OperandSize::Qword)
}

#[test]
fn vpmaxsd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 69, 61, 217], OperandSize::Dword)
}

#[test]
fn vpmaxsd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectDisplaced(EAX, 1881033474, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 69, 61, 168, 2, 75, 30, 112], OperandSize::Dword)
}

#[test]
fn vpmaxsd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 61, 217], OperandSize::Qword)
}

#[test]
fn vpmaxsd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledDisplaced(RCX, Four, 1645305974, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 93, 61, 52, 141, 118, 96, 17, 98], OperandSize::Qword)
}

#[test]
fn vpmaxsd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 85, 137, 61, 232], OperandSize::Dword)
}

#[test]
fn vpmaxsd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexed(ESI, ECX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 138, 61, 28, 206], OperandSize::Dword)
}

#[test]
fn vpmaxsd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, ECX, Eight, 2074769382, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 125, 155, 61, 140, 201, 230, 119, 170, 123], OperandSize::Dword)
}

#[test]
fn vpmaxsd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSD, operand1: Some(Direct(XMM19)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM14)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 194, 69, 139, 61, 222], OperandSize::Qword)
}

#[test]
fn vpmaxsd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM14)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RAX, Four, 1395738977, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 13, 138, 61, 148, 128, 97, 73, 49, 83], OperandSize::Qword)
}

#[test]
fn vpmaxsd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM15)), operand3: Some(IndirectScaledDisplaced(RSI, Eight, 976349645, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 5, 155, 61, 36, 245, 205, 233, 49, 58], OperandSize::Qword)
}

