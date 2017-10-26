use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpunpcklbw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLBW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 96, 225], OperandSize::Dword)
}

#[test]
fn vpunpcklbw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLBW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: Some(Indirect(EAX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 96, 16], OperandSize::Dword)
}

#[test]
fn vpunpcklbw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLBW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 96, 215], OperandSize::Qword)
}

#[test]
fn vpunpcklbw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLBW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectDisplaced(RDI, 1980783596, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 96, 151, 236, 91, 16, 118], OperandSize::Qword)
}

#[test]
fn vpunpcklbw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLBW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 96, 214], OperandSize::Dword)
}

#[test]
fn vpunpcklbw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLBW, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM7)), operand3: Some(Indirect(EDI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 96, 47], OperandSize::Dword)
}

#[test]
fn vpunpcklbw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLBW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 96, 192], OperandSize::Qword)
}

#[test]
fn vpunpcklbw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLBW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RDI, Two, 107065020, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 96, 148, 127, 188, 174, 97, 6], OperandSize::Qword)
}

#[test]
fn vpunpcklbw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLBW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 117, 139, 96, 195], OperandSize::Dword)
}

#[test]
fn vpunpcklbw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLBW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectDisplaced(ESI, 1221679264, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 125, 142, 96, 182, 160, 88, 209, 72], OperandSize::Dword)
}

#[test]
fn vpunpcklbw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLBW, operand1: Some(Direct(XMM15)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM13)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 81, 85, 142, 96, 253], OperandSize::Qword)
}

#[test]
fn vpunpcklbw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLBW, operand1: Some(Direct(XMM28)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledDisplaced(RDX, Four, 1325022478, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 97, 69, 142, 96, 36, 149, 14, 61, 250, 78], OperandSize::Qword)
}

