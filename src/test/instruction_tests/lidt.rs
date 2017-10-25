use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn lidt_1() {
    run_test(&Instruction { mnemonic: Mnemonic::LIDT, operand1: Some(IndirectScaledIndexed(BP, SI, One, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 1, 26], OperandSize::Word)
}

fn lidt_2() {
    run_test(&Instruction { mnemonic: Mnemonic::LIDT, operand1: Some(IndirectScaledDisplaced(ESI, Two, 1586314753, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 1, 28, 117, 1, 62, 141, 94], OperandSize::Dword)
}

fn lidt_3() {
    run_test(&Instruction { mnemonic: Mnemonic::LIDT, operand1: Some(IndirectScaledIndexedDisplaced(RCX, RAX, Eight, 410038951, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 1, 156, 193, 167, 178, 112, 24], OperandSize::Qword)
}

