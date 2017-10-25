use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmsub132ss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132SS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 155, 241], OperandSize::Dword)
}

#[test]
fn vfmsub132ss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132SS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, ESI, Two, 1783897935, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 155, 156, 114, 79, 31, 84, 106], OperandSize::Dword)
}

#[test]
fn vfmsub132ss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132SS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 155, 244], OperandSize::Qword)
}

#[test]
fn vfmsub132ss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132SS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RDX, Eight, 1989861644, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 155, 148, 209, 12, 225, 154, 118], OperandSize::Qword)
}

#[test]
fn vfmsub132ss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132SS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 85, 190, 155, 238], OperandSize::Dword)
}

#[test]
fn vfmsub132ss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132SS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexed(EDI, EBX, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 93, 137, 155, 12, 223], OperandSize::Dword)
}

#[test]
fn vfmsub132ss_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132SS, operand1: Some(Direct(XMM23)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 226, 93, 189, 155, 253], OperandSize::Qword)
}

#[test]
fn vfmsub132ss_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132SS, operand1: Some(Direct(XMM28)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RCX, Two, 1588368644, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 98, 69, 137, 155, 164, 79, 4, 149, 172, 94], OperandSize::Qword)
}

