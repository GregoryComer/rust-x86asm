use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pmovzxdq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVZXDQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 53, 250], OperandSize::Dword)
}

#[test]
fn pmovzxdq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVZXDQ, operand1: Some(Direct(XMM3)), operand2: Some(IndirectDisplaced(ESI, 1284680230, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 53, 158, 38, 170, 146, 76], OperandSize::Dword)
}

#[test]
fn pmovzxdq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVZXDQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 53, 251], OperandSize::Qword)
}

#[test]
fn pmovzxdq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVZXDQ, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RDI, Eight, 983885691, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 53, 148, 250, 123, 231, 164, 58], OperandSize::Qword)
}

