use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vminss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINSS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 93, 250], OperandSize::Dword)
}

#[test]
fn vminss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINSS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledDisplaced(ESI, Four, 1195012584, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 226, 93, 52, 181, 232, 113, 58, 71], OperandSize::Dword)
}

#[test]
fn vminss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINSS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 210, 93, 198], OperandSize::Qword)
}

#[test]
fn vminss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINSS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexed(RDI, RDX, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 194, 93, 12, 151], OperandSize::Qword)
}

#[test]
fn vminss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINSS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 102, 155, 93, 194], OperandSize::Dword)
}

#[test]
fn vminss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINSS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: Some(Indirect(ESI, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 78, 140, 93, 14], OperandSize::Dword)
}

#[test]
fn vminss_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINSS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM13)), operand3: Some(Direct(XMM10)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K6), broadcast: None }, &[98, 209, 22, 158, 93, 226], OperandSize::Qword)
}

#[test]
fn vminss_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINSS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexed(RBX, RSI, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 126, 141, 93, 36, 179], OperandSize::Qword)
}

