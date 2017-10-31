use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn lldt_1() {
    run_test(&Instruction { mnemonic: Mnemonic::LLDT, operand1: Some(Direct(DI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 0, 215], OperandSize::Word)
}

#[test]
fn lldt_2() {
    run_test(&Instruction { mnemonic: Mnemonic::LLDT, operand1: Some(Memory(26471, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 0, 22, 103, 103], OperandSize::Word)
}

#[test]
fn lldt_3() {
    run_test(&Instruction { mnemonic: Mnemonic::LLDT, operand1: Some(Direct(DX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 0, 210], OperandSize::Dword)
}

#[test]
fn lldt_4() {
    run_test(&Instruction { mnemonic: Mnemonic::LLDT, operand1: Some(IndirectScaledDisplaced(EAX, Two, 1481955556, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 0, 20, 69, 228, 216, 84, 88], OperandSize::Dword)
}

#[test]
fn lldt_5() {
    run_test(&Instruction { mnemonic: Mnemonic::LLDT, operand1: Some(Direct(SP)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 0, 212], OperandSize::Qword)
}

#[test]
fn lldt_6() {
    run_test(&Instruction { mnemonic: Mnemonic::LLDT, operand1: Some(IndirectScaledDisplaced(RBX, Two, 587078233, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 0, 20, 93, 89, 26, 254, 34], OperandSize::Qword)
}

