use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vsqrtss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTSS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 234, 81, 208], OperandSize::Dword)
}

#[test]
fn vsqrtss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTSS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledDisplaced(ESI, Two, 3799683, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 226, 81, 4, 117, 131, 250, 57, 0], OperandSize::Dword)
}

#[test]
fn vsqrtss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTSS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 210, 81, 194], OperandSize::Qword)
}

#[test]
fn vsqrtss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTSS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RBX, Eight, 1794375662, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 202, 81, 188, 222, 238, 255, 243, 106], OperandSize::Qword)
}

#[test]
fn vsqrtss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTSS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 94, 154, 81, 246], OperandSize::Dword)
}

#[test]
fn vsqrtss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTSS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexed(ESI, EAX, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 110, 137, 81, 44, 198], OperandSize::Dword)
}

#[test]
fn vsqrtss_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTSS, operand1: Some(Direct(XMM21)), operand2: Some(Direct(XMM10)), operand3: Some(Direct(XMM18)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 161, 46, 217, 81, 234], OperandSize::Qword)
}

#[test]
fn vsqrtss_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTSS, operand1: Some(Direct(XMM25)), operand2: Some(Direct(XMM14)), operand3: Some(IndirectScaledDisplaced(RSI, Two, 1475163134, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 97, 14, 143, 81, 12, 117, 254, 51, 237, 87], OperandSize::Qword)
}

