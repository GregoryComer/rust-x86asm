use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pmovzxbd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVZXBD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 49, 243], OperandSize::Dword)
}

#[test]
fn pmovzxbd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVZXBD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, EAX, Eight, 1975313903, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 49, 164, 198, 239, 229, 188, 117], OperandSize::Dword)
}

#[test]
fn pmovzxbd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVZXBD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 49, 224], OperandSize::Qword)
}

#[test]
fn pmovzxbd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVZXBD, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledDisplaced(RAX, Eight, 814317687, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 49, 20, 197, 119, 128, 137, 48], OperandSize::Qword)
}

