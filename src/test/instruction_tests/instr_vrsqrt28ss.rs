use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vrsqrt28ss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT28SS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 85, 155, 205, 247], OperandSize::Dword)
}

#[test]
fn vrsqrt28ss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT28SS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexed(EAX, ECX, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 77, 137, 205, 28, 200], OperandSize::Dword)
}

#[test]
fn vrsqrt28ss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT28SS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM13)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K7), broadcast: None }, &[98, 210, 109, 159, 205, 197], OperandSize::Qword)
}

#[test]
fn vrsqrt28ss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT28SS, operand1: Some(Direct(XMM24)), operand2: Some(Direct(XMM27)), operand3: Some(IndirectDisplaced(RAX, 1667909160, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 98, 37, 130, 205, 128, 40, 70, 106, 99], OperandSize::Qword)
}

