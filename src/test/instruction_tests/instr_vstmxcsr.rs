use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vstmxcsr_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VSTMXCSR, operand1: Some(Indirect(ESI, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 174, 30], OperandSize::Dword)
}

#[test]
fn vstmxcsr_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VSTMXCSR, operand1: Some(IndirectDisplaced(RAX, 1594218384, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 174, 152, 144, 215, 5, 95], OperandSize::Qword)
}

