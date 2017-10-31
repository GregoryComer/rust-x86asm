use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fstcw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FSTCW, operand1: Some(IndirectDisplaced(BP, 13438, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[155, 217, 190, 126, 52], OperandSize::Word)
}

#[test]
fn fstcw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FSTCW, operand1: Some(IndirectScaledIndexedDisplaced(ESI, EAX, Two, 537588244, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[155, 217, 188, 70, 20, 242, 10, 32], OperandSize::Dword)
}

#[test]
fn fstcw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FSTCW, operand1: Some(IndirectDisplaced(RDI, 480907970, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[155, 217, 191, 194, 18, 170, 28], OperandSize::Qword)
}

