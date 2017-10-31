use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pmaxud_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXUD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 63, 216], OperandSize::Dword)
}

#[test]
fn pmaxud_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXUD, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, EAX, Two, 135125831, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 63, 188, 66, 71, 219, 13, 8], OperandSize::Dword)
}

#[test]
fn pmaxud_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXUD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 63, 200], OperandSize::Qword)
}

#[test]
fn pmaxud_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXUD, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RDX, Eight, 364821197, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 63, 180, 210, 205, 186, 190, 21], OperandSize::Qword)
}

