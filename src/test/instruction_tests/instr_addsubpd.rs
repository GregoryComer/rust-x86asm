use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn addsubpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::ADDSUBPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 208, 213], OperandSize::Dword)
}

#[test]
fn addsubpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::ADDSUBPD, operand1: Some(Direct(XMM2)), operand2: Some(Indirect(EDI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 208, 23], OperandSize::Dword)
}

#[test]
fn addsubpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::ADDSUBPD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 208, 227], OperandSize::Qword)
}

#[test]
fn addsubpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::ADDSUBPD, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledDisplaced(RBX, Two, 1278908911, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 208, 52, 93, 239, 153, 58, 76], OperandSize::Qword)
}

