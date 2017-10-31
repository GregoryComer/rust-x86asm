use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn addpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::ADDPD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 88, 195], OperandSize::Dword)
}

#[test]
fn addpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::ADDPD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexed(EAX, ESI, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 88, 36, 240], OperandSize::Dword)
}

#[test]
fn addpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::ADDPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 88, 248], OperandSize::Qword)
}

#[test]
fn addpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::ADDPD, operand1: Some(Direct(XMM4)), operand2: Some(Indirect(RBX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 88, 35], OperandSize::Qword)
}

