use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn clflushopt_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CLFLUSHOPT, operand1: Some(Indirect(DI, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 174, 61], OperandSize::Word)
}

fn clflushopt_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CLFLUSHOPT, operand1: Some(IndirectDisplaced(EBX, 1746983053, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 174, 187, 141, 216, 32, 104], OperandSize::Dword)
}

fn clflushopt_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CLFLUSHOPT, operand1: Some(Indirect(RCX, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 174, 57], OperandSize::Qword)
}

