use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pmovzxwq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVZXWQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 52, 216], OperandSize::Dword)
}

#[test]
fn pmovzxwq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVZXWQ, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexed(EDI, EAX, Eight, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 52, 36, 199], OperandSize::Dword)
}

#[test]
fn pmovzxwq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVZXWQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 52, 212], OperandSize::Qword)
}

#[test]
fn pmovzxwq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVZXWQ, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RDI, Four, 1456070866, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 52, 180, 184, 210, 224, 201, 86], OperandSize::Qword)
}

