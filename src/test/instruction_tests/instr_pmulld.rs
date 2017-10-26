use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pmulld_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULLD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 64, 202], OperandSize::Dword)
}

#[test]
fn pmulld_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULLD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, ESI, Eight, 359224990, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 64, 164, 243, 158, 86, 105, 21], OperandSize::Dword)
}

#[test]
fn pmulld_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULLD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 64, 220], OperandSize::Qword)
}

#[test]
fn pmulld_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULLD, operand1: Some(Direct(XMM2)), operand2: Some(IndirectDisplaced(RDX, 1241219731, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 64, 146, 147, 130, 251, 73], OperandSize::Qword)
}

