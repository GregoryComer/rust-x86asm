use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pmovzxbq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVZXBQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 50, 203], OperandSize::Dword)
}

#[test]
fn pmovzxbq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVZXBQ, operand1: Some(Direct(XMM7)), operand2: Some(Indirect(ESI, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 50, 62], OperandSize::Dword)
}

#[test]
fn pmovzxbq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVZXBQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 50, 252], OperandSize::Qword)
}

#[test]
fn pmovzxbq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVZXBQ, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledDisplaced(RAX, Four, 840870893, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 50, 52, 133, 237, 171, 30, 50], OperandSize::Qword)
}

