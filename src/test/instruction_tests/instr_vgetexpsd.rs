use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vgetexpsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPSD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 221, 157, 67, 220], OperandSize::Dword)
}

#[test]
fn vgetexpsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPSD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EDI, Eight, 524314641, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 205, 143, 67, 180, 251, 17, 104, 64, 31], OperandSize::Dword)
}

#[test]
fn vgetexpsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPSD, operand1: Some(Direct(XMM31)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K3), broadcast: None }, &[98, 98, 197, 155, 67, 254], OperandSize::Qword)
}

#[test]
fn vgetexpsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPSD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RBX, Two, 1416930450, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 197, 137, 67, 164, 89, 146, 164, 116, 84], OperandSize::Qword)
}

