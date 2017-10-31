use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vscalefss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFSS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 117, 158, 45, 231], OperandSize::Dword)
}

#[test]
fn vscalefss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFSS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexed(EDI, EAX, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 85, 137, 45, 20, 199], OperandSize::Dword)
}

#[test]
fn vscalefss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFSS, operand1: Some(Direct(XMM31)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM8)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 66, 77, 249, 45, 248], OperandSize::Qword)
}

#[test]
fn vscalefss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFSS, operand1: Some(Direct(XMM10)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RCX, Two, 219343214, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 114, 77, 138, 45, 148, 73, 110, 233, 18, 13], OperandSize::Qword)
}

