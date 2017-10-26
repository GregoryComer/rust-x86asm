use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn str_1() {
    run_test(&Instruction { mnemonic: Mnemonic::STR, operand1: Some(Direct(SP)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 0, 204], OperandSize::Word)
}

#[test]
fn str_2() {
    run_test(&Instruction { mnemonic: Mnemonic::STR, operand1: Some(IndirectDisplaced(SI, 19124, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 0, 140, 180, 74], OperandSize::Word)
}

#[test]
fn str_3() {
    run_test(&Instruction { mnemonic: Mnemonic::STR, operand1: Some(Direct(CX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 0, 201], OperandSize::Dword)
}

#[test]
fn str_4() {
    run_test(&Instruction { mnemonic: Mnemonic::STR, operand1: Some(IndirectDisplaced(ECX, 1319423944, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 0, 137, 200, 207, 164, 78], OperandSize::Dword)
}

#[test]
fn str_5() {
    run_test(&Instruction { mnemonic: Mnemonic::STR, operand1: Some(Direct(SP)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 0, 204], OperandSize::Qword)
}

#[test]
fn str_6() {
    run_test(&Instruction { mnemonic: Mnemonic::STR, operand1: Some(IndirectDisplaced(RDI, 590516136, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 0, 143, 168, 143, 50, 35], OperandSize::Qword)
}

