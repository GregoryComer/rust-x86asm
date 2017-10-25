use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn maxpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MAXPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 95, 218], OperandSize::Dword)
}

#[test]
fn maxpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MAXPD, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledDisplaced(EBX, Two, 712671717, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 95, 12, 93, 229, 129, 122, 42], OperandSize::Dword)
}

#[test]
fn maxpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MAXPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 95, 250], OperandSize::Qword)
}

#[test]
fn maxpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MAXPD, operand1: Some(Direct(XMM1)), operand2: Some(Indirect(RAX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 95, 8], OperandSize::Qword)
}

