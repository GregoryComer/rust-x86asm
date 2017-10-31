use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn addpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::ADDPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 88, 247], OperandSize::Dword)
}

#[test]
fn addpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::ADDPD, operand1: Some(Direct(XMM6)), operand2: Some(Indirect(EDI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 88, 55], OperandSize::Dword)
}

#[test]
fn addpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::ADDPD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 88, 231], OperandSize::Qword)
}

#[test]
fn addpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::ADDPD, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexed(RDX, RAX, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 88, 4, 130], OperandSize::Qword)
}

