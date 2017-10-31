use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn sgdt_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SGDT, operand1: Some(IndirectDisplaced(BP, 47, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 1, 70, 47], OperandSize::Word)
}

#[test]
fn sgdt_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SGDT, operand1: Some(IndirectScaledIndexedDisplaced(EDX, ECX, Two, 511376979, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 1, 132, 74, 83, 254, 122, 30], OperandSize::Dword)
}

#[test]
fn sgdt_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SGDT, operand1: Some(IndirectScaledIndexedDisplaced(RDX, RCX, Eight, 662100125, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 1, 132, 202, 157, 216, 118, 39], OperandSize::Qword)
}

