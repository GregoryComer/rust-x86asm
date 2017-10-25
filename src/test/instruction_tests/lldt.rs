use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn lldt_1() {
    run_test(&Instruction { mnemonic: Mnemonic::LLDT, operand1: Some(Direct(DI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 0, 215], OperandSize::Word)
}

fn lldt_2() {
    run_test(&Instruction { mnemonic: Mnemonic::LLDT, operand1: Some(IndirectDisplaced(DI, 69, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 0, 85, 69], OperandSize::Word)
}

fn lldt_3() {
    run_test(&Instruction { mnemonic: Mnemonic::LLDT, operand1: Some(Direct(BP)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 0, 213], OperandSize::Dword)
}

fn lldt_4() {
    run_test(&Instruction { mnemonic: Mnemonic::LLDT, operand1: Some(IndirectDisplaced(EDI, 1542129923, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 0, 151, 3, 9, 235, 91], OperandSize::Dword)
}

fn lldt_5() {
    run_test(&Instruction { mnemonic: Mnemonic::LLDT, operand1: Some(Direct(DI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 0, 215], OperandSize::Qword)
}

fn lldt_6() {
    run_test(&Instruction { mnemonic: Mnemonic::LLDT, operand1: Some(IndirectScaledIndexedDisplaced(RBX, RCX, Two, 601756777, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 0, 148, 75, 105, 20, 222, 35], OperandSize::Qword)
}

