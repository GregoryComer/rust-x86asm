use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn sidt_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SIDT, operand1: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 14726, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 1, 139, 134, 57], OperandSize::Word)
}

fn sidt_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SIDT, operand1: Some(IndirectScaledIndexed(EDX, EAX, Four, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 1, 12, 130], OperandSize::Dword)
}

fn sidt_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SIDT, operand1: Some(IndirectScaledIndexedDisplaced(RBX, RCX, Eight, 62276479, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 1, 140, 203, 127, 67, 182, 3], OperandSize::Qword)
}

