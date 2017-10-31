use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vminsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINSD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 93, 209], OperandSize::Dword)
}

#[test]
fn vminsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINSD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: Some(Indirect(EAX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 243, 93, 32], OperandSize::Dword)
}

#[test]
fn vminsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINSD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 211, 93, 228], OperandSize::Qword)
}

#[test]
fn vminsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINSD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectDisplaced(RBX, 949161935, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 211, 93, 139, 207, 15, 147, 56], OperandSize::Qword)
}

#[test]
fn vminsd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINSD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 199, 158, 93, 197], OperandSize::Dword)
}

#[test]
fn vminsd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINSD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledDisplaced(EDI, Eight, 1607357601, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 239, 140, 93, 60, 253, 161, 84, 206, 95], OperandSize::Dword)
}

#[test]
fn vminsd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINSD, operand1: Some(Direct(XMM31)), operand2: Some(Direct(XMM16)), operand3: Some(Direct(XMM12)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K4), broadcast: None }, &[98, 65, 255, 148, 93, 252], OperandSize::Qword)
}

#[test]
fn vminsd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINSD, operand1: Some(Direct(XMM27)), operand2: Some(Direct(XMM21)), operand3: Some(IndirectDisplaced(RBX, 1429892969, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 97, 215, 133, 93, 155, 105, 111, 58, 85], OperandSize::Qword)
}

