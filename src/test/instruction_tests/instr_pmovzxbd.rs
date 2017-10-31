use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pmovzxbd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVZXBD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 49, 248], OperandSize::Dword)
}

#[test]
fn pmovzxbd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVZXBD, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledDisplaced(ESI, Eight, 1420993054, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 49, 52, 245, 30, 162, 178, 84], OperandSize::Dword)
}

#[test]
fn pmovzxbd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVZXBD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 49, 208], OperandSize::Qword)
}

#[test]
fn pmovzxbd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVZXBD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RBX, Two, 369925517, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 49, 164, 88, 141, 157, 12, 22], OperandSize::Qword)
}

