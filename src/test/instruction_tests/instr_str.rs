use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn str_1() {
    run_test(&Instruction { mnemonic: Mnemonic::STR, operand1: Some(Direct(SI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 0, 206], OperandSize::Word)
}

#[test]
fn str_2() {
    run_test(&Instruction { mnemonic: Mnemonic::STR, operand1: Some(Indirect(BX, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 0, 15], OperandSize::Word)
}

#[test]
fn str_3() {
    run_test(&Instruction { mnemonic: Mnemonic::STR, operand1: Some(Direct(SI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 0, 206], OperandSize::Dword)
}

#[test]
fn str_4() {
    run_test(&Instruction { mnemonic: Mnemonic::STR, operand1: Some(IndirectDisplaced(EAX, 1099014402, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 0, 136, 2, 161, 129, 65], OperandSize::Dword)
}

#[test]
fn str_5() {
    run_test(&Instruction { mnemonic: Mnemonic::STR, operand1: Some(Direct(SI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 0, 206], OperandSize::Qword)
}

#[test]
fn str_6() {
    run_test(&Instruction { mnemonic: Mnemonic::STR, operand1: Some(IndirectScaledDisplaced(RBX, Two, 508640028, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 0, 12, 93, 28, 59, 81, 30], OperandSize::Qword)
}

