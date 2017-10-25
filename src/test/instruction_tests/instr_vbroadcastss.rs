use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vbroadcastss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSS, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledDisplaced(EBX, Eight, 1182386042, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 24, 20, 221, 122, 199, 121, 70], OperandSize::Dword)
}

#[test]
fn vbroadcastss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSS, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledDisplaced(RDX, Two, 823735600, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 24, 4, 85, 48, 53, 25, 49], OperandSize::Qword)
}

#[test]
fn vbroadcastss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSS, operand1: Some(Direct(YMM2)), operand2: Some(IndirectDisplaced(EDI, 82580475, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 24, 151, 251, 19, 236, 4], OperandSize::Dword)
}

#[test]
fn vbroadcastss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSS, operand1: Some(Direct(YMM3)), operand2: Some(IndirectScaledDisplaced(RBX, Two, 1016487262, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 24, 28, 93, 94, 93, 150, 60], OperandSize::Qword)
}

#[test]
fn vbroadcastss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 24, 201], OperandSize::Dword)
}

#[test]
fn vbroadcastss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 24, 248], OperandSize::Qword)
}

#[test]
fn vbroadcastss_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 24, 230], OperandSize::Dword)
}

#[test]
fn vbroadcastss_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 24, 255], OperandSize::Qword)
}

#[test]
fn vbroadcastss_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 137, 24, 216], OperandSize::Dword)
}

#[test]
fn vbroadcastss_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSS, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledDisplaced(EBX, Eight, 1523700174, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 141, 24, 44, 221, 206, 209, 209, 90], OperandSize::Dword)
}

#[test]
fn vbroadcastss_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM9)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 210, 125, 141, 24, 217], OperandSize::Qword)
}

#[test]
fn vbroadcastss_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSS, operand1: Some(Direct(XMM14)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RAX, Eight, 1314697695, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 114, 125, 142, 24, 180, 193, 223, 177, 92, 78], OperandSize::Qword)
}

#[test]
fn vbroadcastss_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 169, 24, 230], OperandSize::Dword)
}

#[test]
fn vbroadcastss_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSS, operand1: Some(Direct(YMM1)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, EDX, Four, 1815134645, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 173, 24, 140, 151, 181, 193, 48, 108], OperandSize::Dword)
}

#[test]
fn vbroadcastss_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSS, operand1: Some(Direct(YMM27)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 98, 125, 172, 24, 223], OperandSize::Qword)
}

#[test]
fn vbroadcastss_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSS, operand1: Some(Direct(YMM13)), operand2: Some(IndirectDisplaced(RAX, 783689397, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 114, 125, 174, 24, 168, 181, 38, 182, 46], OperandSize::Qword)
}

#[test]
fn vbroadcastss_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSS, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 203, 24, 214], OperandSize::Dword)
}

#[test]
fn vbroadcastss_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSS, operand1: Some(Direct(ZMM1)), operand2: Some(IndirectScaledDisplaced(EDI, Two, 626311824, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 203, 24, 12, 125, 144, 194, 84, 37], OperandSize::Dword)
}

#[test]
fn vbroadcastss_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSS, operand1: Some(Direct(ZMM15)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 114, 125, 206, 24, 252], OperandSize::Qword)
}

#[test]
fn vbroadcastss_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSS, operand1: Some(Direct(ZMM14)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RCX, Two, 1738083696, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 114, 125, 207, 24, 180, 74, 112, 13, 153, 103], OperandSize::Qword)
}

