use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vreducess_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCESS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM0)), operand4: Some(Literal8(52)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 93, 156, 87, 216, 52], OperandSize::Dword)
}

#[test]
fn vreducess_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCESS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(Indirect(ECX, Some(OperandSize::Dword), None)), operand4: Some(Literal8(82)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 125, 137, 87, 49, 82], OperandSize::Dword)
}

#[test]
fn vreducess_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCESS, operand1: Some(Direct(XMM31)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM6)), operand4: Some(Literal8(55)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K2), broadcast: None }, &[98, 99, 125, 154, 87, 254, 55], OperandSize::Qword)
}

#[test]
fn vreducess_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCESS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM30)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RCX, Two, 2124832671, Some(OperandSize::Dword), None)), operand4: Some(Literal8(126)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 13, 132, 87, 148, 79, 159, 95, 166, 126, 126], OperandSize::Qword)
}

