use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vreducess_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCESS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM5)), operand4: Some(Literal8(89)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 117, 153, 87, 229, 89], OperandSize::Dword)
}

#[test]
fn vreducess_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCESS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexed(ESI, EDX, Four, Some(OperandSize::Dword), None)), operand4: Some(Literal8(100)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 77, 142, 87, 60, 150, 100], OperandSize::Dword)
}

#[test]
fn vreducess_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCESS, operand1: Some(Direct(XMM12)), operand2: Some(Direct(XMM21)), operand3: Some(Direct(XMM16)), operand4: Some(Literal8(103)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K5), broadcast: None }, &[98, 51, 85, 149, 87, 224, 103], OperandSize::Qword)
}

#[test]
fn vreducess_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCESS, operand1: Some(Direct(XMM8)), operand2: Some(Direct(XMM24)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RSI, Eight, 931317641, Some(OperandSize::Dword), None)), operand4: Some(Literal8(99)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 115, 61, 129, 87, 132, 240, 137, 199, 130, 55, 99], OperandSize::Qword)
}

