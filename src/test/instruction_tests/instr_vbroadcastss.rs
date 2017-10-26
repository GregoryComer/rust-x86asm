use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vbroadcastss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSS, operand1: Some(Direct(XMM3)), operand2: Some(IndirectDisplaced(ECX, 1663065660, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 24, 153, 60, 94, 32, 99], OperandSize::Dword)
}

#[test]
fn vbroadcastss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSS, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledDisplaced(RDI, Two, 420320477, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 24, 36, 125, 221, 148, 13, 25], OperandSize::Qword)
}

#[test]
fn vbroadcastss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSS, operand1: Some(Direct(YMM2)), operand2: Some(IndirectScaledIndexed(EAX, ECX, Two, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 24, 20, 72], OperandSize::Dword)
}

#[test]
fn vbroadcastss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSS, operand1: Some(Direct(YMM5)), operand2: Some(IndirectDisplaced(RDI, 133410213, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 24, 175, 165, 173, 243, 7], OperandSize::Qword)
}

#[test]
fn vbroadcastss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 24, 255], OperandSize::Dword)
}

#[test]
fn vbroadcastss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 24, 199], OperandSize::Qword)
}

#[test]
fn vbroadcastss_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 24, 250], OperandSize::Dword)
}

#[test]
fn vbroadcastss_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 24, 241], OperandSize::Qword)
}

#[test]
fn vbroadcastss_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 139, 24, 242], OperandSize::Dword)
}

#[test]
fn vbroadcastss_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSS, operand1: Some(Direct(XMM4)), operand2: Some(IndirectDisplaced(EDX, 637649946, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 141, 24, 162, 26, 196, 1, 38], OperandSize::Dword)
}

#[test]
fn vbroadcastss_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSS, operand1: Some(Direct(XMM11)), operand2: Some(Direct(XMM23)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 50, 125, 141, 24, 223], OperandSize::Qword)
}

#[test]
fn vbroadcastss_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSS, operand1: Some(Direct(XMM11)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RSI, Two, 84979859, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 114, 125, 139, 24, 156, 115, 147, 176, 16, 5], OperandSize::Qword)
}

#[test]
fn vbroadcastss_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 173, 24, 212], OperandSize::Dword)
}

#[test]
fn vbroadcastss_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSS, operand1: Some(Direct(YMM0)), operand2: Some(IndirectDisplaced(ESI, 120619824, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 173, 24, 134, 48, 131, 48, 7], OperandSize::Dword)
}

#[test]
fn vbroadcastss_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSS, operand1: Some(Direct(YMM22)), operand2: Some(Direct(XMM11)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 194, 125, 172, 24, 243], OperandSize::Qword)
}

#[test]
fn vbroadcastss_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSS, operand1: Some(Direct(YMM16)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RDX, Eight, 895477571, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 226, 125, 175, 24, 132, 208, 67, 231, 95, 53], OperandSize::Qword)
}

#[test]
fn vbroadcastss_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSS, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 201, 24, 238], OperandSize::Dword)
}

#[test]
fn vbroadcastss_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSS, operand1: Some(Direct(ZMM5)), operand2: Some(Indirect(ESI, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 201, 24, 46], OperandSize::Dword)
}

#[test]
fn vbroadcastss_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSS, operand1: Some(Direct(ZMM25)), operand2: Some(Direct(XMM30)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 2, 125, 205, 24, 206], OperandSize::Qword)
}

#[test]
fn vbroadcastss_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSS, operand1: Some(Direct(ZMM14)), operand2: Some(IndirectScaledIndexed(RAX, RBX, Eight, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 114, 125, 201, 24, 52, 216], OperandSize::Qword)
}

