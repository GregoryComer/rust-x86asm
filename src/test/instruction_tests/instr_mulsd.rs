use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn mulsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MULSD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 89, 219], OperandSize::Dword)
}

#[test]
fn mulsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MULSD, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexed(EDX, EAX, Eight, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 89, 28, 194], OperandSize::Dword)
}

#[test]
fn mulsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MULSD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 89, 236], OperandSize::Qword)
}

#[test]
fn mulsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MULSD, operand1: Some(Direct(XMM3)), operand2: Some(Indirect(RDX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 89, 26], OperandSize::Qword)
}

