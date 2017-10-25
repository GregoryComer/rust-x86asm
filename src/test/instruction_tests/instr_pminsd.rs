use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pminsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINSD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 57, 248], OperandSize::Dword)
}

#[test]
fn pminsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINSD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledDisplaced(EDI, Two, 43119999, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 57, 36, 125, 127, 245, 145, 2], OperandSize::Dword)
}

#[test]
fn pminsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINSD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 57, 238], OperandSize::Qword)
}

#[test]
fn pminsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINSD, operand1: Some(Direct(XMM5)), operand2: Some(Indirect(RCX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 57, 41], OperandSize::Qword)
}

