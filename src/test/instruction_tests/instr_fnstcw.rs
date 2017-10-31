use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fnstcw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FNSTCW, operand1: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 32360, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[217, 184, 104, 126], OperandSize::Word)
}

#[test]
fn fnstcw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FNSTCW, operand1: Some(IndirectScaledDisplaced(EDI, Eight, 438582450, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[217, 60, 253, 178, 60, 36, 26], OperandSize::Dword)
}

#[test]
fn fnstcw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FNSTCW, operand1: Some(IndirectDisplaced(RAX, 1852331748, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[217, 184, 228, 86, 104, 110], OperandSize::Qword)
}

