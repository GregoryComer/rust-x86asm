use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pmovzxbw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVZXBW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 48, 229], OperandSize::Dword)
}

#[test]
fn pmovzxbw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVZXBW, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexed(EAX, EBX, Four, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 48, 28, 152], OperandSize::Dword)
}

#[test]
fn pmovzxbw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVZXBW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 48, 220], OperandSize::Qword)
}

#[test]
fn pmovzxbw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVZXBW, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexed(RDI, RDX, Eight, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 48, 52, 215], OperandSize::Qword)
}

