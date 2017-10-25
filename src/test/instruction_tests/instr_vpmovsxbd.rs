use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmovsxbd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 33, 229], OperandSize::Dword)
}

#[test]
fn vpmovsxbd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBD, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, EAX, Two, 472483228, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 33, 156, 71, 156, 133, 41, 28], OperandSize::Dword)
}

#[test]
fn vpmovsxbd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 33, 236], OperandSize::Qword)
}

#[test]
fn vpmovsxbd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBD, operand1: Some(Direct(XMM5)), operand2: Some(Indirect(RSI, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 33, 46], OperandSize::Qword)
}

#[test]
fn vpmovsxbd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 33, 232], OperandSize::Dword)
}

#[test]
fn vpmovsxbd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBD, operand1: Some(Direct(YMM1)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, EDI, Four, 1745876317, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 33, 140, 186, 93, 245, 15, 104], OperandSize::Dword)
}

#[test]
fn vpmovsxbd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 33, 254], OperandSize::Qword)
}

#[test]
fn vpmovsxbd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBD, operand1: Some(Direct(YMM1)), operand2: Some(IndirectScaledIndexed(RDX, RDX, Eight, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 33, 12, 210], OperandSize::Qword)
}

#[test]
fn vpmovsxbd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 142, 33, 197], OperandSize::Dword)
}

#[test]
fn vpmovsxbd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectDisplaced(EAX, 2039914436, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 140, 33, 160, 196, 159, 150, 121], OperandSize::Dword)
}

#[test]
fn vpmovsxbd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBD, operand1: Some(Direct(XMM23)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 226, 125, 143, 33, 253], OperandSize::Qword)
}

#[test]
fn vpmovsxbd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBD, operand1: Some(Direct(XMM25)), operand2: Some(IndirectDisplaced(RBX, 416378716, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 98, 125, 139, 33, 139, 92, 111, 209, 24], OperandSize::Qword)
}

#[test]
fn vpmovsxbd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 174, 33, 251], OperandSize::Dword)
}

#[test]
fn vpmovsxbd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBD, operand1: Some(Direct(YMM2)), operand2: Some(IndirectDisplaced(ESI, 1893398902, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 169, 33, 150, 118, 249, 218, 112], OperandSize::Dword)
}

#[test]
fn vpmovsxbd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(XMM11)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 210, 125, 171, 33, 219], OperandSize::Qword)
}

#[test]
fn vpmovsxbd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBD, operand1: Some(Direct(YMM31)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RBX, Eight, 2065268984, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 98, 125, 173, 33, 188, 222, 248, 128, 25, 123], OperandSize::Qword)
}

#[test]
fn vpmovsxbd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 202, 33, 248], OperandSize::Dword)
}

#[test]
fn vpmovsxbd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBD, operand1: Some(Direct(ZMM7)), operand2: Some(Indirect(ECX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 207, 33, 57], OperandSize::Dword)
}

#[test]
fn vpmovsxbd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBD, operand1: Some(Direct(ZMM25)), operand2: Some(Direct(XMM17)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 34, 125, 202, 33, 201], OperandSize::Qword)
}

#[test]
fn vpmovsxbd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBD, operand1: Some(Direct(ZMM27)), operand2: Some(IndirectScaledIndexed(RDX, RCX, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 98, 125, 204, 33, 28, 202], OperandSize::Qword)
}

