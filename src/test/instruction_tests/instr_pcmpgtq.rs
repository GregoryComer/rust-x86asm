use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pcmpgtq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPGTQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 55, 219], OperandSize::Dword)
}

#[test]
fn pcmpgtq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPGTQ, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledDisplaced(ESI, Four, 382703249, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 55, 60, 181, 145, 150, 207, 22], OperandSize::Dword)
}

#[test]
fn pcmpgtq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPGTQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 55, 233], OperandSize::Qword)
}

#[test]
fn pcmpgtq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPGTQ, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexed(RDX, RCX, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 55, 12, 74], OperandSize::Qword)
}

