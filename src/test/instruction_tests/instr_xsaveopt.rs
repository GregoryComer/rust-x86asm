use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn xsaveopt_1() {
    run_test(&Instruction { mnemonic: Mnemonic::XSAVEOPT, operand1: Some(IndirectScaledDisplaced(EBX, Two, 1759191567, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 174, 52, 93, 15, 34, 219, 104], OperandSize::Dword)
}

#[test]
fn xsaveopt_2() {
    run_test(&Instruction { mnemonic: Mnemonic::XSAVEOPT, operand1: Some(IndirectScaledIndexedDisplaced(RBX, RDX, Eight, 1405198881, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 174, 180, 211, 33, 162, 193, 83], OperandSize::Qword)
}

