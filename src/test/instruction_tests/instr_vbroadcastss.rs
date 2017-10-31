use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vbroadcastss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSS, operand1: Some(Direct(XMM2)), operand2: Some(IndirectDisplaced(EBX, 456443938, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 24, 147, 34, 200, 52, 27], OperandSize::Dword)
}

#[test]
fn vbroadcastss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSS, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RDX, Eight, 1686385737, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 24, 148, 211, 73, 52, 132, 100], OperandSize::Qword)
}

#[test]
fn vbroadcastss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSS, operand1: Some(Direct(YMM2)), operand2: Some(Indirect(EDI, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 24, 23], OperandSize::Dword)
}

#[test]
fn vbroadcastss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSS, operand1: Some(Direct(YMM1)), operand2: Some(IndirectScaledDisplaced(RDX, Four, 2021790408, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 24, 12, 149, 200, 18, 130, 120], OperandSize::Qword)
}

#[test]
fn vbroadcastss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 24, 226], OperandSize::Dword)
}

#[test]
fn vbroadcastss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 24, 204], OperandSize::Qword)
}

#[test]
fn vbroadcastss_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 24, 243], OperandSize::Dword)
}

#[test]
fn vbroadcastss_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 24, 250], OperandSize::Qword)
}

#[test]
fn vbroadcastss_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 143, 24, 194], OperandSize::Dword)
}

#[test]
fn vbroadcastss_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSS, operand1: Some(Direct(XMM0)), operand2: Some(IndirectDisplaced(EBX, 1379853108, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 142, 24, 131, 52, 227, 62, 82], OperandSize::Dword)
}

#[test]
fn vbroadcastss_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSS, operand1: Some(Direct(XMM10)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 114, 125, 138, 24, 214], OperandSize::Qword)
}

#[test]
fn vbroadcastss_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSS, operand1: Some(Direct(XMM4)), operand2: Some(IndirectDisplaced(RDI, 1602832887, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 138, 24, 167, 247, 73, 137, 95], OperandSize::Qword)
}

#[test]
fn vbroadcastss_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 174, 24, 216], OperandSize::Dword)
}

#[test]
fn vbroadcastss_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSS, operand1: Some(Direct(YMM6)), operand2: Some(IndirectScaledIndexed(ESI, EBX, Four, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 173, 24, 52, 158], OperandSize::Dword)
}

#[test]
fn vbroadcastss_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSS, operand1: Some(Direct(YMM25)), operand2: Some(Direct(XMM21)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 34, 125, 170, 24, 205], OperandSize::Qword)
}

#[test]
fn vbroadcastss_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSS, operand1: Some(Direct(YMM13)), operand2: Some(IndirectScaledIndexed(RCX, RBX, Two, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 114, 125, 169, 24, 44, 89], OperandSize::Qword)
}

#[test]
fn vbroadcastss_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSS, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 206, 24, 206], OperandSize::Dword)
}

#[test]
fn vbroadcastss_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSS, operand1: Some(Direct(ZMM4)), operand2: Some(Indirect(EAX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 203, 24, 32], OperandSize::Dword)
}

#[test]
fn vbroadcastss_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSS, operand1: Some(Direct(ZMM16)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 226, 125, 207, 24, 195], OperandSize::Qword)
}

#[test]
fn vbroadcastss_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSS, operand1: Some(Direct(ZMM28)), operand2: Some(Indirect(RDX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 98, 125, 207, 24, 34], OperandSize::Qword)
}

