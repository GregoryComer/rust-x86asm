use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pmovmskb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVMSKB, operand1: Some(Direct(EBX)), operand2: Some(Direct(MM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 215, 223], OperandSize::Dword)
}

#[test]
fn pmovmskb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVMSKB, operand1: Some(Direct(RCX)), operand2: Some(Direct(MM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 215, 206], OperandSize::Qword)
}

#[test]
fn pmovmskb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVMSKB, operand1: Some(Direct(EDX)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 215, 210], OperandSize::Dword)
}

#[test]
fn pmovmskb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVMSKB, operand1: Some(Direct(RBX)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 215, 216], OperandSize::Qword)
}

