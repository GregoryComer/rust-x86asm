use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vbroadcastss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSS, operand1: Some(Direct(XMM3)), operand2: Some(Indirect(EBX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 24, 27], OperandSize::Dword)
}

#[test]
fn vbroadcastss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSS, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexed(RSI, RAX, Four, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 24, 12, 134], OperandSize::Qword)
}

#[test]
fn vbroadcastss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSS, operand1: Some(Direct(YMM6)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, ESI, Four, 1619324037, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 24, 180, 182, 133, 236, 132, 96], OperandSize::Dword)
}

#[test]
fn vbroadcastss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSS, operand1: Some(Direct(YMM6)), operand2: Some(Indirect(RDX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 24, 50], OperandSize::Qword)
}

#[test]
fn vbroadcastss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 24, 202], OperandSize::Dword)
}

#[test]
fn vbroadcastss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 24, 227], OperandSize::Qword)
}

#[test]
fn vbroadcastss_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 24, 198], OperandSize::Dword)
}

#[test]
fn vbroadcastss_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 24, 211], OperandSize::Qword)
}

#[test]
fn vbroadcastss_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 138, 24, 201], OperandSize::Dword)
}

#[test]
fn vbroadcastss_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSS, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, EDX, Two, 1144001784, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 142, 24, 164, 86, 248, 20, 48, 68], OperandSize::Dword)
}

#[test]
fn vbroadcastss_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSS, operand1: Some(Direct(XMM10)), operand2: Some(Direct(XMM15)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 82, 125, 139, 24, 215], OperandSize::Qword)
}

#[test]
fn vbroadcastss_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSS, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexed(RAX, RCX, Four, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 143, 24, 28, 136], OperandSize::Qword)
}

#[test]
fn vbroadcastss_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 174, 24, 212], OperandSize::Dword)
}

#[test]
fn vbroadcastss_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSS, operand1: Some(Direct(YMM4)), operand2: Some(IndirectDisplaced(ECX, 110425198, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 174, 24, 161, 110, 244, 148, 6], OperandSize::Dword)
}

#[test]
fn vbroadcastss_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSS, operand1: Some(Direct(YMM12)), operand2: Some(Direct(XMM24)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 18, 125, 170, 24, 224], OperandSize::Qword)
}

#[test]
fn vbroadcastss_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSS, operand1: Some(Direct(YMM21)), operand2: Some(IndirectScaledIndexed(RSI, RCX, Two, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 226, 125, 170, 24, 44, 78], OperandSize::Qword)
}

#[test]
fn vbroadcastss_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSS, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 205, 24, 254], OperandSize::Dword)
}

#[test]
fn vbroadcastss_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSS, operand1: Some(Direct(ZMM5)), operand2: Some(IndirectScaledDisplaced(EDX, Four, 94730554, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 205, 24, 44, 149, 58, 121, 165, 5], OperandSize::Dword)
}

#[test]
fn vbroadcastss_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSS, operand1: Some(Direct(ZMM13)), operand2: Some(Direct(XMM16)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 50, 125, 201, 24, 232], OperandSize::Qword)
}

#[test]
fn vbroadcastss_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTSS, operand1: Some(Direct(ZMM26)), operand2: Some(IndirectDisplaced(RBX, 645048727, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 98, 125, 203, 24, 147, 151, 169, 114, 38], OperandSize::Qword)
}

