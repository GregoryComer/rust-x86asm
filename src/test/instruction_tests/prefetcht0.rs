use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn prefetcht0_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PREFETCHT0, operand1: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 27732, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 24, 139, 84, 108], OperandSize::Word)
}

fn prefetcht0_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PREFETCHT0, operand1: Some(IndirectScaledDisplaced(EDX, Eight, 1078670835, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 24, 12, 213, 243, 53, 75, 64], OperandSize::Dword)
}

fn prefetcht0_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PREFETCHT0, operand1: Some(Indirect(RSI, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 24, 14], OperandSize::Qword)
}

