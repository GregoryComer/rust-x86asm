use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn movmskpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVMSKPD, operand1: Some(Direct(ESI)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 80, 246], OperandSize::Dword)
}

#[test]
fn movmskpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVMSKPD, operand1: Some(Direct(RCX)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 80, 200], OperandSize::Qword)
}

