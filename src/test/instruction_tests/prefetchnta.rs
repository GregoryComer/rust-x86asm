use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn prefetchnta_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PREFETCHNTA, operand1: Some(Memory(4956, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 24, 6, 92, 19], OperandSize::Word)
}

fn prefetchnta_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PREFETCHNTA, operand1: Some(IndirectScaledIndexed(EDX, ECX, Eight, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 24, 4, 202], OperandSize::Dword)
}

fn prefetchnta_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PREFETCHNTA, operand1: Some(Indirect(RBX, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 24, 3], OperandSize::Qword)
}

