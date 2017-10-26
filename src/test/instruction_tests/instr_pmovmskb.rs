use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pmovmskb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVMSKB, operand1: Some(Direct(EBP)), operand2: Some(Direct(MM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 215, 236], OperandSize::Dword)
}

#[test]
fn pmovmskb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVMSKB, operand1: Some(Direct(RDX)), operand2: Some(Direct(MM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 215, 214], OperandSize::Qword)
}

#[test]
fn pmovmskb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVMSKB, operand1: Some(Direct(EDI)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 215, 254], OperandSize::Dword)
}

#[test]
fn pmovmskb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVMSKB, operand1: Some(Direct(RSI)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 215, 244], OperandSize::Qword)
}

