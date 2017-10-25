use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn sqrtsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SQRTSD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 81, 221], OperandSize::Dword)
}

#[test]
fn sqrtsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SQRTSD, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexed(EAX, EDX, Eight, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 81, 44, 208], OperandSize::Dword)
}

#[test]
fn sqrtsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SQRTSD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 81, 211], OperandSize::Qword)
}

#[test]
fn sqrtsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SQRTSD, operand1: Some(Direct(XMM2)), operand2: Some(IndirectDisplaced(RDX, 301294489, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 81, 146, 153, 99, 245, 17], OperandSize::Qword)
}

