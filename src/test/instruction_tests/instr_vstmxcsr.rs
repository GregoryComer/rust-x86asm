use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vstmxcsr_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VSTMXCSR, operand1: Some(IndirectScaledIndexedDisplaced(EAX, EDI, Four, 206176944, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 174, 156, 184, 176, 2, 74, 12], OperandSize::Dword)
}

#[test]
fn vstmxcsr_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VSTMXCSR, operand1: Some(IndirectScaledIndexed(RAX, RSI, Two, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 174, 28, 112], OperandSize::Qword)
}

