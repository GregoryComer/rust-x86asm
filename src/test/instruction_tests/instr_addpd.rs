use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn addpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::ADDPD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 88, 207], OperandSize::Dword)
}

#[test]
fn addpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::ADDPD, operand1: Some(Direct(XMM2)), operand2: Some(Indirect(EAX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 88, 16], OperandSize::Dword)
}

#[test]
fn addpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::ADDPD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 88, 233], OperandSize::Qword)
}

#[test]
fn addpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::ADDPD, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RDI, Eight, 1088330912, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 88, 148, 250, 160, 156, 222, 64], OperandSize::Qword)
}

