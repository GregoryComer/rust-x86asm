use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn lldt_1() {
    run_test(&Instruction { mnemonic: Mnemonic::LLDT, operand1: Some(Direct(DX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 0, 210], OperandSize::Word)
}

#[test]
fn lldt_2() {
    run_test(&Instruction { mnemonic: Mnemonic::LLDT, operand1: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 21224, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 0, 145, 232, 82], OperandSize::Word)
}

#[test]
fn lldt_3() {
    run_test(&Instruction { mnemonic: Mnemonic::LLDT, operand1: Some(Direct(SI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 0, 214], OperandSize::Dword)
}

#[test]
fn lldt_4() {
    run_test(&Instruction { mnemonic: Mnemonic::LLDT, operand1: Some(IndirectDisplaced(EDI, 719932067, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 0, 151, 163, 74, 233, 42], OperandSize::Dword)
}

#[test]
fn lldt_5() {
    run_test(&Instruction { mnemonic: Mnemonic::LLDT, operand1: Some(Direct(SP)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 0, 212], OperandSize::Qword)
}

#[test]
fn lldt_6() {
    run_test(&Instruction { mnemonic: Mnemonic::LLDT, operand1: Some(IndirectScaledIndexed(RDI, RCX, Eight, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 0, 20, 207], OperandSize::Qword)
}

