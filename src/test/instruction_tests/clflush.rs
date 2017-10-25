use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn clflush_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CLFLUSH, operand1: Some(IndirectDisplaced(BP, 2096, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 174, 190, 48, 8], OperandSize::Word)
}

fn clflush_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CLFLUSH, operand1: Some(IndirectScaledIndexedDisplaced(EBX, EDI, Four, 899580465, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 174, 188, 187, 49, 130, 158, 53], OperandSize::Dword)
}

fn clflush_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CLFLUSH, operand1: Some(Indirect(RCX, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 174, 57], OperandSize::Qword)
}

