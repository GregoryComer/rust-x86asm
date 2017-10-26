use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn stmxcsr_1() {
    run_test(&Instruction { mnemonic: Mnemonic::STMXCSR, operand1: Some(IndirectScaledIndexed(EDI, EBX, Four, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 174, 28, 159], OperandSize::Dword)
}

#[test]
fn stmxcsr_2() {
    run_test(&Instruction { mnemonic: Mnemonic::STMXCSR, operand1: Some(IndirectScaledIndexedDisplaced(RDI, RDX, Two, 1114929468, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 174, 156, 87, 60, 121, 116, 66], OperandSize::Qword)
}

