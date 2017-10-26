use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vrsqrtss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRTSS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 194, 82, 220], OperandSize::Dword)
}

#[test]
fn vrsqrtss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRTSS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledDisplaced(EAX, Eight, 1433712933, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 202, 82, 28, 197, 37, 185, 116, 85], OperandSize::Dword)
}

#[test]
fn vrsqrtss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRTSS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 226, 82, 254], OperandSize::Qword)
}

#[test]
fn vrsqrtss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRTSS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledDisplaced(RDX, Two, 223863242, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 82, 20, 85, 202, 225, 87, 13], OperandSize::Qword)
}

