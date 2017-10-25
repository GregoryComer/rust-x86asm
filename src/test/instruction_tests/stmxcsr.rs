use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn stmxcsr_1() {
    run_test(&Instruction { mnemonic: Mnemonic::STMXCSR, operand1: Some(IndirectScaledIndexedDisplaced(EDI, EAX, Eight, 1740141887, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 174, 156, 199, 63, 117, 184, 103], OperandSize::Dword)
}

fn stmxcsr_2() {
    run_test(&Instruction { mnemonic: Mnemonic::STMXCSR, operand1: Some(IndirectScaledIndexed(RCX, RCX, Two, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 174, 28, 73], OperandSize::Qword)
}

