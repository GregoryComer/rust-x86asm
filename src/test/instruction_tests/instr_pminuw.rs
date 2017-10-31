use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pminuw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINUW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 58, 192], OperandSize::Dword)
}

#[test]
fn pminuw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINUW, operand1: Some(Direct(XMM2)), operand2: Some(IndirectDisplaced(ECX, 1869076827, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 58, 145, 91, 217, 103, 111], OperandSize::Dword)
}

#[test]
fn pminuw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINUW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 58, 226], OperandSize::Qword)
}

#[test]
fn pminuw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINUW, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledDisplaced(RDX, Two, 406770384, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 58, 36, 85, 208, 210, 62, 24], OperandSize::Qword)
}

