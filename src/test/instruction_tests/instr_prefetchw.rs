use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn prefetchw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PREFETCHW, operand1: Some(IndirectDisplaced(EDX, 1328897990, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 13, 138, 198, 95, 53, 79], OperandSize::Dword)
}

#[test]
fn prefetchw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PREFETCHW, operand1: Some(IndirectScaledDisplaced(RCX, Two, 983839721, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 13, 12, 77, 233, 51, 164, 58], OperandSize::Qword)
}

