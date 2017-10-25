use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pminuw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINUW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 58, 218], OperandSize::Dword)
}

#[test]
fn pminuw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINUW, operand1: Some(Direct(XMM0)), operand2: Some(IndirectDisplaced(EDI, 226474773, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 58, 135, 21, 187, 127, 13], OperandSize::Dword)
}

#[test]
fn pminuw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINUW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 58, 250], OperandSize::Qword)
}

#[test]
fn pminuw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINUW, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledDisplaced(RBX, Eight, 1228597335, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 58, 52, 221, 87, 232, 58, 73], OperandSize::Qword)
}

