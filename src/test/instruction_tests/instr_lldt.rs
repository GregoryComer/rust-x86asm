use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn lldt_1() {
    run_test(&Instruction { mnemonic: Mnemonic::LLDT, operand1: Some(Direct(SI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 0, 214], OperandSize::Word)
}

#[test]
fn lldt_2() {
    run_test(&Instruction { mnemonic: Mnemonic::LLDT, operand1: Some(IndirectScaledIndexed(BP, DI, One, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 0, 19], OperandSize::Word)
}

#[test]
fn lldt_3() {
    run_test(&Instruction { mnemonic: Mnemonic::LLDT, operand1: Some(Direct(BP)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 0, 213], OperandSize::Dword)
}

#[test]
fn lldt_4() {
    run_test(&Instruction { mnemonic: Mnemonic::LLDT, operand1: Some(IndirectDisplaced(EAX, 2014081125, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 0, 144, 101, 112, 12, 120], OperandSize::Dword)
}

#[test]
fn lldt_5() {
    run_test(&Instruction { mnemonic: Mnemonic::LLDT, operand1: Some(Direct(BX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 0, 211], OperandSize::Qword)
}

#[test]
fn lldt_6() {
    run_test(&Instruction { mnemonic: Mnemonic::LLDT, operand1: Some(IndirectDisplaced(RDX, 2069053079, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 0, 146, 151, 62, 83, 123], OperandSize::Qword)
}

