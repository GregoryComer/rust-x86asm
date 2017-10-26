use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fnstcw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FNSTCW, operand1: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 169, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[217, 187, 169, 0], OperandSize::Word)
}

#[test]
fn fnstcw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FNSTCW, operand1: Some(IndirectScaledIndexed(ECX, EDI, Eight, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[217, 60, 249], OperandSize::Dword)
}

#[test]
fn fnstcw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FNSTCW, operand1: Some(IndirectScaledIndexedDisplaced(RDI, RBX, Eight, 1147975121, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[217, 188, 223, 209, 181, 108, 68], OperandSize::Qword)
}

