use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vminss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINSS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 242, 93, 245], OperandSize::Dword)
}

#[test]
fn vminss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINSS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EDX, Two, 399107775, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 234, 93, 188, 80, 191, 230, 201, 23], OperandSize::Dword)
}

#[test]
fn vminss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINSS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 194, 93, 254], OperandSize::Qword)
}

#[test]
fn vminss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINSS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: Some(Indirect(RAX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 93, 0], OperandSize::Qword)
}

#[test]
fn vminss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINSS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 86, 158, 93, 193], OperandSize::Dword)
}

#[test]
fn vminss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINSS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: Some(Indirect(ESI, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 78, 142, 93, 14], OperandSize::Dword)
}

#[test]
fn vminss_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINSS, operand1: Some(Direct(XMM10)), operand2: Some(Direct(XMM14)), operand3: Some(Direct(XMM21)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K2), broadcast: None }, &[98, 49, 14, 154, 93, 213], OperandSize::Qword)
}

#[test]
fn vminss_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINSS, operand1: Some(Direct(XMM24)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledDisplaced(RDX, Four, 997301508, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 97, 118, 140, 93, 4, 149, 4, 157, 113, 59], OperandSize::Qword)
}

