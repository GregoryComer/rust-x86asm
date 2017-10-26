use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vrangess_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGESS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM4)), operand4: Some(Literal8(122)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 69, 157, 81, 228, 122], OperandSize::Dword)
}

#[test]
fn vrangess_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGESS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(EDX, 1487967169, Some(OperandSize::Dword), None)), operand4: Some(Literal8(60)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 117, 138, 81, 130, 193, 147, 176, 88, 60], OperandSize::Dword)
}

#[test]
fn vrangess_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGESS, operand1: Some(Direct(XMM22)), operand2: Some(Direct(XMM30)), operand3: Some(Direct(XMM0)), operand4: Some(Literal8(89)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K4), broadcast: None }, &[98, 227, 13, 148, 81, 240, 89], OperandSize::Qword)
}

#[test]
fn vrangess_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGESS, operand1: Some(Direct(XMM8)), operand2: Some(Direct(XMM17)), operand3: Some(IndirectScaledDisplaced(RAX, Four, 1240019800, Some(OperandSize::Dword), None)), operand4: Some(Literal8(34)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 115, 117, 134, 81, 4, 133, 88, 51, 233, 73, 34], OperandSize::Qword)
}

