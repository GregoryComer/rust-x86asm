use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn prefetcht2_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PREFETCHT2, operand1: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 18665, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 24, 152, 233, 72], OperandSize::Word)
}

fn prefetcht2_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PREFETCHT2, operand1: Some(IndirectScaledIndexedDisplaced(EDI, EAX, Eight, 1715495440, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 24, 156, 199, 16, 98, 64, 102], OperandSize::Dword)
}

fn prefetcht2_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PREFETCHT2, operand1: Some(Indirect(RAX, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 24, 24], OperandSize::Qword)
}

