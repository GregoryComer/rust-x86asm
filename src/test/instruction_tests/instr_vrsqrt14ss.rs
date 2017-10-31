use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vrsqrt14ss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14SS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 93, 137, 79, 217], OperandSize::Dword)
}

#[test]
fn vrsqrt14ss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14SS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, EBX, Two, 1122342621, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 77, 139, 79, 140, 89, 221, 150, 229, 66], OperandSize::Dword)
}

#[test]
fn vrsqrt14ss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14SS, operand1: Some(Direct(XMM16)), operand2: Some(Direct(XMM10)), operand3: Some(Direct(XMM19)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 162, 45, 137, 79, 195], OperandSize::Qword)
}

#[test]
fn vrsqrt14ss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14SS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectDisplaced(RAX, 1609391315, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 77, 139, 79, 136, 211, 92, 237, 95], OperandSize::Qword)
}

