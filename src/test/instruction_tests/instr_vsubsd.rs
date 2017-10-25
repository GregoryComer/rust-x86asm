use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vsubsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBSD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 195, 92, 234], OperandSize::Dword)
}

#[test]
fn vsubsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBSD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledDisplaced(EAX, Two, 1074587015, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 235, 92, 44, 69, 135, 229, 12, 64], OperandSize::Dword)
}

#[test]
fn vsubsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBSD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 92, 224], OperandSize::Qword)
}

#[test]
fn vsubsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBSD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(Indirect(RDI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 227, 92, 55], OperandSize::Qword)
}

#[test]
fn vsubsd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBSD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 231, 255, 92, 205], OperandSize::Dword)
}

#[test]
fn vsubsd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBSD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexed(EBX, EBX, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 199, 142, 92, 4, 219], OperandSize::Dword)
}

#[test]
fn vsubsd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBSD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM21)), operand3: Some(Direct(XMM26)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 145, 215, 213, 92, 234], OperandSize::Qword)
}

#[test]
fn vsubsd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBSD, operand1: Some(Direct(XMM12)), operand2: Some(Direct(XMM14)), operand3: Some(IndirectDisplaced(RDX, 992979017, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 113, 143, 142, 92, 162, 73, 168, 47, 59], OperandSize::Qword)
}

