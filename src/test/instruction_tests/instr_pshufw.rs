use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pshufw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PSHUFW, operand1: Some(Direct(MM3)), operand2: Some(Direct(MM1)), operand3: Some(Literal8(97)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 112, 217, 97], OperandSize::Word)
}

#[test]
fn pshufw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PSHUFW, operand1: Some(Direct(MM2)), operand2: Some(Indirect(BX, Some(OperandSize::Qword), None)), operand3: Some(Literal8(22)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 112, 23, 22], OperandSize::Word)
}

#[test]
fn pshufw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PSHUFW, operand1: Some(Direct(MM5)), operand2: Some(Direct(MM6)), operand3: Some(Literal8(90)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 112, 238, 90], OperandSize::Dword)
}

#[test]
fn pshufw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PSHUFW, operand1: Some(Direct(MM7)), operand2: Some(IndirectDisplaced(EBX, 1515041433, Some(OperandSize::Qword), None)), operand3: Some(Literal8(93)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 112, 187, 153, 178, 77, 90, 93], OperandSize::Dword)
}

#[test]
fn pshufw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PSHUFW, operand1: Some(Direct(MM2)), operand2: Some(Direct(MM2)), operand3: Some(Literal8(43)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 112, 210, 43], OperandSize::Qword)
}

#[test]
fn pshufw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PSHUFW, operand1: Some(Direct(MM6)), operand2: Some(IndirectDisplaced(RAX, 1114894598, Some(OperandSize::Qword), None)), operand3: Some(Literal8(14)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 112, 176, 6, 241, 115, 66, 14], OperandSize::Qword)
}

