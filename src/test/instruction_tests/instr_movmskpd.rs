use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn movmskpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVMSKPD, operand1: Some(Direct(EDI)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 80, 249], OperandSize::Dword)
}

#[test]
fn movmskpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVMSKPD, operand1: Some(Direct(RSI)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 80, 241], OperandSize::Qword)
}

