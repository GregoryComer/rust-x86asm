use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vstmxcsr_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VSTMXCSR, operand1: Some(IndirectScaledDisplaced(EAX, Eight, 1989024879, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 174, 28, 197, 111, 28, 142, 118], OperandSize::Dword)
}

#[test]
fn vstmxcsr_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VSTMXCSR, operand1: Some(IndirectScaledIndexedDisplaced(RDI, RCX, Eight, 1403853736, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 174, 156, 207, 168, 27, 173, 83], OperandSize::Qword)
}

